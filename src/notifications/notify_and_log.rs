use crate::gui::types::favorite::{FavoriteItem, Favorites};
use crate::networking::manage_packets::get_address_to_lookup;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::data_representation::DataRepr;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use crate::notifications::types::logged_notification::{
    BlacklistedTransmitted, DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
    LoggedNotifications,
};
use crate::notifications::types::notifications::{Notifications, RemoteNotifications};
use crate::notifications::types::sound::{Sound, play};
use crate::report::types::sort_type::SortType;
use crate::utils::error_logger::{ErrorLogger, Location};
use crate::utils::formatted_strings::APP_VERSION;
use crate::utils::formatted_strings::get_formatted_timestamp;
use crate::{InfoTraffic, SNIFFNET_LOWERCASE, location};
use std::collections::HashMap;
use std::net::IpAddr;

/// Checks if one or more notifications have to be emitted and logs them.
///
/// It returns the number of new notifications emitted
pub fn notify_and_log(
    logged_notifications: &mut LoggedNotifications,
    notifications: &Notifications,
    info_traffic_msg: &InfoTraffic,
    favorites: &Favorites,
    cs: &CaptureSource,
    addresses_resolved: &HashMap<IpAddr, (String, Host)>,
) -> usize {
    let mut sound_to_play = Sound::None;
    let emitted_notifications_prev = logged_notifications.tot();
    let timestamp = info_traffic_msg.last_packet_timestamp;
    let data_info = info_traffic_msg.tot_data_info;

    // data threshold
    if let Some(threshold) = notifications.data_notification.threshold {
        let data_repr = notifications.data_notification.data_repr;
        if data_info.tot_data(data_repr) > u128::from(threshold) {
            let notification = LoggedNotification::DataThresholdExceeded(DataThresholdExceeded {
                id: logged_notifications.tot(),
                data_repr,
                threshold: notifications.data_notification.previous_threshold,
                data_info,
                timestamp: get_formatted_timestamp(timestamp),
                is_expanded: false,
                hosts: threshold_hosts(info_traffic_msg, data_repr),
                services: threshold_services(info_traffic_msg, data_repr),
            });

            //log this notification
            logged_notifications.push(&notification);

            // send remote notification
            send_remote_notification(notification, notifications.remote_notifications.clone());

            // register sound to play
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.data_notification.sound;
            }
        }
    }

    // from favorites
    if notifications.favorite_notification.is_active {
        let favorites_last_interval = favorites_last_interval(info_traffic_msg, favorites);

        if !favorites_last_interval.is_empty() {
            for favorite in favorites_last_interval {
                let notification = LoggedNotification::FavoriteTransmitted(FavoriteTransmitted {
                    id: logged_notifications.tot(),
                    favorite,
                    timestamp: get_formatted_timestamp(timestamp),
                });

                //log this notification
                logged_notifications.push(&notification);

                // send remote notification
                send_remote_notification(notification, notifications.remote_notifications.clone());
            }

            // register sound to play
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.favorite_notification.sound;
            }
        }
    }

    // IP blacklist
    if notifications.ip_blacklist_notification.is_active {
        let mut blacklisted_last_interval: HashMap<IpAddr, (Host, DataInfoHost)> = HashMap::new();
        for (k, v) in info_traffic_msg
            .map
            .iter()
            .filter(|(_, v)| v.is_blacklisted)
        {
            let address_to_lookup = &get_address_to_lookup(k, v.traffic_direction);
            let host = addresses_resolved
                .get(address_to_lookup)
                .map(|(_, h)| h.clone())
                .unwrap_or_default();
            let mut data_info_host = info_traffic_msg
                .hosts
                .get(&host)
                .copied()
                .unwrap_or_default();
            data_info_host.data_info = v.data_info();
            blacklisted_last_interval
                .entry(*address_to_lookup)
                .and_modify(|(_, existing_data_info_host)| {
                    existing_data_info_host
                        .data_info
                        .refresh(data_info_host.data_info);
                })
                .or_insert((host, data_info_host));
        }
        if !blacklisted_last_interval.is_empty() {
            for (ip, (host, data_info_host)) in blacklisted_last_interval {
                let notification =
                    LoggedNotification::BlacklistedTransmitted(BlacklistedTransmitted {
                        id: logged_notifications.tot(),
                        ip,
                        host,
                        data_info_host,
                        timestamp: get_formatted_timestamp(timestamp),
                    });

                //log this notification
                logged_notifications.push(&notification);

                // send remote notification
                send_remote_notification(notification, notifications.remote_notifications.clone());
            }

            // register sound to play
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.ip_blacklist_notification.sound;
            }
        }
    }

    // don't play sound when importing data from pcap file
    if matches!(cs, CaptureSource::Device(_)) {
        play(sound_to_play, notifications.volume);
    }

    logged_notifications.tot() - emitted_notifications_prev
}

fn threshold_hosts(
    info_traffic_msg: &InfoTraffic,
    data_repr: DataRepr,
) -> Vec<(Host, DataInfoHost)> {
    let mut hosts: Vec<(Host, DataInfoHost)> = info_traffic_msg
        .hosts
        .iter()
        .map(|(h, data)| (h.clone(), *data))
        .collect();
    hosts.sort_by(|(_, a), (_, b)| {
        a.data_info
            .compare(&b.data_info, SortType::Descending, data_repr)
    });
    hosts.truncate(4);
    hosts
}

fn threshold_services(
    info_traffic_msg: &InfoTraffic,
    data_repr: DataRepr,
) -> Vec<(Service, DataInfo)> {
    let mut services: Vec<(Service, DataInfo)> = info_traffic_msg
        .services
        .iter()
        .filter(|(service, _)| service != &&Service::NotApplicable)
        .map(|(s, data_info)| (*s, *data_info))
        .collect();
    services.sort_by(|(_, a), (_, b)| a.compare(b, SortType::Descending, data_repr));
    services.truncate(4);
    services
}

fn favorites_last_interval(
    info_traffic_msg: &InfoTraffic,
    favorites: &Favorites,
) -> Vec<FavoriteItem> {
    let hosts = favorites.hosts().iter().filter_map(|h| {
        info_traffic_msg
            .hosts
            .get(h)
            .map(|d| FavoriteItem::Host((h.clone(), *d)))
    });

    let services = favorites.services().iter().filter_map(|s| {
        info_traffic_msg
            .services
            .get(s)
            .map(|d| FavoriteItem::Service((*s, *d)))
    });

    let programs = favorites.programs().iter().filter_map(|p| {
        let mut data_info = DataInfo::default();
        info_traffic_msg
            .map
            .values()
            .filter(|v| v.program.eq(p))
            .for_each(|v| data_info.refresh(v.data_info()));
        if data_info.tot_data(DataRepr::Packets) > 0 {
            Some(FavoriteItem::Program((p.clone(), data_info)))
        } else {
            None
        }
    });

    hosts.chain(services).chain(programs).collect()
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
