use log::error;
use reqwest::StatusCode;
use std::ops::Add;
use tracing::{debug, instrument, trace_span, Instrument};
use uuid::Uuid;

use crate::error::{AuthError, AuthErrorKind};
use crate::models::user::UserSchema;
use crate::util::handle_response_code;
use crate::AuthClient;

impl AuthClient {
    /// Retrieves user information using an authentication token
    ///
    /// This method validates the provided access token and returns the associated user's information.
    /// It's commonly used to verify that a token is valid and to get the current user's details.
    ///
    /// # Arguments
    ///
    /// * `auth_token` - A valid JWT access token
    ///
    /// # Errors
    ///
    /// Returns `AuthError::InvalidParameters` if the token is empty.
    /// Returns `AuthError::NotAuthorized` if the token is invalid or expired.
    /// Returns `AuthError::Http` if the API request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_rs::AuthClient;
    /// # async fn example() -> Result<(), supabase_auth_rs::AuthError> {
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
    ///
    /// // After user signs in and you have their access token
    /// let user = client.get_user_by_token("user-access-token").await?;
    /// println!("User email: {:?}", user.email);
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_user_by_token(&self, auth_token: &str) -> Result<UserSchema, AuthError> {
        if auth_token.is_empty() {
            error!("empty token");
            return Err(AuthError::InvalidParameters);
        }

        let resp = match self
            .http_client
            .get(format!("{}/auth/v1/{}", self.supabase_api_url, "user"))
            .bearer_auth(auth_token)
            .header("apiKey", &self.supabase_anon_key)
            .send()
            .instrument(trace_span!("gotrue get user"))
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
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result?;

        let user = match serde_json::from_str::<UserSchema>(&resp_text) {
            Ok(user) => user,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };

        Ok(user)
    }

    /// Retrieves user information by user ID
    ///
    /// This method fetches a user's information directly from the database using their UUID.
    /// Note: This requires appropriate permissions and may need a service role key depending
    /// on your Row Level Security policies.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user to retrieve
    ///
    /// # Returns
    ///
    /// Returns `Ok(Some(user))` if the user exists, `Ok(None)` if not found.
    ///
    /// # Errors
    ///
    /// Returns `AuthError::Http` if the database query fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_rs::AuthClient;
    /// # use uuid::Uuid;
    /// # async fn example() -> Result<(), supabase_auth_rs::AuthError> {
    /// let client = AuthClient::new("https://your-project.supabase.co", "your-anon-key")?;
    ///
    /// let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    /// if let Some(user) = client.get_user_by_id(user_id).await? {
    ///     println!("Found user: {:?}", user.email);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip(self))]
    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<Option<UserSchema>, AuthError> {
        let query_result = self
            .postgrest_client
            .from("users")
            .auth(&self.supabase_anon_key)
            .eq("id", user_id.to_string())
            .select("*")
            .execute()
            .await;
        let query_response = match query_result {
            Ok(query_response) => query_response,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        if query_response.status().as_u16() == StatusCode::NOT_FOUND.as_u16() {
            return Ok(None);
        }

        let reqwuest_http_status_result = StatusCode::from_u16(query_response.status().as_u16());
        let Ok(eqwuest_http_status) = reqwuest_http_status_result else {
            log::error!(
                "could not covert http status: {:?}",
                reqwuest_http_status_result.unwrap_err()
            );
            return Err(AuthError::Http);
        };
        let handle_response_code_result = handle_response_code(eqwuest_http_status).await;
        let body_text = match query_response.text().await {
            Ok(resp_text) => resp_text,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!(body = body_text);
        if let Err(e) = handle_response_code_result {
            if e.kind() == AuthErrorKind::NotFound {
                return Ok(None);
            }
            handle_response_code_result?
        }

        let users = match serde_json::from_str::<Vec<UserSchema>>(&body_text) {
            Ok(users) => users,
            Err(e) => {
                error!("{}", e);
                return Err(AuthError::Http);
            }
        };

        if users.iter().len() > 1 {
            let user_ids_stringify = users
                .iter()
                .map(|user| user.id)
                .fold(String::new(), |mut acc, user_id| {
                    if acc.is_empty() {
                        let s = format!("[ {}", user_id);
                        acc = acc.add(&s);
                    } else {
                        let s = format!(", {}", user_id);
                        acc = acc.add(&s);
                    }
                    acc
                })
                .add(" ]");
            debug!(
                user_ids = user_ids_stringify,
                "multiple users returned for single user_id"
            );
            return Err(AuthError::Internal);
        }

        Ok(users.first().cloned())
    }
}
