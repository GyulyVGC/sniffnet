use crate::InfoTraffic;
use crate::chart::types::chart_type::ChartType;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::notifications::types::logged_notification::{
    DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
};
use crate::notifications::types::notifications::Notifications;
use crate::notifications::types::sound::{Sound, play};
use crate::utils::formatted_strings::get_formatted_timestamp;
use std::collections::{HashSet, VecDeque};

/// Checks if one or more notifications have to be emitted and logs them.
///
/// It returns the number of new notifications emitted
pub fn notify_and_log(
    logged_notifications: &mut VecDeque<LoggedNotification>,
    notifications: Notifications,
    info_traffic_msg: &InfoTraffic,
    favorites: &HashSet<Host>,
    cs: &CaptureSource,
) -> usize {
    let mut sound_to_play = Sound::None;
    let mut emitted_notifications = 0;
    let timestamp = info_traffic_msg.last_packet_timestamp;
    let data_info = info_traffic_msg.tot_data_info;
    // packets threshold
    if let Some(threshold) = notifications.packets_notification.threshold {
        if data_info.tot_packets() > u128::from(threshold) {
            // log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }
            logged_notifications.push_front(LoggedNotification::DataThresholdExceeded(
                DataThresholdExceeded {
                    chart_type: ChartType::Packets,
                    threshold: notifications.packets_notification.previous_threshold,
                    data_info,
                    timestamp: get_formatted_timestamp(timestamp),
                },
            ));
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.packets_notification.sound;
            }
        }
    }
    // bytes threshold
    if let Some(threshold) = notifications.bytes_notification.threshold {
        if data_info.tot_bytes() > u128::from(threshold) {
            //log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }
            logged_notifications.push_front(LoggedNotification::DataThresholdExceeded(
                DataThresholdExceeded {
                    chart_type: ChartType::Bytes,
                    threshold: notifications.bytes_notification.previous_threshold,
                    data_info,
                    timestamp: get_formatted_timestamp(timestamp),
                },
            ));
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.bytes_notification.sound;
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
                //log this notification
                emitted_notifications += 1;
                if logged_notifications.len() >= 30 {
                    logged_notifications.pop_back();
                }

                logged_notifications.push_front(LoggedNotification::FavoriteTransmitted(
                    FavoriteTransmitted {
                        host,
                        data_info_host,
                        timestamp: get_formatted_timestamp(timestamp),
                    },
                ));
            }
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.favorite_notification.sound;
            }
        }
    }

    // don't play sound when importing data from pcap file
    if matches!(cs, CaptureSource::Device(_)) {
        play(sound_to_play, notifications.volume);
    }

    emitted_notifications
}
