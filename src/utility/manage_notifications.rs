use crate::enums::logged_notification::LoggedNotification;
use crate::enums::sound::{play_sound, Sound};
use crate::structs::notifications::Notifications;
use crate::RunTimeData;
use chrono::Local;
use std::cell::RefMut;

pub fn notify_and_log(mut runtime_data: RefMut<RunTimeData>, notifications: Notifications) {
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
                LoggedNotification::PacketsThresholdExceeded {
                    notification: notifications.packets_notification,
                    incoming: received_packets_entry.try_into().unwrap(),
                    outgoing: sent_packets_entry.try_into().unwrap(),
                    timestamp: Local::now().to_string().get(0..19).unwrap().to_string(),
                },
            );
            if notifications.packets_notification.sound.ne(&Sound::None) {
                // emit sound
                play_sound(
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
                LoggedNotification::BytesThresholdExceeded {
                    notification: notifications.bytes_notification,
                    incoming: received_bytes_entry.try_into().unwrap(),
                    outgoing: sent_bytes_entry.try_into().unwrap(),
                    timestamp: Local::now().to_string().get(0..19).unwrap().to_string(),
                },
            );
            if !already_emitted_sound && notifications.bytes_notification.sound.ne(&Sound::None) {
                // emit sound
                play_sound(notifications.bytes_notification.sound, notifications.volume);
                already_emitted_sound = true;
            }
        }
    }
    // from favorites
    if notifications.favorite_notification.notify_on_favorite
        && runtime_data.favorite_featured_last_interval.is_some()
    {
        //log this notification
        if runtime_data.logged_notifications.len() >= 30 {
            runtime_data.logged_notifications.pop_back();
        }
        let favorite_featured = runtime_data
            .favorite_featured_last_interval
            .as_ref()
            .unwrap()
            .clone();
        runtime_data
            .logged_notifications
            .push_front(LoggedNotification::FavoriteTransmitted {
                notification: notifications.favorite_notification,
                connection: favorite_featured,
                timestamp: Local::now().to_string().get(0..19).unwrap().to_string(),
            });
        if !already_emitted_sound && notifications.favorite_notification.sound.ne(&Sound::None) {
            // emit sound
            play_sound(
                notifications.favorite_notification.sound,
                notifications.volume,
            );
        }
    }
}
