#![allow(missing_docs)]

use kinded::Kinded;
use thiserror::Error;

/// Errors that can occur when interacting with the Supabase Auth API
#[derive(Debug, Default, Clone, Copy, Error, Kinded)]
#[non_exhaustive]
pub enum AuthError {
    /// User is not authorized to perform the requested operation
    #[error("not authorized")]
    NotAuthorized,

    /// Invalid parameters provided to the API
    #[error("invalid parameters")]
    InvalidParameters,

    /// HTTP communication error
    #[error("http error")]
    Http,

    /// Internal library error (e.g., JSON parsing)
    #[error("internal library error")]
    Internal,

    /// Requested resource was not found
    #[error("resource not found")]
    NotFound,

    /// Service role key is required for admin operations
    #[error("service role key required for admin operations")]
    ServiceRoleKeyRequired,

    /// General authentication error
    #[error("general gotrue error")]
    #[default]
    GeneralError,
}
