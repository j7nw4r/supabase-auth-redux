//! # Supabase Auth Rust Client
//!
//! A Rust client library for interacting with the Supabase Auth API.
//!
//! This library provides a simple and type-safe way to integrate Supabase authentication
//! into your Rust applications, supporting common authentication flows like signup,
//! signin, token refresh, and user management.
//!
//! ## Example
//!
//! ```rust,no_run
//! use supabase_auth_redux::{AuthClient, AuthError, IdType};
//!
//! # async fn example() -> Result<(), AuthError> {
//! // Initialize the client
//! let auth_client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
//!
//! // Sign up a new user
//! let (user, access_token) = auth_client
//!     .signup(
//!         IdType::Email("user@example.com".to_string()),
//!         "secure_password".to_string(),
//!         None,
//!     )
//!     .await?;
//!
//! // Sign in an existing user
//! let token_response = auth_client
//!     .signin_with_password(
//!         IdType::Email("user@example.com".to_string()),
//!         "secure_password".to_string(),
//!     )
//!     .await?;
//! # Ok(())
//! # }
//! ```

#![warn(clippy::all)]
#![warn(missing_docs)]

use std::fmt::{Debug, Display, Formatter};

use postgrest::Postgrest;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub use error::AuthError;
pub use models::token::TokenResponse;
pub use models::user::UserSchema as User;

// Re-export for backward compatibility
#[allow(unused)]
#[deprecated(
    since = "0.1.0",
    note = "Use specific error types from AuthError instead"
)]
pub use GoTrueErrorResponse as Error;

mod delete_user;
mod error;
mod get_user;
mod logout;
pub mod models;
mod refresh_token;
mod signin_with_password;
mod signup;
mod util;

/// The main authentication client for interacting with Supabase Auth API
///
/// This client handles all authentication operations including user signup,
/// signin, token management, and user administration.
#[derive(Clone)]
pub struct AuthClient {
    /// HTTP client for making API requests
    http_client: reqwest::Client,
    /// Base URL of the Supabase API (e.g., `https://your-project.supabase.co`)
    supabase_api_url: String,
    /// Anonymous key for public API access
    supabase_anon_key: String,
    /// Optional service role key for admin operations
    supabase_service_role_key: Option<String>,
    /// PostgREST client for direct database queries
    postgrest_client: Postgrest,
}

impl Debug for AuthClient {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("AuthClient")
    }
}

impl AuthClient {
    /// Creates a new authentication client with the given API URL and anonymous key
    ///
    /// # Arguments
    ///
    /// * `api_url` - The base URL of your Supabase instance
    /// * `anon_key` - The anonymous key for your Supabase project
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use supabase_auth_redux::AuthClient;
    ///
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")
    ///     .expect("Failed to create auth client");
    /// ```
    pub fn new(api_url: &str, anon_key: &str) -> Result<Self, AuthError> {
        if api_url.is_empty() {
            return Err(AuthError::InvalidParameters);
        }
        if anon_key.is_empty() {
            return Err(AuthError::InvalidParameters);
        }

        Ok(Self {
            http_client: reqwest::Client::new(),
            supabase_api_url: api_url.to_owned(),
            supabase_anon_key: anon_key.to_owned(),
            supabase_service_role_key: None,
            postgrest_client: Postgrest::new(format!("{}/rest/v1/", api_url.to_owned()))
                .schema("auth")
                .insert_header("apikey", anon_key),
        })
    }

    /// Creates a new builder for constructing an AuthClient with additional options
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use supabase_auth_redux::AuthClient;
    ///
    /// let client = AuthClient::builder()
    ///     .api_url("https://your-project.supabase.co")
    ///     .anon_key("your-anon-key")
    ///     .service_role_key("your-service-role-key")
    ///     .build()
    ///     .expect("Failed to create auth client");
    /// ```
    pub fn builder() -> AuthClientBuilder {
        AuthClientBuilder::default()
    }
}

/// Builder for constructing an AuthClient with custom configuration
#[derive(Default)]
pub struct AuthClientBuilder {
    /// API URL for the Supabase instance
    api_url: Option<String>,
    /// Anonymous key for the Supabase project
    anon_key: Option<String>,
    /// Optional service role key for admin operations
    service_role_key: Option<String>,
}

impl AuthClientBuilder {
    /// Sets the API URL for the Supabase instance
    pub fn api_url(mut self, url: &str) -> Self {
        self.api_url = Some(url.to_string());
        self
    }

    /// Sets the anonymous key for the Supabase project
    pub fn anon_key(mut self, key: &str) -> Self {
        self.anon_key = Some(key.to_string());
        self
    }

    /// Sets the service role key for admin operations
    pub fn service_role_key(mut self, key: &str) -> Self {
        self.service_role_key = Some(key.to_string());
        self
    }

    /// Builds the AuthClient with the configured settings
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidParameters` if required fields are missing
    pub fn build(self) -> Result<AuthClient, AuthError> {
        let api_url = self.api_url.ok_or(AuthError::InvalidParameters)?;
        let anon_key = self.anon_key.ok_or(AuthError::InvalidParameters)?;

        Ok(AuthClient {
            http_client: reqwest::Client::new(),
            supabase_api_url: api_url.clone(),
            supabase_anon_key: anon_key.clone(),
            supabase_service_role_key: self.service_role_key,
            postgrest_client: Postgrest::new(format!("{}/rest/v1/", api_url))
                .schema("auth")
                .insert_header("apikey", &anon_key),
        })
    }
}

/// Error response from the GoTrue/Supabase Auth API
#[derive(Debug, Error, Deserialize, Serialize)]
pub struct GoTrueErrorResponse {
    /// Error code number from the API
    pub code: Option<u8>,
    /// Primary error message
    pub error: Option<String>,
    /// Detailed error description
    pub error_description: Option<String>,
    /// Alternative error message field used by some endpoints
    pub msg: Option<String>,
}

impl Display for GoTrueErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(ref e) = self.error {
            f.write_str(e)?;
            return Ok(());
        }
        if let Some(ref msg) = self.msg {
            f.write_str(msg)?;
            return Ok(());
        }
        Err(std::fmt::Error)
    }
}

/// Identifier type for authentication operations
#[derive(Debug)]
pub enum IdType {
    /// Email-based authentication
    Email(String),
    /// Phone number-based authentication
    PhoneNumber(String),
}
