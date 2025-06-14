use supabase_auth_redux::{AuthClient, GoTrueErrorResponse};

#[test]
fn test_auth_client_creation() {
    let result = AuthClient::new("http://localhost:54321", "test-key");
    assert!(result.is_ok(), "AuthClient creation should succeed");
}


#[test]
fn test_auth_client_builder() {
    let client = AuthClient::builder()
        .api_url("http://localhost:54321")
        .anon_key("test-anon-key")
        .service_role_key("test-service-key")
        .build()
        .unwrap();
    
    // Just ensure it builds successfully
    let debug_str = format!("{:?}", client);
    assert_eq!(debug_str, "AuthClient");
}

#[test]
fn test_auth_client_builder_missing_url() {
    let result = AuthClient::builder()
        .anon_key("test-anon-key")
        .build();
    
    assert!(result.is_err(), "Builder should fail without API URL");
}

#[test]
fn test_auth_client_builder_missing_anon_key() {
    let result = AuthClient::builder()
        .api_url("http://localhost:54321")
        .build();
    
    assert!(result.is_err(), "Builder should fail without anon key");
}

#[test]
fn test_auth_client_debug() {
    let client = AuthClient::new("http://localhost:54321", "test-key").unwrap();
    let debug_str = format!("{:?}", client);
    assert_eq!(debug_str, "AuthClient");
}

#[test]
fn test_error_schema_display() {
    let error = GoTrueErrorResponse {
        code: Some(40),
        error: Some("Invalid request".to_string()),
        error_description: None,
        msg: None,
    };

    assert_eq!(error.to_string(), "Invalid request");

    let error_with_msg = GoTrueErrorResponse {
        code: Some(50),
        error: None,
        error_description: None,
        msg: Some("Internal error".to_string()),
    };

    assert_eq!(error_with_msg.to_string(), "Internal error");
    
    // Test with error_description
    let error_with_description = GoTrueErrorResponse {
        code: Some(60),
        error: None,
        error_description: Some("Detailed error description".to_string()),
        msg: None,
    };

    assert_eq!(error_with_description.to_string(), "Detailed error description");

    let empty_error = GoTrueErrorResponse {
        code: None,
        error: None,
        error_description: None,
        msg: None,
    };

    // Display trait should return Err for empty error
    use std::fmt::Write;
    let mut buf = String::new();
    let result = write!(&mut buf, "{}", empty_error);
    assert!(result.is_err());
}


#[test]
fn test_id_type_enum() {
    let email_id = supabase_auth_redux::IdType::Email("test@example.com".to_string());
    match email_id {
        supabase_auth_redux::IdType::Email(email) => assert_eq!(email, "test@example.com"),
        _ => panic!("Expected Email variant"),
    }

    let phone_id = supabase_auth_redux::IdType::PhoneNumber("+1234567890".to_string());
    match phone_id {
        supabase_auth_redux::IdType::PhoneNumber(phone) => assert_eq!(phone, "+1234567890"),
        _ => panic!("Expected PhoneNumber variant"),
    }
}
