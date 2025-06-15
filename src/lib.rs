use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::OnceLock;
use thiserror::Error;

/// Custom error types for status code operations
#[derive(Error, Debug)]
pub enum StatusError {
    #[error("Status code or message does not exist")]
    NotFound,
    #[error("Failed to read status codes file: {0}")]
    FileError(#[from] std::io::Error),
    #[error("Failed to parse JSON data: {0}")]
    JsonError(#[from] serde_json::Error),
}

/// Represents an HTTP status with its code and message
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Status {
    pub code: String,
    pub message: String,
}

/// Global cache for code-to-message lookups
static CODE_TO_MESSAGE: OnceLock<HashMap<String, String>> = OnceLock::new();

/// Global cache for message-to-code lookups  
static MESSAGE_TO_CODE: OnceLock<HashMap<String, String>> = OnceLock::new();

/// Normalizes input strings for consistent key matching
/// Converts to lowercase and trims whitespace
fn normalize_key(s: &str) -> String {
    s.trim().to_lowercase()
}

/// Loads status codes from JSON file and creates bidirectional lookup maps
///
/// # Returns
///
/// A tuple containing (code_to_message_map, message_to_code_map)
///
/// # Errors
///
/// Returns `StatusError` if file reading or JSON parsing fails
fn load_status_maps() -> Result<(HashMap<String, String>, HashMap<String, String>), StatusError> {
    // Read the JSON configuration file
    let json_content = std::fs::read_to_string("codes.json")?;
    let statuses: Vec<Status> = serde_json::from_str(&json_content)?;

    // Build bidirectional lookup maps
    let mut code_to_message = HashMap::with_capacity(statuses.len());
    let mut message_to_code = HashMap::with_capacity(statuses.len());

    for status in statuses {
        let normalized_code = normalize_key(&status.code);
        let normalized_message = normalize_key(&status.message);

        code_to_message.insert(normalized_code, status.message.clone());
        message_to_code.insert(normalized_message, status.code.clone());
    }

    // Debug output for development (consider using log crate in production)
    #[cfg(debug_assertions)]
    {
        println!("Loaded {} status codes", code_to_message.len());
        println!("Code-to-message cache: {:?}", code_to_message);
        println!("Message-to-code cache: {:?}", message_to_code);
    }

    Ok((code_to_message, message_to_code))
}

/// Returns a reference to the global code-to-message lookup map
/// Initializes the cache lazily on first access
fn get_code_to_message() -> &'static HashMap<String, String> {
    CODE_TO_MESSAGE.get_or_init(|| {
        let (code_to_message, _) =
            load_status_maps().expect("Failed to load status codes from file");
        code_to_message
    })
}

/// Returns a reference to the global message-to-code lookup map
/// Initializes the cache lazily on first access
fn get_message_to_code() -> &'static HashMap<String, String> {
    MESSAGE_TO_CODE.get_or_init(|| {
        let (_, message_to_code) =
            load_status_maps().expect("Failed to load status codes from file");
        message_to_code
    })
}

/// Retrieves the HTTP status code for a given status message
///
/// # Arguments
///
/// * `message` - The HTTP status message (e.g., "OK", "Not Found", "Internal Server Error")
///
/// # Returns
///
/// Returns the corresponding status code as a string on success
///
/// # Errors
///
/// Returns `StatusError::NotFound` if the message doesn't exist in the lookup table
///
/// # Examples
///
/// ```
/// use statuses::code;
///
/// let status_code = code("OK").unwrap();
/// assert_eq!(status_code, "200");
///
/// let not_found_code = code("Not Found").unwrap();
/// assert_eq!(not_found_code, "404");
///
/// // Case-insensitive matching
/// let ok_code = code("ok").unwrap();
/// assert_eq!(ok_code, "200");
/// ```
pub fn code(message: &str) -> Result<String, StatusError> {
    get_message_to_code()
        .get(&normalize_key(message))
        .cloned()
        .ok_or(StatusError::NotFound)
}

/// Retrieves the HTTP status message for a given status code
///
/// # Arguments
///
/// * `code` - The HTTP status code (e.g., "200", "404", "500")
///
/// # Returns
///
/// Returns the corresponding status message as a string on success
///
/// # Errors
///
/// Returns `StatusError::NotFound` if the code doesn't exist in the lookup table
///
/// # Examples
///
/// ```
/// use statuses::message;
///
/// let status_message = message("200").unwrap();
/// assert_eq!(status_message, "OK");
///
/// let not_found_message = message("404").unwrap();
/// assert_eq!(not_found_message, "Not Found");
///
/// // Leading/trailing whitespace is handled
/// let ok_message = message(" 200 ").unwrap();
/// assert_eq!(ok_message, "OK");
/// ```
pub fn message(code: &str) -> Result<String, StatusError> {
    get_code_to_message()
        .get(&normalize_key(code))
        .cloned()
        .ok_or(StatusError::NotFound)
}

/// Checks if a given status code exists in the lookup table
///
/// # Arguments
///
/// * `code` - The HTTP status code to check
///
/// # Returns
///
/// Returns `true` if the code exists, `false` otherwise
///
/// # Examples
///
/// ```
/// use statuses::is_valid_code;
///
/// assert!(is_valid_code("200"));
/// assert!(is_valid_code("404"));
/// assert!(!is_valid_code("999"));
/// ```
pub fn is_valid_code(code: &str) -> bool {
    get_code_to_message().contains_key(&normalize_key(code))
}

/// Checks if a given status message exists in the lookup table
///
/// # Arguments
///
/// * `message` - The HTTP status message to check
///
/// # Returns
///
/// Returns `true` if the message exists, `false` otherwise
///
/// # Examples
///
/// ```
/// use statuses::is_valid_message;
///
/// assert!(is_valid_message("OK"));
/// assert!(is_valid_message("Not Found"));
/// assert!(!is_valid_message("Invalid Status"));
/// ```
pub fn is_valid_message(message: &str) -> bool {
    get_message_to_code().contains_key(&normalize_key(message))
}

/// Returns all available status codes as a vector
///
/// # Returns
///
/// A vector containing all valid HTTP status codes
///
/// # Examples
///
/// ```
/// use statuses::all_codes;
///
/// let codes = all_codes();
/// assert!(codes.contains(&"200".to_string()));
/// assert!(codes.contains(&"404".to_string()));
/// ```
pub fn all_codes() -> Vec<String> {
    get_message_to_code().values().cloned().collect()
}

/// Returns all available status messages as a vector
///
/// # Returns
///
/// A vector containing all valid HTTP status messages
///
/// # Examples
///
/// ```
/// use statuses::all_messages;
///
/// let messages = all_messages();
/// assert!(messages.contains(&"OK".to_string()));
/// assert!(messages.contains(&"Not Found".to_string()));
/// ```
pub fn all_messages() -> Vec<String> {
    get_code_to_message().values().cloned().collect()
}
