use mcp_tandoor::server::TandoorMcpServer;
use rmcp::transport::sse_server::{SseServer, SseServerConfig};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info".to_string().into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Get configuration from environment variables
    let base_url =
        env::var("TANDOOR_BASE_URL").unwrap_or_else(|_| "http://localhost:8080".to_string());

    let username = env::var("TANDOOR_USERNAME").unwrap_or_else(|_| "admin".to_string());

    let password = env::var("TANDOOR_PASSWORD").unwrap_or_else(|_| "admin".to_string());

    let bind_addr = env::var("BIND_ADDR").unwrap_or_else(|_| "127.0.0.1:3001".to_string());

    // Test authentication and validate token
    tracing::info!("Validating Tandoor credentials...");
    let test_server = TandoorMcpServer::new_with_credentials(
        base_url.clone(),
        username.clone(),
        password.clone()
    );
    
    if let Err(e) = test_server
        .authenticate(username.clone(), password.clone())
        .await
    {
        tracing::error!("Authentication failed: {}", e);
        tracing::error!("Please verify:");
        tracing::error!("  - TANDOOR_BASE_URL is correct: {}", base_url);
        tracing::error!("  - TANDOOR_USERNAME is correct: {}", username);
        tracing::error!("  - TANDOOR_PASSWORD is correct");
        tracing::error!("  - Tandoor server is running and accessible");
        tracing::error!("  - Check if you're being rate limited (wait before retrying)");
        std::process::exit(1);
    }
    
    // Test that we can actually use the token to make an API call
    tracing::info!("Testing API access with token...");
    match test_server.test_api_access().await {
        Ok(_) => {
            tracing::info!("API access test passed");
        }
        Err(e) => {
            tracing::warn!("API access test failed: {}", e);
            tracing::warn!("This may be due to Tandoor permission configuration.");
            tracing::warn!("The server will continue, but some functionality may be limited.");
            tracing::warn!("If you encounter issues, check Tandoor space permissions and user roles.");
        }
    }
    
    tracing::info!("Successfully authenticated and validated API access with Tandoor");

    // Create server configuration and start SSE server
    let config = SseServerConfig {
        bind: bind_addr.parse()?,
        sse_path: "/sse".to_string(),
        post_path: "/message".to_string(),
        ct: tokio_util::sync::CancellationToken::new(),
        sse_keep_alive: None,
    };

    tracing::info!("Tandoor MCP Server listening on {}", config.bind);

    // serve_with_config handles binding, axum server setup, and graceful shutdown internally
    let sse_server = SseServer::serve_with_config(config).await?;

    // Add the Tandoor service with authentication credentials
    let base_url_clone = base_url.clone();
    let username_clone = username.clone();
    let password_clone = password.clone();
    
    let ct = sse_server.with_service(move || {
        TandoorMcpServer::new_with_credentials(
            base_url_clone.clone(),
            username_clone.clone(),
            password_clone.clone()
        )
    });

    tracing::info!("Tandoor MCP Server started successfully");

    // Wait for Ctrl+C
    tokio::signal::ctrl_c().await?;
    tracing::info!("Shutting down...");
    ct.cancel();

    Ok(())
}
