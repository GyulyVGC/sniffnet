use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use serde::Deserialize;

use crate::utils::formatted_strings::APP_VERSION;
use crate::SNIFFNET_LOWERCASE;

#[derive(Deserialize, Debug)]
struct AppVersion {
    name: String,
}

/// Calls a method to check if a newer release of Sniffnet is available on GitHub
/// and updates application status accordingly
pub fn set_newer_release_status(newer_release_available: &Arc<Mutex<Option<bool>>>) {
    let result = is_newer_release_available(6, 30);
    *newer_release_available.lock().unwrap() = result;
}

/// Checks if a newer release of Sniffnet is available on GitHub
fn is_newer_release_available(max_retries: u8, seconds_between_retries: u8) -> Option<bool> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get("https://api.github.com/repos/GyulyVGC/sniffnet/releases/latest")
        .header("User-agent", format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send();

    if let Ok(result) = response {
        let result_json = result.json::<AppVersion>();

        #[cfg(test)]
        if result_json.is_err() {
            let response2 = client
                .get("https://api.github.com/repos/GyulyVGC/sniffnet/releases/latest")
                .header("User-agent", format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send();
            println!("\nResponse text: {:?}", response2.unwrap());
            println!("JSON result: {result_json:?}\n");
        }

        let mut latest_version = result_json
            .unwrap_or_else(|_| AppVersion {
                name: String::from(":-("),
            })
            .name;
        latest_version = latest_version.trim().to_string();

        // release name sample: v1.1.2
        let latest_version_as_bytes = latest_version.as_bytes();
        if latest_version.len() == 6
            && latest_version.starts_with('v')
            && char::from(latest_version_as_bytes[1]).is_numeric()
            && char::from(latest_version_as_bytes[2]).eq(&'.')
            && char::from(latest_version_as_bytes[3]).is_numeric()
            && char::from(latest_version_as_bytes[4]).eq(&'.')
            && char::from(latest_version_as_bytes[5]).is_numeric()
        {
            latest_version.remove(0);
            return if latest_version.gt(&APP_VERSION.to_string()) {
                Some(true)
            } else {
                Some(false)
            };
        }
    }
    let retries_left = max_retries - 1;
    if retries_left > 0 {
        // sleep seconds_between_retries and retries the request
        thread::sleep(Duration::from_secs(u64::from(seconds_between_retries)));
        is_newer_release_available(retries_left, seconds_between_retries)
    } else {
        None
    }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
    use super::*;

    #[test]
    fn fetch_latest_release_from_github() {
        let result = is_newer_release_available(6, 2);
        result.expect("Latest release request from GitHub error");
    }
}
