# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of supabase-auth-rs
- Full authentication flow support (signup, signin, logout)
- Token management (refresh, validation)
- User management operations
- Admin operations with service role key support
- Builder pattern for AuthClient configuration
- Comprehensive error handling with AuthError enum
- Full async/await support with Tokio
- Documentation for all public APIs
- Examples for common use cases

### Changed
- `soft_delete_user` now accepts `user_id: Uuid` instead of `auth_token: &str`
- `AuthClient::new` now returns `Result<Self, AuthError>` instead of `anyhow::Result<Self>`
- Made `AuthError` enum `#[non_exhaustive]` for future compatibility

### Security
- Service role key is now required for admin operations (user deletion)

## [0.1.0] - TBD

Initial public release.

[Unreleased]: https://github.com/tunemenu/supabase-auth-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/tunemenu/supabase-auth-rs/releases/tag/v0.1.0