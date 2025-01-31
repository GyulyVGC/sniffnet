pub mod manage_packets;
pub mod types;

use regex::Regex;
use std::collections::HashMap;
use std::sync::Mutex;

lazy_static! {
    static ref INPUT_VALIDATION_REGEX: Regex = Regex::new(r"^[a-zA-Z0-9_\-]+$").unwrap();
}

pub fn validate_and_sanitize_input(input: &str) -> Option<String> {
    if INPUT_VALIDATION_REGEX.is_match(input) {
        Some(input.trim().to_string())
    } else {
        None
    }
}

pub struct SecurityEventLogger {
    events: Mutex<Vec<String>>,
}

impl SecurityEventLogger {
    pub fn new() -> Self {
        SecurityEventLogger {
            events: Mutex::new(Vec::new()),
        }
    }

    pub fn log_event(&self, event: &str) {
        let mut events = self.events.lock().unwrap();
        events.push(event.to_string());
    }

    pub fn get_events(&self) -> Vec<String> {
        let events = self.events.lock().unwrap();
        events.clone()
    }
}

pub struct AuthManager {
    users: HashMap<String, String>,
}

impl AuthManager {
    pub fn new() -> Self {
        AuthManager {
            users: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, username: &str, password: &str) {
        self.users.insert(username.to_string(), password.to_string());
    }

    pub fn authenticate(&self, username: &str, password: &str) -> bool {
        if let Some(stored_password) = self.users.get(username) {
            stored_password == password
        } else {
            false
        }
    }
}
