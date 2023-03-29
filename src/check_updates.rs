use crate::utility::get_formatted_strings::APP_VERSION;
use reqwest::header::USER_AGENT;
use serde::Deserialize;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct AppVersion {
    name: String,
}

pub fn check_updates() {
    let _ = is_newer_version_available(6, 30);
}

/// Checks if a newer version of Sniffnet is available on GitHub
fn is_newer_version_available(
    max_retries: u8,
    seconds_between_retries: u8,
) -> Result<bool, String> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/repos/GyulyVGC/Sniffnet/releases/latest")
        .header(USER_AGENT, format!("sniffnet/{APP_VERSION}"))
        .send();

    if let Ok(result) = response {
        let mut latest_version = result
            .json::<AppVersion>()
            .unwrap_or(AppVersion {
                name: String::new(),
            })
            .name;
        if latest_version.len() == 6 {
            latest_version.remove(0);
            return if latest_version.gt(&APP_VERSION.to_string()) {
                Ok(true)
            } else {
                Ok(false)
            };
        }
        Err("Cannot parse latest version name".to_string())
    } else {
        let retries_left = max_retries - 1;
        if retries_left > 0 {
            // sleep 30 seconds and retries the request
            thread::sleep(Duration::from_secs(u64::from(seconds_between_retries)));
            is_newer_version_available(retries_left, seconds_between_retries)
        } else {
            Err(response.err().unwrap().to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fetch_latest_release_from_github() {
        let result = is_newer_version_available(6, 2);
        result.expect("Latest release request from GitHub error");
    }
}
