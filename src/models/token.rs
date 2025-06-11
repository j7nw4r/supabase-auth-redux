use serde::{Deserialize, Serialize};

use crate::models::user::UserSchema;

/// Response containing authentication tokens and user information
///
/// This is returned after successful authentication operations like signin or signup.
#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct TokenResponse {
    /// JWT access token for API authentication
    pub access_token: String,
    /// Token type (typically "bearer")
    pub token_type: String,
    /// Token validity duration in seconds
    pub expires_in: u64,
    /// Unix timestamp when the token expires
    pub expires_at: u64,
    /// Refresh token for obtaining new access tokens
    pub refresh_token: String,
    /// User information associated with the token
    pub user: Option<UserSchema>,
    /// OAuth provider token (if using third-party auth)
    pub provider_token: String,
    /// OAuth provider refresh token (if using third-party auth)
    pub provider_refresh_token: String,
    /// Weak password warning information
    pub weak_password: Option<WeakPasswordError>,
}

/// Error information returned when a password is considered weak
#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[serde(default)]
pub struct WeakPasswordError {
    /// Description of why the password is weak
    pub message: String,
    /// List of specific reasons the password was rejected
    pub reasons: Vec<String>,
}
