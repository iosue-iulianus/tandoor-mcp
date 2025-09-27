//! # Tandoor HTTP Client
//!
//! This module provides a direct HTTP client for the Tandoor API, handling authentication,
//! recipe management, shopping lists, and other Tandoor functionality.
//!
//! ## Modules
//!
//! - [`auth`] - Authentication handling for Tandoor OAuth2 tokens
//! - [`client`] - Main HTTP client implementation with all API methods
//! - [`types`] - Type definitions for API requests and responses
//!
//! ## Quick Start
//!
//! ```no_run
//! use mcp_tandoor::client::TandoorClient;
//!
//! # async fn example() -> anyhow::Result<()> {
//! let mut client = TandoorClient::new("http://localhost:8080".to_string());
//!
//! // Authenticate with Tandoor
//! client.authenticate("username".to_string(), "password".to_string()).await?;
//!
//! // Search for recipes
//! let recipes = client.search_recipes(Some("pasta"), Some(10)).await?;
//! println!("Found {} recipes", recipes.count);
//! # Ok(())
//! # }
//! ```

pub mod auth;
#[allow(clippy::module_inception)]
pub mod client;
pub mod types;

pub use client::TandoorClient;
pub use types::*;
