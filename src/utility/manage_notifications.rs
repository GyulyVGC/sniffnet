use crate::enums::logged_notification::{
    BytesThresholdExceeded, FavoriteTransmitted, LoggedNotification, PacketsThresholdExceeded,
};
use crate::enums::sound::{play, Sound};
use crate::structs::notifications::Notifications;
use crate::{InfoTraffic, RunTimeData};
use chrono::Local;
use std::cell::RefMut;
use std::sync::{Arc, Mutex};

pub fn notify_and_log(
    mut runtime_data: RefMut<RunTimeData>,
    notifications: Notifications,
    info_traffic: &Arc<Mutex<InfoTraffic>>,
) {
    let mut already_emitted_sound = false;
    // packets threshold
    if notifications.packets_notification.threshold.is_some() {
        let sent_packets_entry = runtime_data.tot_sent_packets - runtime_data.tot_sent_packets_prev;
        let received_packets_entry =
            runtime_data.tot_received_packets - runtime_data.tot_received_packets_prev;
        if received_packets_entry + sent_packets_entry
            > u128::from(notifications.packets_notification.threshold.unwrap())
        {
            // log this notification
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
        let sent_bytes_entry = runtime_data.tot_sent_bytes - runtime_data.tot_sent_bytes_prev;
        let received_bytes_entry =
            runtime_data.tot_received_bytes - runtime_data.tot_received_bytes_prev;
        if received_bytes_entry + sent_bytes_entry
            > u128::from(notifications.bytes_notification.threshold.unwrap())
        {
            //log this notification
            if runtime_data.logged_notifications.len() >= 30 {
                runtime_data.logged_notifications.pop_back();
            }
            runtime_data.logged_notifications.push_front(
                LoggedNotification::BytesThresholdExceeded(BytesThresholdExceeded {
                    threshold: notifications.bytes_notification.previous_threshold,
                    byte_multiple: notifications.bytes_notification.byte_multiple,
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
        && !runtime_data.favorites_last_interval.is_empty()
    {
        let info_traffic_lock = info_traffic.lock().unwrap();
        for index in &runtime_data.favorites_last_interval.clone() {
            //log this notification
            if runtime_data.logged_notifications.len() >= 30 {
                runtime_data.logged_notifications.pop_back();
            }
            let key_val = info_traffic_lock.map.get_index(*index).unwrap();
            runtime_data
                .logged_notifications
                .push_front(LoggedNotification::FavoriteTransmitted(
                    FavoriteTransmitted {
                        connection: (key_val.0.clone(), key_val.1.clone()),
                        timestamp: Local::now().to_string().get(11..19).unwrap().to_string(),
                    },
                ));
            if !already_emitted_sound && notifications.favorite_notification.sound.ne(&Sound::None)
            {
                // emit sound
                play(
                    notifications.favorite_notification.sound,
                    notifications.volume,
                );
                already_emitted_sound = true;
            }
        }
    }
}
