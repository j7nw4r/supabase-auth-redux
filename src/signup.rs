use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tracing::{debug, info, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::user::UserSchema;
use crate::util::handle_response_code;
use crate::{AuthClient, IdType};

#[derive(Debug, Serialize, Deserialize)]
struct SignupRequest {
    pub email: Option<String>,
    pub phone_number: Option<String>,
    pub password: String,
    pub data: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct SignupResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub expires_at: i64,
    pub refresh_token: String,
    pub user: UserSchema,
}

impl AuthClient {
    /// Creates a new user account
    ///
    /// This method registers a new user with the provided credentials and optional metadata.
    /// Upon successful registration, the user is automatically signed in and authentication
    /// tokens are returned.
    ///
    /// # Arguments
    ///
    /// * `signup_id_type` - The user's identifier (email or phone number)
    /// * `password` - The desired password for the account
    /// * `metadata` - Optional user metadata to store with the account
    ///
    /// # Returns
    ///
    /// Returns a tuple containing:
    /// - The newly created `UserSchema` with user information
    /// - An access token string for immediate authentication
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidParameters` if required fields are missing.
    /// Returns `AuthError::Http` if the API request fails or user already exists.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_redux::{AuthClient, IdType};
    /// # use std::collections::HashMap;
    /// # async fn example() -> Result<(), supabase_auth_redux::AuthError> {
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
    ///
    /// let mut metadata = HashMap::new();
    /// metadata.insert("first_name".to_string(), "John".to_string());
    /// metadata.insert("last_name".to_string(), "Doe".to_string());
    ///
    /// let (user, access_token) = client
    ///     .signup(
    ///         IdType::Email("newuser@example.com".to_string()),
    ///         "secure_password".to_string(),
    ///         Some(metadata),
    ///     )
    ///     .await?;
    ///
    /// println!("User created with ID: {}", user.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn signup(
        &self,
        signup_id_type: IdType,
        password: String,
        _metadata: Option<HashMap<String, String>>,
    ) -> Result<(UserSchema, String), AuthError> {
        let body = match signup_id_type {
            IdType::Email(email) => SignupRequest {
                email: Some(email),
                phone_number: None,
                password,
                data: _metadata,
            },
            IdType::PhoneNumber(phone_number) => SignupRequest {
                email: None,
                phone_number: Some(phone_number),
                password,
                data: _metadata,
            },
        };

        let resp = match self
            .http_client
            .post(format!("{}/auth/v1/{}", self.supabase_api_url, "signup"))
            .header("apiKey", &self.supabase_anon_key)
            .bearer_auth(&self.supabase_anon_key)
            .json(&body)
            .send()
            .instrument(trace_span!("gotrue create user"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                debug!("{}", e);
                return Err(AuthError::Http);
            }
        };

        let resp_code_result = handle_response_code(resp.status()).await;
        let resp_text = match resp.text().await {
            Ok(resp_text) => resp_text,
            Err(e) => {
                debug!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result?;

        let created_user_resp = match serde_json::from_str::<SignupResponse>(&resp_text) {
            Ok(token_response) => token_response,
            Err(e) => {
                debug!("{}", e);
                return Err(AuthError::Internal);
            }
        };

        let created_user = created_user_resp.user;
        info!(user_id = created_user.id.to_string(), "created user");

        Ok((created_user, created_user_resp.access_token))
    }
}
