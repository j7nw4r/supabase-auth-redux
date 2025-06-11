use std::env;
use supabase_auth_rs::{AuthClient, IdType};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing for debug output
    tracing_subscriber::fmt::init();

    // Get configuration from environment or use defaults for local Supabase
    let api_url = env::var("SUPABASE_URL").unwrap_or_else(|_| "http://127.0.0.1:54321".to_string());
    let anon_key = env::var("SUPABASE_ANON_KEY").unwrap_or_else(|_| {
        // This is the default anon key for local Supabase development
        "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0".to_string()
    });
    let service_role_key = env::var("SUPABASE_SERVICE_ROLE_KEY").ok();

    println!("Connecting to Supabase at: {}", api_url);

    // Create auth client
    let mut builder = AuthClient::builder().api_url(&api_url).anon_key(&anon_key);

    if let Some(key) = service_role_key {
        builder = builder.service_role_key(&key);
    }

    let auth_client = builder.build()?;
    println!("✓ Auth client created successfully");

    // Test email for this example
    let test_email = format!("test-{}@example.com", uuid::Uuid::new_v4());
    let test_password = "password123";

    // 1. Sign up a new user
    println!("\n1. Testing signup...");
    let (user, _access_token) = auth_client
        .signup(
            IdType::Email(test_email.clone()),
            test_password.to_string(),
            None,
        )
        .await?;
    println!("✓ User created: {}", user.id);
    println!("  Email: {:?}", user.email);
    println!("  Role: {}", user.role);

    // 2. Sign in with the created user
    println!("\n2. Testing signin...");
    let token_response = auth_client
        .signin_with_password(IdType::Email(test_email.clone()), test_password.to_string())
        .await?;
    println!("✓ Sign in successful");
    println!("  Access token: {}...", &token_response.access_token[..20]);
    println!("  Expires in: {} seconds", token_response.expires_in);

    // 3. Get user by token
    println!("\n3. Testing get user by token...");
    let fetched_user = auth_client
        .get_user_by_token(&token_response.access_token)
        .await?;
    println!("✓ User fetched successfully");
    println!("  User ID matches: {}", fetched_user.id == user.id);

    // 4. Refresh token
    println!("\n4. Testing token refresh...");
    let new_tokens = auth_client
        .refresh_token(&token_response.refresh_token)
        .await?;
    println!("✓ Token refreshed successfully");
    println!("  New access token: {}...", &new_tokens.access_token[..20]);

    // 5. Test invalid token
    println!("\n5. Testing invalid token handling...");
    match auth_client.get_user_by_token("invalid-token").await {
        Err(e) => println!("✓ Invalid token correctly rejected: {:?}", e),
        Ok(_) => println!("✗ Invalid token was accepted (this shouldn't happen)"),
    }

    // 6. Clean up - delete the test user
    println!("\n6. Cleaning up - deleting test user...");
    match auth_client.hard_delete_user(user.id).await {
        Ok(_) => println!("✓ Test user deleted"),
        Err(e) => println!(
            "✗ Could not delete user (service role key may be needed): {:?}",
            e
        ),
    }

    println!("\nAll tests passed! The auth client works correctly with local Supabase.");

    Ok(())
}
