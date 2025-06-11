#![allow(dead_code)]

use std::sync::Once;
use supabase_auth_redux::{AuthClient, IdType};

static INIT: Once = Once::new();

/// Initialize test environment (logging, etc.)
pub fn init_test_env() {
    INIT.call_once(|| {
        // Initialize tracing for tests if RUST_LOG is set
        if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::fmt::init();
        }
    });
}

/// Configuration for tests
pub struct TestConfig {
    pub api_url: String,
    pub anon_key: String,
}

impl TestConfig {
    pub fn from_env() -> Self {
        Self {
            api_url: std::env::var("SUPABASE_URL")
                .unwrap_or_else(|_| "http://127.0.0.1:54321".to_string()),
            anon_key: std::env::var("SUPABASE_ANON_KEY")
                .unwrap_or_else(|_| {
                    // Default anon key for local Supabase
                    "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZS1kZW1vIiwicm9sZSI6ImFub24iLCJleHAiOjE5ODM4MTI5OTZ9.CRXP1A7WOeoJeXxjNni43kdQwgnWNReilDMblYTn_I0".to_string()
                })
        }
    }

    pub fn create_client(&self) -> AuthClient {
        AuthClient::new(&self.api_url, &self.anon_key).expect("Failed to create auth client")
    }

    pub fn create_admin_client(&self) -> Option<AuthClient> {
        let service_role_key = std::env::var("SUPABASE_SERVICE_ROLE_KEY").ok()?;

        Some(
            AuthClient::builder()
                .api_url(&self.api_url)
                .anon_key(&self.anon_key)
                .service_role_key(&service_role_key)
                .build()
                .expect("Failed to create admin auth client"),
        )
    }
}

/// Test user helper
pub struct TestUser {
    pub email: String,
    pub password: String,
    pub id: uuid::Uuid,
    pub access_token: String,
    client: AuthClient,
}

impl TestUser {
    /// Create a new test user
    pub async fn create(client: AuthClient) -> anyhow::Result<Self> {
        let email = format!("test-{}@example.com", uuid::Uuid::new_v4());
        let password = "TestPassword123!";

        let (user, access_token) = client
            .signup(IdType::Email(email.clone()), password.to_string(), None)
            .await?;

        Ok(Self {
            email,
            password: password.to_string(),
            id: user.id,
            access_token,
            client,
        })
    }

    /// Sign in and get new tokens
    pub async fn signin(
        &self,
    ) -> anyhow::Result<supabase_auth_redux::models::token::TokenResponse> {
        self.client
            .signin_with_password(IdType::Email(self.email.clone()), self.password.clone())
            .await
            .map_err(Into::into)
    }
}

impl Drop for TestUser {
    fn drop(&mut self) {
        // Best effort cleanup - delete user when test is done
        // Try to get admin client for deletion
        if let Some(admin_client) = TestConfig::from_env().create_admin_client() {
            let user_id = self.id;
            tokio::spawn(async move {
                let _ = admin_client.hard_delete_user(user_id).await;
            });
        }
    }
}

/// Skip test if Supabase is not running
#[macro_export]
macro_rules! require_supabase {
    () => {
        // Try to check if Supabase is accessible
        let client = reqwest::Client::new();
        let check_url =
            std::env::var("SUPABASE_URL").unwrap_or_else(|_| "http://127.0.0.1:54321".to_string());

        match client.get(&check_url).send().await {
            Ok(resp) if resp.status().is_success() => {
                // Supabase is running
            }
            _ => {
                eprintln!("Skipping test - Supabase is not running at {}", check_url);
                eprintln!("Run 'supabase start' to enable this test");
                return;
            }
        }
    };
}
