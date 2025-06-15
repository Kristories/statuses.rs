use statuses::{code, message};
use serde::Deserialize;
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
        assert_eq!(result_message, status.message, "Failed on code: {}", status.code);

        // Test message -> code
        let result_code = code(&status.message).unwrap();
        assert_eq!(result_code, status.code, "Failed on message: {}", status.message);
    }
}
