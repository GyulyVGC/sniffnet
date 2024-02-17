use std::sync::{Arc, Mutex};

use chrono::Local;

use crate::notifications::types::logged_notification::{
    BytesThresholdExceeded, FavoriteTransmitted, LoggedNotification, PacketsThresholdExceeded,
};
use crate::notifications::types::notifications::Notifications;
use crate::notifications::types::sound::{play, Sound};
use crate::{InfoTraffic, RunTimeData};

/// Checks if one or more notifications have to be emitted and logs them.
///
/// It returns the number of new notifications emitted
pub fn notify_and_log(
    runtime_data: &mut RunTimeData,
    notifications: Notifications,
    info_traffic: &Arc<Mutex<InfoTraffic>>,
) -> usize {
    let mut already_emitted_sound = false;
    let mut emitted_notifications = 0;
    // packets threshold
    if notifications.packets_notification.threshold.is_some() {
        let sent_packets_entry = runtime_data.tot_out_packets - runtime_data.tot_out_packets_prev;
        let received_packets_entry = runtime_data.tot_in_packets - runtime_data.tot_in_packets_prev;
        if received_packets_entry + sent_packets_entry
            > u128::from(notifications.packets_notification.threshold.unwrap())
        {
            // log this notification
            emitted_notifications += 1;
            if runtime_data.logged_notifications.len() >= 30 {
                runtime_data.logged_notifications.pop_back();
            }
            runtime_data.logged_notifications.push_front(
                LoggedNotification::PacketsThresholdExceeded(PacketsThresholdExceeded {
                    threshold: notifications.packets_notification.previous_threshold,
                    incoming: received_packets_entry.try_into().unwrap(),
                    outgoing: sent_packets_entry.try_into().unwrap(),
                    timestamp: Local::now().to_string().get(11..19).unwrap().to_string(),
                }),
            );
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
    if notifications.bytes_notification.threshold.is_some() {
        let sent_bytes_entry = runtime_data.tot_out_bytes - runtime_data.tot_out_bytes_prev;
        let received_bytes_entry = runtime_data.tot_in_bytes - runtime_data.tot_in_bytes_prev;
        if received_bytes_entry + sent_bytes_entry
            > u128::from(notifications.bytes_notification.threshold.unwrap())
        {
            //log this notification
            emitted_notifications += 1;
            if runtime_data.logged_notifications.len() >= 30 {
                runtime_data.logged_notifications.pop_back();
            }
            runtime_data.logged_notifications.push_front(
                LoggedNotification::BytesThresholdExceeded(BytesThresholdExceeded {
                    threshold: notifications.bytes_notification.previous_threshold,
                    incoming: received_bytes_entry.try_into().unwrap(),
                    outgoing: sent_bytes_entry.try_into().unwrap(),
                    timestamp: Local::now().to_string().get(11..19).unwrap().to_string(),
                }),
            );
            if !already_emitted_sound && notifications.bytes_notification.sound.ne(&Sound::None) {
                // emit sound
                play(notifications.bytes_notification.sound, notifications.volume);
                already_emitted_sound = true;
            }
        }
    }
    // from favorites
    if notifications.favorite_notification.notify_on_favorite
        && !info_traffic
            .lock()
            .unwrap()
            .favorites_last_interval
            .is_empty()
    {
        let info_traffic_lock = info_traffic.lock().unwrap();
        for host in &info_traffic_lock.favorites_last_interval.clone() {
            //log this notification
            emitted_notifications += 1;
            if runtime_data.logged_notifications.len() >= 30 {
                runtime_data.logged_notifications.pop_back();
            }

            runtime_data
                .logged_notifications
                .push_front(LoggedNotification::FavoriteTransmitted(
                    FavoriteTransmitted {
                        host: host.clone(),
                        data_info_host: *info_traffic_lock.hosts.get(host).unwrap(),
                        timestamp: Local::now().to_string().get(11..19).unwrap().to_string(),
                    },
                ));
        }
        drop(info_traffic_lock);
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
