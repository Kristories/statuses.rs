use serde::Deserialize;
use statuses::{code, is_valid_code, is_valid_message, message, StatusError};
use std::fs;

#[derive(Debug, Deserialize)]
struct StatusCode {
    code: String,
    message: String,
}

#[test]
fn test_all_status_codes() {
    // Load codes.json
    let data = fs::read_to_string("codes.json").expect("Unable to read codes.json");
    let codes: Vec<StatusCode> = serde_json::from_str(&data).expect("Unable to parse codes.json");

    for status in codes {
        // Test code -> message
        let result_message = message(&status.code).unwrap();
        assert_eq!(
            result_message, status.message,
            "Failed on code: {}",
            status.code
        );

        // Test message -> code
        let result_code = code(&status.message).unwrap();
        assert_eq!(
            result_code, status.code,
            "Failed on message: {}",
            status.message
        );
    }
}

#[test]
fn test_case_insensitive_and_whitespace_inputs() {
    assert_eq!(message(" 200 ").unwrap(), "OK");
    assert_eq!(code("  not found  ").unwrap(), "404");
    assert_eq!(code("ok").unwrap(), "200");
}

#[test]
fn test_validation_helpers() {
    assert!(is_valid_code("200"));
    assert!(is_valid_code(" 404 "));
    assert!(is_valid_message("OK"));
    assert!(is_valid_message("  internal server error  "));

    assert!(!is_valid_code("999"));
    assert!(!is_valid_message("invalid status"));
}

#[test]
fn test_invalid_inputs_return_not_found() {
    let err_code = code("Unknown Status").unwrap_err();
    assert!(matches!(err_code, StatusError::NotFound));

    let err_message = message("999").unwrap_err();
    assert!(matches!(err_message, StatusError::NotFound));
}
