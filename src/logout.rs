use tracing::{debug, error, instrument, trace_span, Instrument};

use crate::util::handle_response_code;
use crate::{AuthClient, AuthError};

impl AuthClient {
    /// Logs out a user by invalidating their authentication token
    ///
    /// This method revokes the provided access token, effectively logging the user out.
    /// After calling this method, the token will no longer be valid for authentication.
    ///
    /// # Arguments
    ///
    /// * `token` - The access token to invalidate
    ///
    /// # Errors
    ///
    /// Returns `AuthError::Http` if the API request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_redux::AuthClient;
    /// # async fn example() -> Result<(), supabase_auth_redux::AuthError> {
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
    ///
    /// // After user signs in and you have their access token
    /// let access_token = "user-access-token";
    /// client.logout(access_token).await?;
    ///
    /// // The token is now invalid and cannot be used for authentication
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip_all)]
    pub async fn logout(&self, token: &str) -> Result<(), AuthError> {
        let resp = match self
            .http_client
            .post(format!("{}/auth/v1/{}", self.supabase_api_url, "logout"))
            .bearer_auth(token)
            .header("apiKey", &self.supabase_anon_key)
            .send()
            .instrument(trace_span!("gotrue logout user"))
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

        Ok(())
    }
}
