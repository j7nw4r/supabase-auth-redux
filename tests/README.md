# Supabase Auth RS Tests

This directory contains comprehensive tests for the `supabase-auth-rs` library.

## Test Structure

- `integration_tests.rs` - Full integration tests that require a running Supabase instance
- `unit_tests.rs` - Unit tests for basic functionality that don't require external services
- `auth_flow_tests.rs` - Complex authentication flow tests with helpers
- `test_helper.rs` - Shared test utilities and helpers

## Running Tests

### Prerequisites

1. Start a local Supabase instance:
   ```bash
   cd /path/to/your/supabase/project
   supabase start
   ```

2. Note the API URL and anon key from the output

### Running All Tests

```bash
# Run all tests (including integration tests)
cargo test

# Run with output
cargo test -- --nocapture

# Run tests with debug logging
RUST_LOG=debug cargo test -- --nocapture
```

### Running Specific Test Files

```bash
# Unit tests only (no Supabase required)
cargo test --test unit_tests

# Integration tests
cargo test --test integration_tests

# Auth flow tests
cargo test --test auth_flow_tests
```

### Running Without Supabase

If Supabase is not running, tests that use the `require_supabase!()` macro will be skipped automatically:

```bash
cargo test
# Tests requiring Supabase will print: "Skipping test - Supabase is not running"
```

### Environment Variables

You can override the default local Supabase configuration:

```bash
SUPABASE_URL=http://localhost:54321 \
SUPABASE_ANON_KEY=your-anon-key \
cargo test
```

## Test Coverage

The tests cover:

- ✅ User signup (email and phone)
- ✅ User signin
- ✅ Token validation
- ✅ Token refresh
- ✅ User deletion
- ✅ Logout
- ✅ Error handling
- ✅ Invalid inputs
- ✅ Concurrent operations
- ✅ Metadata handling

## Writing New Tests

Use the test helpers for common operations:

```rust
use test_helper::{TestConfig, TestUser};

#[tokio::test]
async fn test_my_feature() {
    require_supabase!(); // Skip if Supabase not running
    
    let config = TestConfig::from_env();
    let client = config.create_client();
    
    // TestUser auto-cleans up on drop
    let user = TestUser::create(client).await?;
    
    // Your test logic here
}
```

## Troubleshooting

1. **"Supabase is not running"**: Start Supabase with `supabase start`
2. **Authentication errors**: Check that your anon key matches the one from `supabase start`
3. **Connection refused**: Ensure Supabase is running on the expected port (54321 by default)
4. **Test cleanup**: TestUser automatically deletes itself when dropped, but you can manually clean up if needed