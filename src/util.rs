use crate::AuthError;
use reqwest::StatusCode;
use tracing::{debug, info, instrument};

#[instrument]
pub(super) async fn handle_response_code(resp_status: StatusCode) -> Result<(), AuthError> {
    info!(response.status = resp_status.as_u16());
    if !resp_status.is_success() {
        debug!("non-success response status code from supabase auth");
        return match resp_status {
            StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Err(AuthError::NotAuthorized),
            StatusCode::UNPROCESSABLE_ENTITY | StatusCode::BAD_REQUEST => {
                Err(AuthError::InvalidParameters)
            }
            StatusCode::NOT_ACCEPTABLE => Err(AuthError::NotFound),
            StatusCode::INTERNAL_SERVER_ERROR => Err(AuthError::GeneralError),
            _ => Err(AuthError::GeneralError),
        };
    }
    Ok(())
}
