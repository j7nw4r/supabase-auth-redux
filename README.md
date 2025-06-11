# supabase-auth-rs

[![Crates.io](https://img.shields.io/crates/v/supabase-auth-rs.svg)](https://crates.io/crates/supabase-auth-rs)
[![Documentation](https://docs.rs/supabase-auth-rs/badge.svg)](https://docs.rs/supabase-auth-rs)
[![License](https://img.shields.io/crates/l/supabase-auth-rs.svg)](https://github.com/tunemenu/supabase-auth-rs#license)
[![CI](https://github.com/tunemenu/supabase-auth-rs/workflows/CI/badge.svg)](https://github.com/tunemenu/supabase-auth-rs/actions)

A Rust client library for the [Supabase Auth](https://supabase.com/docs/guides/auth) API.

## Features

- 🔐 Full authentication flow support (signup, signin, logout)
- 🔄 Token management (refresh, validation)
- 👤 User management (get user, update, delete)
- 🛡️ Admin operations with service role key
- 🚀 Async/await support with Tokio
- 📝 Comprehensive error handling
- 🎯 Type-safe API

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
supabase-auth-rs = "0.1.0"
```

## Quick Start

```rust
use supabase_auth_rs::{AuthClient, AuthError, IdType};

#[tokio::main]
async fn main() -> Result<(), AuthError> {
    // Initialize the client
    let auth_client = AuthClient::new(
        "https://your-project.supabase.co",
        "your-anon-key"
    )?;

    // Sign up a new user
    let (user, access_token) = auth_client
        .signup(
            IdType::Email("user@example.com".to_string()),
            "secure_password".to_string(),
            None,
        )
        .await?;

    println!("User created: {}", user.id);

    // Sign in an existing user
    let token_response = auth_client
        .signin_with_password(
            IdType::Email("user@example.com".to_string()),
            "secure_password".to_string(),
        )
        .await?;

    println!("Access token: {}", token_response.access_token);

    Ok(())
}
```

## Advanced Usage

### Using the Builder Pattern

For more control over the client configuration, use the builder pattern:

```rust
use supabase_auth_rs::AuthClient;

let auth_client = AuthClient::builder()
    .api_url("https://your-project.supabase.co")
    .anon_key("your-anon-key")
    .service_role_key("your-service-role-key")  // Optional: for admin operations
    .build()?;
```

### User Management

```rust
// Get user by access token
let user = auth_client
    .get_user_by_token(&access_token)
    .await?;

// Refresh tokens
let new_tokens = auth_client
    .refresh_token(&refresh_token)
    .await?;

// Logout
auth_client
    .logout(&access_token)
    .await?;
```

### Admin Operations

Admin operations require a service role key:

```rust
let admin_client = AuthClient::builder()
    .api_url("https://your-project.supabase.co")
    .anon_key("your-anon-key")
    .service_role_key("your-service-role-key")
    .build()?;

// Hard delete a user (requires service role key)
admin_client
    .hard_delete_user(user_id)
    .await?;

// Soft delete a user (marks as deleted but keeps data)
admin_client
    .soft_delete_user(user_id)
    .await?;
```

## API Reference

### Authentication Methods

- `signup()` - Create a new user account
- `signin_with_password()` - Sign in with email/phone and password
- `logout()` - Sign out a user

### Token Management

- `refresh_token()` - Refresh access tokens
- `get_user_by_token()` - Validate a token and get user info

### User Management

- `get_user_by_id()` - Get user by UUID (requires service role key)
- `hard_delete_user()` - Permanently delete a user account
- `soft_delete_user()` - Mark user as deleted but keep data

## Error Handling

The library provides a comprehensive `AuthError` enum for different error scenarios:

```rust
use supabase_auth_rs::AuthError;

match auth_client.signin_with_password(id_type, password).await {
    Ok(token_response) => {
        println!("Signed in successfully!");
    }
    Err(AuthError::NotAuthorized) => {
        println!("Invalid credentials");
    }
    Err(AuthError::InvalidParameters) => {
        println!("Invalid input parameters");
    }
    Err(e) => {
        println!("An error occurred: {}", e);
    }
}
```

## Requirements

- Rust 1.70 or later
- Tokio runtime

## Development

### Running Tests

Tests require a local Supabase instance:

```bash
# Start local Supabase
supabase start

# Run tests
cargo test

# Run tests with service role key (for admin operations)
SUPABASE_SERVICE_ROLE_KEY=your-service-role-key cargo test
```

### Examples

Check out the [examples](examples/) directory for more detailed usage examples:

```bash
# Run the local Supabase example
cargo run --example local_supabase
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Acknowledgments

- Built for use with [Supabase](https://supabase.com/)
- Inspired by the official Supabase JavaScript client

## Links

- [Crate Documentation](https://docs.rs/supabase-auth-rs)
- [API Documentation](https://supabase.com/docs/reference/auth)
- [Supabase Auth Documentation](https://supabase.com/docs/guides/auth)