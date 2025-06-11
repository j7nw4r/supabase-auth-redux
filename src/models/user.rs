use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a user in the Supabase Auth system
///
/// This struct contains all the information about a user including their
/// authentication status, contact information, and metadata.
#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
#[serde(default)]
pub struct UserSchema {
    /// Unique identifier for the user
    pub id: Uuid,

    /// Audience claim for JWT (typically the API URL)
    pub aud: String,
    /// User's role in the system (e.g., "authenticated")
    pub role: String,
    /// User's primary contact email. In most cases you can uniquely identify a user by their email address, but not in all cases.
    pub email: Option<String>,
    /// Timestamp when the email was confirmed
    #[serde(with = "time::serde::rfc3339::option")]
    pub email_confirmed_at: Option<time::OffsetDateTime>,
    /// Timestamp when the user was invited
    #[serde(with = "time::serde::rfc3339::option")]
    pub invited_at: Option<time::OffsetDateTime>,
    /// User's primary contact phone number. In most cases you can uniquely identify a user by their phone number, but not in all cases.
    pub phone: Option<String>,
    /// Timestamp when the phone number was confirmed
    #[serde(with = "time::serde::rfc3339::option")]
    pub phone_confirmed_at: Option<time::OffsetDateTime>,
    /// Timestamp when confirmation email/SMS was sent
    #[serde(with = "time::serde::rfc3339::option")]
    pub confirmation_sent_at: Option<time::OffsetDateTime>,
    /// Timestamp when the user confirmed their account
    #[serde(with = "time::serde::rfc3339::option")]
    pub confirmed_at: Option<time::OffsetDateTime>,
    /// Timestamp when password recovery email was sent
    #[serde(with = "time::serde::rfc3339::option")]
    pub recovery_sent_at: Option<time::OffsetDateTime>,
    /// Pending new email address (awaiting confirmation)
    pub new_email: Option<String>,
    /// Timestamp when email change confirmation was sent
    #[serde(with = "time::serde::rfc3339::option")]
    pub email_change_sent_at: Option<time::OffsetDateTime>,
    /// Pending new phone number (awaiting confirmation)
    pub new_phone: Option<String>,
    /// Timestamp when phone change confirmation was sent
    #[serde(with = "time::serde::rfc3339::option")]
    pub phone_change_sent_at: Option<time::OffsetDateTime>,
    /// Timestamp when reauthentication request was sent
    #[serde(with = "time::serde::rfc3339::option")]
    pub reauthentication_sent_at: Option<time::OffsetDateTime>,
    /// Timestamp of the user's last sign in
    #[serde(with = "time::serde::rfc3339::option")]
    pub last_sign_in_at: Option<time::OffsetDateTime>,
    /// Custom user metadata that can be updated by the user
    pub user_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Custom app metadata that can only be updated by service role
    pub app_metadata: Option<HashMap<String, serde_json::Value>>,
    /// Multi-factor authentication factors
    pub factors: Vec<MFAFactorSchema>,
    /// OAuth/social login identities linked to this user
    pub identities: Option<Vec<HashMap<String, serde_json::Value>>>,
    /// Timestamp until which the user is banned
    #[serde(with = "time::serde::rfc3339::option")]
    pub banned_until: Option<time::OffsetDateTime>,
    /// Timestamp when the user was created
    #[serde(with = "time::serde::rfc3339::option")]
    pub created_at: Option<time::OffsetDateTime>,
    /// Timestamp when the user was soft deleted
    #[serde(with = "time::serde::rfc3339::option")]
    pub deleted_at: Option<time::OffsetDateTime>,
    /// Timestamp when the user was last updated
    #[serde(with = "time::serde::rfc3339::option")]
    pub updated_at: Option<time::OffsetDateTime>,
}

/// Multi-factor authentication factor information
#[derive(Debug, Clone, Deserialize, Serialize, Eq, PartialEq)]
pub struct MFAFactorSchema {
    /// Type of MFA factor (e.g., "totp")
    factor_type: Option<String>,
    /// User-friendly name for the factor
    friendly_name: Option<String>,
    /// Unique identifier for the factor
    id: Option<Uuid>,
    /// Verification status of the factor
    status: Option<MFAFactorStatus>,
}

/// Status of a multi-factor authentication factor
#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
pub enum MFAFactorStatus {
    /// Factor has been verified and is active
    Verified,
    /// Factor is not yet verified
    #[default]
    Unverified,
}
