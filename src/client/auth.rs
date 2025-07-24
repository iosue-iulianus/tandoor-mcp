use anyhow::Result;
use reqwest::Client;
use crate::client::types::{AuthRequest, AuthToken};

pub struct TandoorAuth {
    base_url: String,
    client: Client,
    token: Option<String>,
}

impl TandoorAuth {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: Client::new(),
            token: None,
        }
    }

    pub async fn authenticate(&mut self, username: String, password: String) -> Result<()> {
        tracing::info!("Attempting authentication for user: {}", username);
        
        let auth_request = AuthRequest { username: username.clone(), password };
        let auth_url = format!("{}/api-token-auth/", self.base_url);
        
        tracing::debug!("Making authentication request to: {}", auth_url);
        
        let response = self
            .client
            .post(&auth_url)
            .json(&auth_request)
            .timeout(std::time::Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| {
                tracing::error!("Network error during authentication: {}", e);
                anyhow::anyhow!("Failed to connect to Tandoor server at {}: {}", self.base_url, e)
            })?;

        let status = response.status();
        tracing::debug!("Authentication response status: {}", status);
        
        if !status.is_success() {
            let error_body = response.text().await.unwrap_or_else(|_| "Unable to read error response".to_string());
            tracing::error!("Authentication failed with status {}: {}", status, error_body);
            
            match status.as_u16() {
                400 => anyhow::bail!("Invalid credentials provided. Please check username and password."),
                401 => anyhow::bail!("Authentication failed: Invalid username or password"),
                403 => anyhow::bail!("Access denied: User account may be disabled"),
                404 => anyhow::bail!("Tandoor API endpoint not found. Check your base URL: {}", self.base_url),
                500..=599 => anyhow::bail!("Tandoor server error ({}): {}", status, error_body),
                _ => anyhow::bail!("Authentication failed with status {}: {}", status, error_body),
            }
        }

        let auth_token: AuthToken = response.json().await
            .map_err(|e| {
                tracing::error!("Failed to parse authentication response: {}", e);
                anyhow::anyhow!("Invalid response from Tandoor server: {}", e)
            })?;
            
        self.token = Some(auth_token.token.clone());
        tracing::info!("Authentication successful for user: {}", username);
        tracing::debug!("Received token: {}...", &auth_token.token[..auth_token.token.len().min(10)]);
        
        Ok(())
    }

    pub fn get_token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    pub fn is_authenticated(&self) -> bool {
        self.token.is_some()
    }
    
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }
}