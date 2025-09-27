//! # Tandoor MCP Library
//!
//! This library provides tools for integrating with Tandoor (a recipe management system)
//! through the Model Context Protocol (MCP). It consists of two main components:
//!
//! ## Client Module
//!
//! The [`client`] module provides a direct HTTP client for the Tandoor API, handling
//! authentication, recipe management, shopping lists, and more.
//!
//! ## Server Module
//!
//! The [`server`] module implements an MCP server that exposes Tandoor functionality
//! as standardized tools that AI assistants can use.
//!
//! ## Quick Start
//!
//! ```no_run
//! use mcp_tandoor::{TandoorClient, TandoorMcpServer};
//!
//! // Use the client directly
//! let mut client = TandoorClient::new("http://localhost:8080".to_string());
//!
//! // Or create an MCP server
//! let server = TandoorMcpServer::new_with_credentials(
//!     "http://localhost:8080".to_string(),
//!     "username".to_string(),
//!     "password".to_string()
//! );
//! ```

pub mod client;
pub mod server;

pub use client::TandoorClient;
pub use server::TandoorMcpServer;
