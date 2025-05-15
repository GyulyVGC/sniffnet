use crate::SNIFFNET_LOWERCASE;
use crate::utils::formatted_strings::APP_VERSION;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct AppVersion {
    name: String,
}

/// Calls a method to check if a newer release of Sniffnet is available on GitHub
/// and updates application status accordingly
pub async fn set_newer_release_status() -> Option<bool> {
    is_newer_release_available(6, 30).await
}

/// Checks if a newer release of Sniffnet is available on GitHub
async fn is_newer_release_available(max_retries: u8, seconds_between_retries: u8) -> Option<bool> {
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/repos/GyulyVGC/sniffnet/releases/latest")
        .header("User-agent", format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await;

    if let Ok(result) = response {
        let result_json = result.json::<AppVersion>().await;

        #[cfg(test)]
        if result_json.is_err() {
            let response2 = client
                .get("https://api.github.com/repos/GyulyVGC/sniffnet/releases/latest")
                .header("User-agent", format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
                .header("Accept", "application/vnd.github+json")
                .header("X-GitHub-Api-Version", "2022-11-28")
                .send()
                .await;
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
        // TODO: support versions with numbers of more than 1 digit
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
        tokio::time::sleep(Duration::from_secs(u64::from(seconds_between_retries))).await;
        Box::pin(is_newer_release_available(
            retries_left,
            seconds_between_retries,
        ))
        .await
    } else {
        None
    }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
    use super::*;

    #[tokio::test]
    async fn fetch_latest_release_from_github() {
        let result = is_newer_release_available(6, 2).await;
        result.expect("Latest release request from GitHub error");
    }
}
