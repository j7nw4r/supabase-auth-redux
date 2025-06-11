#!/bin/bash

# Script to run supabase-auth-rs tests

echo "🧪 Running supabase-auth-rs tests..."
echo ""

# Check if Supabase is running
if curl -s http://127.0.0.1:54321 > /dev/null; then
    echo "✅ Supabase is running"
    SUPABASE_RUNNING=true
else
    echo "⚠️  Supabase is not running. Integration tests will be skipped."
    echo "   Run 'supabase start' to enable all tests."
    SUPABASE_RUNNING=false
fi

echo ""

# Always run unit tests
echo "📝 Running unit tests..."
cargo test --test unit_tests

echo ""

if [ "$SUPABASE_RUNNING" = true ]; then
    # Run integration tests if Supabase is available
    echo "🔌 Running integration tests..."
    cargo test --test integration_tests -- --test-threads=1
    
    echo ""
    echo "🔄 Running auth flow tests..."
    cargo test --test auth_flow_tests
    
    echo ""
    echo "🎯 Running example..."
    cargo run --example local_supabase
else
    echo "⏭️  Skipping integration tests (Supabase not running)"
fi

echo ""
echo "✨ Test run complete!"