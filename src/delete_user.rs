use crate::util::handle_response_code;
use crate::AuthClient;
use crate::AuthError;
use serde::{Deserialize, Serialize};
use tracing::{debug, instrument};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
struct DeleteBody {
    should_soft_delete: bool,
}

impl AuthClient {
    /// Soft deletes a user, marking them as deleted but preserving their data
    ///
    /// This operation requires a service role key to be configured on the AuthClient.
    /// The user will be marked as deleted but their data will be retained in the database.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user to soft delete
    ///
    /// # Errors
    ///
    /// Returns `AuthError::ServiceRoleKeyRequired` if no service role key is configured.
    /// Returns `AuthError::Http` if the API request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_rs::AuthClient;
    /// # use uuid::Uuid;
    /// # async fn example() -> Result<(), supabase_auth_rs::AuthError> {
    /// let admin_client = AuthClient::builder()
    ///     .api_url("https://your-project.supabase.co")
    ///     .anon_key("your-anon-key")
    ///     .service_role_key("your-service-role-key")
    ///     .build()?;
    ///
    /// let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    /// admin_client.soft_delete_user(user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip_all)]
    pub async fn soft_delete_user(&self, user_id: Uuid) -> Result<(), AuthError> {
        let service_role_key = self
            .supabase_service_role_key
            .as_ref()
            .ok_or(AuthError::ServiceRoleKeyRequired)?;

        let resp = match self
            .http_client
            .delete(format!(
                "{}/auth/v1/admin/users/{}",
                self.supabase_api_url, user_id
            ))
            .json(&DeleteBody {
                should_soft_delete: true,
            })
            .bearer_auth(service_role_key)
            .header("apiKey", service_role_key)
            .send()
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
                log::error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result
    }

    /// Permanently deletes a user and all their associated data
    ///
    /// This operation requires a service role key to be configured on the AuthClient.
    /// The user and all their data will be permanently removed from the database.
    /// This action cannot be undone.
    ///
    /// # Arguments
    ///
    /// * `user_id` - The UUID of the user to permanently delete
    ///
    /// # Errors
    ///
    /// Returns `AuthError::ServiceRoleKeyRequired` if no service role key is configured.
    /// Returns `AuthError::Http` if the API request fails.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// # use supabase_auth_rs::AuthClient;
    /// # use uuid::Uuid;
    /// # async fn example() -> Result<(), supabase_auth_rs::AuthError> {
    /// let admin_client = AuthClient::builder()
    ///     .api_url("https://your-project.supabase.co")
    ///     .anon_key("your-anon-key")
    ///     .service_role_key("your-service-role-key")
    ///     .build()?;
    ///
    /// let user_id = Uuid::parse_str("123e4567-e89b-12d3-a456-426614174000").unwrap();
    /// admin_client.hard_delete_user(user_id).await?;
    /// # Ok(())
    /// # }
    /// ```
    #[instrument(skip_all)]
    pub async fn hard_delete_user(&self, user_id: Uuid) -> Result<(), AuthError> {
        let service_role_key = self
            .supabase_service_role_key
            .as_ref()
            .ok_or(AuthError::ServiceRoleKeyRequired)?;

        let resp = match self
            .http_client
            .delete(format!(
                "{}/auth/v1/admin/users/{}",
                self.supabase_api_url, user_id
            ))
            .json(&DeleteBody {
                should_soft_delete: false,
            })
            .bearer_auth(service_role_key)
            .header("apiKey", service_role_key)
            .send()
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
                log::error!("{}", e);
                return Err(AuthError::Http);
            }
        };
        debug!("resp_text: {}", resp_text);
        resp_code_result
    }
}
