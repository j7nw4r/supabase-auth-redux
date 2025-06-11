use serde::{Deserialize, Serialize};
use tracing::{debug, error, info, instrument, trace_span, Instrument};

use crate::error::AuthError;
use crate::models::token::TokenResponse;
use crate::util::handle_response_code;
use crate::AuthClient;

#[derive(Debug, Serialize, Deserialize)]
struct TokenRefreshGrant {
    pub refresh_token: String,
}

impl AuthClient {
    /// Refreshes an authentication token to obtain new access and refresh tokens
    ///
    /// This method exchanges a valid refresh token for a new set of tokens, extending
    /// the user's session without requiring them to sign in again. The new access token
    /// can be used for API authentication.
    ///
    /// # Arguments
    ///
    /// * `token` - A valid refresh token obtained from signin or a previous refresh
    ///
    /// # Returns
    ///
    /// Returns a `TokenResponse` containing new access and refresh tokens.
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidParameters` if the token is empty.
    /// Returns `AuthError::NotAuthorized` if the refresh token is invalid or expired.
    /// Returns `AuthError::Http` if the API request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_rs::AuthClient;
    /// # async fn example() -> Result<(), supabase_auth_rs::AuthError> {
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
    ///
    /// // After user signs in, you have their refresh token
    /// let refresh_token = "user-refresh-token";
    /// let new_tokens = client.refresh_token(refresh_token).await?;
    ///
    /// println!("New access token: {}", new_tokens.access_token);
    /// println!("New refresh token: {}", new_tokens.refresh_token);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn refresh_token(&self, token: &str) -> Result<TokenResponse, AuthError> {
        if token.is_empty() {
            error!("empty token");
            return Err(AuthError::InvalidParameters);
        }

        let token_grant = TokenRefreshGrant {
            refresh_token: token.to_string(),
        };

        let resp = match self
            .http_client
            .post(format!(
                "{}/auth/v1/{}",
                self.supabase_api_url, "token?grant_type=refresh_token"
            ))
            .bearer_auth(&self.supabase_anon_key)
            .header("apiKey", &self.supabase_anon_key)
            .json(&token_grant)
            .send()
            .instrument(trace_span!("gotrue refresh token"))
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
