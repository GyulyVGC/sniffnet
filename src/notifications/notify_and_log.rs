use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use crate::notifications::types::logged_notification::{
    DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
};
use crate::notifications::types::notifications::{Notifications, RemoteNotifications};
use crate::notifications::types::sound::{Sound, play};
use crate::report::types::sort_type::SortType;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::formatted_strings::get_formatted_timestamp;
use crate::{InfoTraffic, SNIFFNET_LOWERCASE, location};
use std::cmp::min;
use std::collections::{HashSet, VecDeque};

/// Checks if one or more notifications have to be emitted and logs them.
///
/// It returns the number of new notifications emitted
pub fn notify_and_log(
    logged_notifications: &mut (VecDeque<LoggedNotification>, usize),
    notifications: &Notifications,
    info_traffic_msg: &InfoTraffic,
    favorites: &HashSet<Host>,
    cs: &CaptureSource,
) -> usize {
    let mut sound_to_play = Sound::None;
    let emitted_notifications_prev = logged_notifications.1;
    let timestamp = info_traffic_msg.last_packet_timestamp;
    let data_info = info_traffic_msg.tot_data_info;
    // data threshold
    if let Some(threshold) = notifications.data_notification.threshold {
        let data_repr = notifications.data_notification.data_repr;
        if data_info.tot_data(data_repr) > u128::from(threshold) {
            let notification = LoggedNotification::DataThresholdExceeded(DataThresholdExceeded {
                id: logged_notifications.1,
                data_repr,
                threshold: notifications.data_notification.previous_threshold,
                data_info,
                timestamp: get_formatted_timestamp(timestamp),
                is_expanded: false,
                hosts: hosts_list(info_traffic_msg, data_repr),
                services: services_list(info_traffic_msg, data_repr),
            });

            //log this notification
            logged_notifications.1 += 1;
            if logged_notifications.0.len() >= 30 {
                logged_notifications.0.pop_back();
            }
            logged_notifications.0.push_front(notification.clone());

            // send remote notification
            send_remote_notification(notification, notifications.remote_notifications.clone());

            // register sound to play
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.data_notification.sound;
            }
        }
    }

    // from favorites
    if notifications.favorite_notification.notify_on_favorite {
        let favorites_last_interval: HashSet<(Host, DataInfoHost)> = info_traffic_msg
            .hosts
            .iter()
            .filter(|(h, _)| favorites.contains(h))
            .map(|(h, data)| (h.clone(), *data))
            .collect();
        if !favorites_last_interval.is_empty() {
            for (host, data_info_host) in favorites_last_interval {
                let notification = LoggedNotification::FavoriteTransmitted(FavoriteTransmitted {
                    id: logged_notifications.1,
                    host,
                    data_info_host,
                    timestamp: get_formatted_timestamp(timestamp),
                });

                //log this notification
                logged_notifications.1 += 1;
                if logged_notifications.0.len() >= 30 {
                    logged_notifications.0.pop_back();
                }
                logged_notifications.0.push_front(notification.clone());

                // send remote notification
                send_remote_notification(notification, notifications.remote_notifications.clone());
            }

            // register sound to play
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.favorite_notification.sound;
            }
        }
    }

    // don't play sound when importing data from pcap file
    if matches!(cs, CaptureSource::Device(_)) {
        play(sound_to_play, notifications.volume);
    }

    logged_notifications.1 - emitted_notifications_prev
}

fn hosts_list(info_traffic_msg: &InfoTraffic, data_repr: DataRepr) -> Vec<(Host, DataInfoHost)> {
    let mut hosts: Vec<(Host, DataInfoHost)> = info_traffic_msg
        .hosts
        .iter()
        .map(|(h, data)| (h.clone(), *data))
        .collect();
    hosts.sort_by(|(_, a), (_, b)| {
        a.data_info
            .compare(&b.data_info, SortType::Descending, data_repr)
    });
    let n_entry = min(hosts.len(), 4);
    hosts
        .get(..n_entry)
        .unwrap_or_default()
        .to_owned()
        .into_iter()
        .collect()
}

fn services_list(info_traffic_msg: &InfoTraffic, data_repr: DataRepr) -> Vec<(Service, DataInfo)> {
    let mut services: Vec<(Service, DataInfo)> = info_traffic_msg
        .services
        .iter()
        .filter(|(service, _)| service != &&Service::NotApplicable)
        .map(|(s, data)| (*s, *data))
        .collect();
    services.sort_by(|(_, a), (_, b)| a.compare(b, SortType::Descending, data_repr));
    let n_entry = min(services.len(), 4);
    services
        .get(..n_entry)
        .unwrap_or_default()
        .to_owned()
        .into_iter()
        .collect()
}

fn send_remote_notification(
    notification: LoggedNotification,
    remote_notifications: RemoteNotifications,
) {
    if remote_notifications.is_active_and_set() {
        tokio::task::spawn(async move {
            let Ok(client) = reqwest::Client::builder()
                .user_agent(format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
                .build()
                .log_err(location!())
            else {
                return;
            };
            let Ok(response) = client
                .post(remote_notifications.url())
                .header("User-agent", format!("{SNIFFNET_LOWERCASE}-{APP_VERSION}"))
                .body(notification.to_json())
                .send()
                .await
                .log_err(location!())
            else {
                return;
            };
            let _ = response.error_for_status().log_err(location!());
        });
    }
}
