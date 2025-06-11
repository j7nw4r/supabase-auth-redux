mod test_helper;

use test_helper::{TestConfig, TestUser};

#[tokio::test]
async fn test_complete_auth_flow() {
    require_supabase!();
    test_helper::init_test_env();

    let config = TestConfig::from_env();
    let client = config.create_client();

    // Create a test user (will auto-cleanup)
    let test_user = TestUser::create(client.clone())
        .await
        .expect("Failed to create test user");

    println!("Created test user: {}", test_user.email);

    // Test signin flow
    let signin_tokens = test_user.signin().await.expect("Signin should succeed");

    assert!(!signin_tokens.access_token.is_empty());
    assert!(!signin_tokens.refresh_token.is_empty());

    // Test token validation
    let user = client
        .get_user_by_token(&signin_tokens.access_token)
        .await
        .expect("Token validation should succeed");

    assert_eq!(user.id, test_user.id);
    assert_eq!(user.email, Some(test_user.email.clone()));

    // Test token refresh
    let new_tokens = client
        .refresh_token(&signin_tokens.refresh_token)
        .await
        .expect("Token refresh should succeed");

    assert_ne!(new_tokens.access_token, signin_tokens.access_token);

    // Verify new token works
    let user_with_new_token = client
        .get_user_by_token(&new_tokens.access_token)
        .await
        .expect("New token should be valid");

    assert_eq!(user_with_new_token.id, test_user.id);

    println!("✓ Complete auth flow test passed");
}

#[tokio::test]
async fn test_concurrent_signins() {
    require_supabase!();
    test_helper::init_test_env();

    let config = TestConfig::from_env();
    let client = config.create_client();

    // Create a test user
    let test_user = TestUser::create(client.clone())
        .await
        .expect("Failed to create test user");

    // Perform multiple concurrent sign-ins
    let mut handles = vec![];
    for i in 0..5 {
        let email = test_user.email.clone();
        let password = test_user.password.clone();
        let client = client.clone();

        let handle = tokio::spawn(async move {
            println!("Concurrent signin attempt {}", i);
            client
                .signin_with_password(supabase_auth_rs::IdType::Email(email), password)
                .await
        });

        handles.push(handle);
    }

    // Wait for all sign-ins to complete
    let mut success_count = 0;
    for handle in handles {
        match handle.await {
            Ok(Ok(_)) => success_count += 1,
            Ok(Err(e)) => eprintln!("Signin failed: {:?}", e),
            Err(e) => eprintln!("Task failed: {:?}", e),
        }
    }

    assert_eq!(success_count, 5, "All concurrent signins should succeed");
    println!("✓ Concurrent signins test passed");
}

#[tokio::test]
async fn test_token_expiry_handling() {
    require_supabase!();
    test_helper::init_test_env();

    let config = TestConfig::from_env();
    let client = config.create_client();

    // Create user and get tokens
    let test_user = TestUser::create(client.clone())
        .await
        .expect("Failed to create test user");

    let tokens = test_user.signin().await.expect("Signin should succeed");

    // Verify token has expiry information
    assert!(tokens.expires_in > 0);
    assert!(tokens.expires_at > 0);

    // In a real test, we'd wait for token expiry, but that takes too long
    // Instead, we'll just verify the refresh mechanism works
    let refreshed = client
        .refresh_token(&tokens.refresh_token)
        .await
        .expect("Refresh should succeed");

    // New token should have new expiry
    assert!(refreshed.expires_at > tokens.expires_at);

    println!("✓ Token expiry handling test passed");
}
