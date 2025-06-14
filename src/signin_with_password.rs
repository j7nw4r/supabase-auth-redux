use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::token::TokenResponse;
use crate::util::handle_response_code;
use crate::AuthClient;
use crate::IdType;

#[derive(Debug, Deserialize, Serialize)]
struct TokenPasswordGrant {
    email: Option<String>,
    phone: Option<String>,
    password: String,
}

impl AuthClient {
    /// Signs in a user with their email/phone and password
    ///
    /// This method authenticates a user using their credentials and returns authentication tokens
    /// that can be used for subsequent API requests.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's identifier (email or phone number)
    /// * `password` - The user's password
    ///
    /// # Returns
    ///
    /// Returns a `TokenResponse` containing access and refresh tokens on successful authentication.
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidParameters` if email/phone or password is empty.
    /// Returns `AuthError::NotAuthorized` if credentials are invalid.
    /// Returns `AuthError::Http` if the API request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_redux::{AuthClient, IdType};
    /// # async fn example() -> Result<(), supabase_auth_redux::AuthError> {
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
    ///
    /// let tokens = client
    ///     .signin_with_password(
    ///         IdType::Email("user@example.com".to_string()),
    ///         "secure_password".to_string(),
    ///     )
    ///     .await?;
    ///
    /// println!("Access token: {}", tokens.access_token);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip_all)]
    pub async fn signin_with_password(
        &self,
        id: IdType,
        password: String,
    ) -> Result<TokenResponse, AuthError> {
        if password.is_empty() {
            error!("empty password");
            return Err(AuthError::InvalidParameters);
        }

        let token_password_grant = match id {
            IdType::Email(email) => {
                if email.is_empty() {
                    error!("empty email");
                    return Err(AuthError::InvalidParameters);
                }

                info!(email = email);
                TokenPasswordGrant {
                    email: Some(email),
                    phone: None,
                    password,
                }
            }
            IdType::PhoneNumber(phone_number) => {
                if phone_number.is_empty() {
                    error!("empty phone_number");
                    return Err(AuthError::InvalidParameters);
                }

                info!(phone_number = phone_number);
                TokenPasswordGrant {
                    email: None,
                    phone: Some(phone_number),
                    password,
                }
            }
        };

        let resp = match self
            .http_client
            .post(format!(
                "{}/auth/v1/{}",
                self.supabase_api_url, "token?grant_type=password"
            ))
            .bearer_auth(&self.supabase_anon_key)
            .header("apiKey", &self.supabase_anon_key)
            .json(&token_password_grant)
            .send()
            .instrument(trace_span!("gotrue token password"))
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        let resp_code_result = handle_response_code(resp.status()).await;
        let resp_text = match resp.text().await {
            Ok(resp_text) => resp_text,
            Err(e) => {
                log::error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result?;

        let token_response = match serde_json::from_str::<TokenResponse>(&resp_text) {
            Ok(token_response) => token_response,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Internal);
            }
        };
        info!(
            tokens_are_nonempty =
                !token_response.access_token.is_empty() && !token_response.refresh_token.is_empty()
        );
        debug!(
            token = token_response.access_token,
            refresh_token = token_response.refresh_token
        );

        Ok(token_response)
    }
}
