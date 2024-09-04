// quantum_cryptographic_toolkit/core/utils.rs

use std::collections::HashMap;

/// Logs a message to the toolkit's log storage.
///
/// # Arguments
///
/// * `logs` - A mutable reference to the log storage.
/// * `message` - The log message to be stored.
pub fn log_message(logs: &mut Vec<String>, message: &str) {
    logs.push(message.to_string());
}

/// Clears the cache storage.
///
/// # Arguments
///
/// * `cache` - A mutable reference to the cache storage.
pub fn clear_cache(cache: &mut HashMap<String, Vec<u8>>) {
    cache.clear();
}
