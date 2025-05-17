use crate::InfoTraffic;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::notifications::types::logged_notification::{
    BytesThresholdExceeded, FavoriteTransmitted, LoggedNotification, PacketsThresholdExceeded,
};
use crate::notifications::types::notifications::Notifications;
use crate::notifications::types::sound::{Sound, play};
use crate::utils::formatted_strings::get_formatted_timestamp;
use std::collections::VecDeque;

/// Checks if one or more notifications have to be emitted and logs them.
///
/// It returns the number of new notifications emitted
pub fn notify_and_log(
    logged_notifications: &mut VecDeque<LoggedNotification>,
    notifications: Notifications,
    info_traffic: &InfoTraffic,
) -> usize {
    let mut already_emitted_sound = false;
    let mut emitted_notifications = 0;
    let timestamp = info_traffic.last_packet_timestamp;
    // packets threshold
    if let Some(threshold) = notifications.packets_notification.threshold {
        let sent_packets_entry = info_traffic.tot_out_packets - info_traffic.tot_out_packets_prev;
        let received_packets_entry = info_traffic.tot_in_packets - info_traffic.tot_in_packets_prev;
        if received_packets_entry + sent_packets_entry > u128::from(threshold) {
            // log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }
            logged_notifications.push_front(LoggedNotification::PacketsThresholdExceeded(
                PacketsThresholdExceeded {
                    threshold: notifications.packets_notification.previous_threshold,
                    incoming: received_packets_entry.try_into().unwrap_or_default(),
                    outgoing: sent_packets_entry.try_into().unwrap_or_default(),
                    timestamp: get_formatted_timestamp(timestamp.unwrap_or_default()),
                },
            ));
            if notifications.packets_notification.sound.ne(&Sound::None) {
                // emit sound
                play(
                    notifications.packets_notification.sound,
                    notifications.volume,
                );
                already_emitted_sound = true;
            }
        }
    }
    // bytes threshold
    if let Some(threshold) = notifications.bytes_notification.threshold {
        let sent_bytes_entry = info_traffic.tot_out_bytes - info_traffic.tot_out_bytes_prev;
        let received_bytes_entry = info_traffic.tot_in_bytes - info_traffic.tot_in_bytes_prev;
        if received_bytes_entry + sent_bytes_entry > u128::from(threshold) {
            //log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }
            logged_notifications.push_front(LoggedNotification::BytesThresholdExceeded(
                BytesThresholdExceeded {
                    threshold: notifications.bytes_notification.previous_threshold,
                    incoming: received_bytes_entry.try_into().unwrap_or_default(),
                    outgoing: sent_bytes_entry.try_into().unwrap_or_default(),
                    timestamp: get_formatted_timestamp(timestamp.unwrap_or_default()),
                },
            ));
            if !already_emitted_sound && notifications.bytes_notification.sound.ne(&Sound::None) {
                // emit sound
                play(notifications.bytes_notification.sound, notifications.volume);
                already_emitted_sound = true;
            }
        }
    }
    // from favorites
    if notifications.favorite_notification.notify_on_favorite
        && !info_traffic.favorites_last_interval.is_empty()
    {
        for host in info_traffic.favorites_last_interval.clone() {
            //log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }

            let data_info_host = *info_traffic
                .hosts
                .get(&host)
                .unwrap_or(&DataInfoHost::default());
            logged_notifications.push_front(LoggedNotification::FavoriteTransmitted(
                FavoriteTransmitted {
                    host,
                    data_info_host,
                    timestamp: get_formatted_timestamp(timestamp.unwrap_or_default()),
                },
            ));
        }
        if !already_emitted_sound && notifications.favorite_notification.sound.ne(&Sound::None) {
            // emit sound
            play(
                notifications.favorite_notification.sound,
                notifications.volume,
            );
        }
    }

    emitted_notifications
}
