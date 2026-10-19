use crate::gui::types::update_status::UpdateStatus;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::formatted_strings::APP_VERSION;
use crate::{SNIFFNET_LOWERCASE, location};
use semver::Version;
use serde::Deserialize;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct AppVersion {
    name: String,
}

/// Checks whether a newer release of Sniffnet is available on GitHub
pub async fn is_newer_release_available() -> UpdateStatus {
    is_newer_release_available_inner(6, 30).await
}

async fn is_newer_release_available_inner(
    max_retries: u8,
    seconds_between_retries: u8,
) -> UpdateStatus {
    let Ok(client) = reqwest::Client::builder()
        .user_agent(format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
        .build()
        .log_err(location!())
    else {
        return UpdateStatus::Unknown;
    };
    let response = client
        .get("https://api.github.com/repos/GyulyVGC/sniffnet/releases/latest")
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

        // release name sample: v1.2.3
        let stripped = latest_version.trim_start_matches('v');

        if let (Ok(latest_semver), Ok(current_semver)) =
            (Version::parse(stripped), Version::parse(APP_VERSION))
        {
            return if latest_semver > current_semver {
                UpdateStatus::UpdateAvailable(stripped.to_string())
            } else {
                UpdateStatus::UpToDate
            };
        }
    }
    let retries_left = max_retries.saturating_sub(1);
    if retries_left > 0 {
        // sleep seconds_between_retries and retries the request
        tokio::time::sleep(Duration::from_secs(u64::from(seconds_between_retries))).await;
        Box::pin(is_newer_release_available_inner(
            retries_left,
            seconds_between_retries,
        ))
        .await
    } else {
        UpdateStatus::Unknown
    }
}

#[cfg(all(test, not(target_os = "macos")))]
mod tests {
    use super::*;
    use std::assert_matches;

    #[tokio::test]
    async fn test_fetch_latest_release_from_github() {
        let result = is_newer_release_available_inner(6, 2).await;
        assert_matches!(
            result,
            UpdateStatus::UpToDate | UpdateStatus::UpdateAvailable(_)
        );
    }
}
