use crate::InfoTraffic;
use crate::networking::types::capture_context::CaptureSource;
use crate::notifications::types::logged_notification::{
    DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
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
    cs: &CaptureSource,
) -> usize {
    let mut sound_to_play = Sound::None;
    let mut emitted_notifications = 0;
    let timestamp = info_traffic.last_packet_timestamp;
    let mut data_info_delta = info_traffic.tot_data_info;
    data_info_delta.subtract(info_traffic.tot_data_info_prev);
    // packets threshold
    if let Some(threshold) = notifications.packets_notification.threshold {
        if data_info_delta.tot_packets() > u128::from(threshold) {
            // log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }
            logged_notifications.push_front(LoggedNotification::PacketsThresholdExceeded(
                DataThresholdExceeded {
                    threshold: notifications.packets_notification.previous_threshold,
                    data_info: data_info_delta,
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
        if data_info_delta.tot_bytes() > u128::from(threshold) {
            //log this notification
            emitted_notifications += 1;
            if logged_notifications.len() >= 30 {
                logged_notifications.pop_back();
            }
            logged_notifications.push_front(LoggedNotification::BytesThresholdExceeded(
                DataThresholdExceeded {
                    threshold: notifications.bytes_notification.previous_threshold,
                    data_info: data_info_delta,
                    timestamp: get_formatted_timestamp(timestamp),
                },
            ));
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.bytes_notification.sound;
            }
        }
    }
    // from favorites
    if notifications.favorite_notification.notify_on_favorite
        && !info_traffic.favorites_last_interval.is_empty()
    {
        for (host, data_info_host) in info_traffic.favorites_last_interval.clone() {
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

    // don't play sound when importing data from pcap file
    if matches!(cs, CaptureSource::Device(_)) {
        play(sound_to_play, notifications.volume);
    }

    emitted_notifications
}
