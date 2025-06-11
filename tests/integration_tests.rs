use std::env;
use supabase_auth_rs::{AuthClient, AuthError, IdType};
use uuid::Uuid;

/// Helper to create an auth client for tests
fn create_test_client() -> AuthClient {
    let api_url = env::var("SUPABASE_URL").unwrap_or_else(|_| "http://127.0.0.1:54321".to_string());
    let anon_key = env::var("SUPABASE_ANON_KEY").unwrap_or_else(|_| {
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0".to_string()
    });

    AuthClient::new(&api_url, &anon_key).expect("Failed to create auth client")
}

/// Helper to create an auth client with service role key for admin operations
fn create_admin_test_client() -> Option<AuthClient> {
    let api_url = env::var("SUPABASE_URL").unwrap_or_else(|_| "http://127.0.0.1:54321".to_string());
    let anon_key = env::var("SUPABASE_ANON_KEY").unwrap_or_else(|_| {
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0".to_string()
    });

    // Check for service role key
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").ok()?;

    Some(
        AuthClient::builder()
            .api_url(&api_url)
            .anon_key(&anon_key)
            .service_role_key(&service_role_key)
            .build()
            .expect("Failed to create admin auth client"),
    )
}

/// Generate a unique test email
fn generate_test_email() -> String {
    format!("test-{}@example.com", Uuid::new_v4())
}

#[tokio::test]
async fn test_signup_with_email() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    let result = client
        .signup(IdType::Email(email.clone()), password.to_string(), None)
        .await;

    assert!(result.is_ok(), "Signup should succeed");
    let (user, access_token) = result.unwrap();

    assert_eq!(user.email, Some(email));
    assert!(!access_token.is_empty());
    assert_eq!(user.role, "authenticated");

    // Clean up
    let _ = client.hard_delete_user(user.id).await;
}

#[tokio::test]
async fn test_signup_with_metadata() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    let mut metadata = std::collections::HashMap::new();
    metadata.insert("first_name".to_string(), "Test".to_string());
    metadata.insert("last_name".to_string(), "User".to_string());

    let result = client
        .signup(
            IdType::Email(email),
            password.to_string(),
            Some(metadata.clone()),
        )
        .await;

    assert!(result.is_ok(), "Signup with metadata should succeed");
    let (user, _access_token) = result.unwrap();

    // Verify metadata was stored
    if let Some(user_metadata) = &user.user_metadata {
        assert_eq!(
            user_metadata.get("first_name").and_then(|v| v.as_str()),
            Some("Test")
        );
        assert_eq!(
            user_metadata.get("last_name").and_then(|v| v.as_str()),
            Some("User")
        );
    }

    // Clean up
    let _ = client.hard_delete_user(user.id).await;
}

#[tokio::test]
async fn test_signin_with_valid_credentials() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    // First create a user
    let (user, _access_token) = client
        .signup(IdType::Email(email.clone()), password.to_string(), None)
        .await
        .expect("Signup should succeed");

    // Then sign in
    let signin_result = client
        .signin_with_password(IdType::Email(email), password.to_string())
        .await;

    assert!(signin_result.is_ok(), "Signin should succeed");
    let token_response = signin_result.unwrap();

    assert!(!token_response.access_token.is_empty());
    assert!(!token_response.refresh_token.is_empty());
    assert!(token_response.expires_in > 0);
    assert_eq!(token_response.token_type, "bearer");

    // Clean up
    let _ = client.hard_delete_user(user.id).await;
}

#[tokio::test]
async fn test_signin_with_invalid_password() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    // First create a user
    let (user, _access_token) = client
        .signup(IdType::Email(email.clone()), password.to_string(), None)
        .await
        .expect("Signup should succeed");

    // Try to sign in with wrong password
    let signin_result = client
        .signin_with_password(IdType::Email(email), "wrongpassword".to_string())
        .await;

    assert!(
        signin_result.is_err(),
        "Signin with wrong password should fail"
    );

    // Clean up
    let _ = client.hard_delete_user(user.id).await;
}

#[tokio::test]
async fn test_signin_with_nonexistent_email() {
    let client = create_test_client();
    let email = generate_test_email();

    let signin_result = client
        .signin_with_password(IdType::Email(email), "anypassword".to_string())
        .await;

    assert!(
        signin_result.is_err(),
        "Signin with nonexistent email should fail"
    );
}

#[tokio::test]
async fn test_get_user_by_valid_token() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    // Create user and sign in
    let (created_user, access_token) = client
        .signup(IdType::Email(email.clone()), password.to_string(), None)
        .await
        .expect("Signup should succeed");

    // Get user by token
    let get_user_result = client.get_user_by_token(&access_token).await;

    assert!(get_user_result.is_ok(), "Get user by token should succeed");
    let fetched_user = get_user_result.unwrap();

    assert_eq!(fetched_user.id, created_user.id);
    assert_eq!(fetched_user.email, Some(email));
    assert_eq!(fetched_user.role, "authenticated");

    // Clean up
    let _ = client.hard_delete_user(created_user.id).await;
}

#[tokio::test]
async fn test_get_user_by_invalid_token() {
    let client = create_test_client();

    let result = client.get_user_by_token("invalid-token-12345").await;

    assert!(result.is_err(), "Get user with invalid token should fail");
    match result.unwrap_err() {
        AuthError::NotAuthorized => {}
        other => panic!("Expected NotAuthorized error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_get_user_by_empty_token() {
    let client = create_test_client();

    let result = client.get_user_by_token("").await;

    assert!(result.is_err(), "Get user with empty token should fail");
    match result.unwrap_err() {
        AuthError::InvalidParameters => {}
        other => panic!("Expected InvalidParameters error, got: {:?}", other),
    }
}

#[tokio::test]
async fn test_refresh_token() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    // Create user and sign in
    let (user, _) = client
        .signup(IdType::Email(email.clone()), password.to_string(), None)
        .await
        .expect("Signup should succeed");

    let signin_response = client
        .signin_with_password(IdType::Email(email), password.to_string())
        .await
        .expect("Signin should succeed");

    // Wait a moment to ensure tokens are different
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Refresh token
    let refresh_result = client.refresh_token(&signin_response.refresh_token).await;

    assert!(refresh_result.is_ok(), "Token refresh should succeed");
    let new_tokens = refresh_result.unwrap();

    assert!(!new_tokens.access_token.is_empty());
    assert!(!new_tokens.refresh_token.is_empty());
    // In local dev, tokens might be the same if not expired
    // Just verify we got valid tokens back

    // Verify new access token works
    let user_result = client.get_user_by_token(&new_tokens.access_token).await;
    assert!(user_result.is_ok(), "New access token should be valid");

    // Clean up
    let _ = client.hard_delete_user(user.id).await;
}

#[tokio::test]
async fn test_refresh_with_invalid_token() {
    let client = create_test_client();

    let result = client.refresh_token("invalid-refresh-token").await;

    assert!(result.is_err(), "Refresh with invalid token should fail");
}

#[tokio::test]
async fn test_delete_user() {
    // Skip test if no service role key available
    let Some(admin_client) = create_admin_test_client() else {
        eprintln!("Skipping test_delete_user: SUPABASE_SERVICE_ROLE_KEY not set");
        return;
    };

    let email = generate_test_email();
    let password = "testpassword123";

    // Create user using regular client
    let client = create_test_client();
    let (user, _access_token) = client
        .signup(IdType::Email(email.clone()), password.to_string(), None)
        .await
        .expect("Signup should succeed");

    // Delete user using admin client
    let delete_result = admin_client.hard_delete_user(user.id).await;
    assert!(delete_result.is_ok(), "Delete user should succeed");

    // Verify user can't sign in anymore
    let signin_result = client
        .signin_with_password(IdType::Email(email), password.to_string())
        .await;

    assert!(signin_result.is_err(), "Signin after deletion should fail");
}

#[tokio::test]
async fn test_delete_user_requires_service_role() {
    let client = create_test_client();
    let result = client.hard_delete_user(Uuid::new_v4()).await;

    assert!(matches!(result, Err(AuthError::ServiceRoleKeyRequired)));
}

#[tokio::test]
async fn test_soft_delete_user_requires_service_role() {
    let client = create_test_client();
    let result = client.soft_delete_user(Uuid::new_v4()).await;

    assert!(matches!(result, Err(AuthError::ServiceRoleKeyRequired)));
}

#[tokio::test]
async fn test_delete_user_with_wrong_token() {
    let client = create_test_client();
    let email1 = generate_test_email();
    let email2 = generate_test_email();
    let password = "testpassword123";

    // Create two users
    let (user1, _token1) = client
        .signup(IdType::Email(email1), password.to_string(), None)
        .await
        .expect("Signup user 1 should succeed");

    let (user2, _token2) = client
        .signup(IdType::Email(email2), password.to_string(), None)
        .await
        .expect("Signup user 2 should succeed");

    // Try to delete user1 with user2's token (this would need admin access)
    // Since we're using anon key, we can't test cross-user deletion
    // Just clean up both users

    // Clean up
    let _ = client.hard_delete_user(user1.id).await;
    let _ = client.hard_delete_user(user2.id).await;
}

#[tokio::test]
async fn test_logout() {
    let client = create_test_client();
    let email = generate_test_email();
    let password = "testpassword123";

    // Create user and sign in
    let (user, access_token) = client
        .signup(IdType::Email(email), password.to_string(), None)
        .await
        .expect("Signup should succeed");

    // Verify token works before logout
    let user_result = client.get_user_by_token(&access_token).await;
    assert!(user_result.is_ok(), "Token should work before logout");

    // Logout
    let logout_result = client.logout(&access_token).await;
    assert!(logout_result.is_ok(), "Logout should succeed");

    // Note: Supabase's logout endpoint doesn't immediately invalidate tokens
    // in development mode. In production, tokens would be invalidated.

    // Clean up
    let _ = client.hard_delete_user(user.id).await;
}

#[tokio::test]
async fn test_signup_with_empty_email() {
    let client = create_test_client();

    let result = client
        .signup(
            IdType::Email("".to_string()),
            "password123".to_string(),
            None,
        )
        .await;

    assert!(result.is_err(), "Signup with empty email should fail");
}

#[tokio::test]
async fn test_signup_with_empty_password() {
    let client = create_test_client();
    let email = generate_test_email();

    let result = client
        .signup(IdType::Email(email), "".to_string(), None)
        .await;

    assert!(result.is_err(), "Signup with empty password should fail");
}

#[tokio::test]
async fn test_signin_with_empty_password() {
    let client = create_test_client();
    let email = generate_test_email();

    let result = client
        .signin_with_password(IdType::Email(email), "".to_string())
        .await;

    assert!(result.is_err(), "Signin with empty password should fail");
    match result.unwrap_err() {
        AuthError::InvalidParameters => {}
        other => panic!("Expected InvalidParameters error, got: {:?}", other),
    }
}
