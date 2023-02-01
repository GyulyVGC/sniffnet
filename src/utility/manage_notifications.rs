use crate::enums::sound::{play_sound, Sound};
use crate::structs::notifications::Notifications;
use crate::RunTimeData;
use std::cell::Ref;

pub fn notify(runtime_data: &Ref<RunTimeData>, notifications: Notifications) {
    let mut already_emitted_sound = false;
    // packets threshold
    if notifications.packets_notification.threshold.is_some() {
        let sent_packets_entry = runtime_data.tot_sent_packets_prev - runtime_data.tot_sent_packets;
        let received_packets_entry =
            runtime_data.tot_received_packets - runtime_data.tot_received_packets_prev;
        if received_packets_entry - sent_packets_entry
            > notifications.packets_notification.threshold.unwrap() as i128
        {
            // log this notification
            // ...
            if notifications.packets_notification.sound.ne(&Sound::None) {
                // emit sound
                play_sound(notifications.packets_notification.sound);
                already_emitted_sound = true;
            }
        }
    }
    // bytes threshold
    if notifications.bytes_notification.threshold.is_some() {
        let sent_bytes_entry = runtime_data.tot_sent_bytes_prev - runtime_data.tot_sent_bytes;
        let received_bytes_entry =
            runtime_data.tot_received_bytes - runtime_data.tot_received_bytes_prev;
        if received_bytes_entry - sent_bytes_entry
            > notifications.bytes_notification.threshold.unwrap() as i128
        {
            //log this notification
            // ...
            if !already_emitted_sound && notifications.bytes_notification.sound.ne(&Sound::None) {
                // emit sound
                play_sound(notifications.bytes_notification.sound);
                already_emitted_sound = true;
            }
        }
    }
    // from favorites
    if notifications.on_favorite_notification.notify_on_favorite {
        // qui andrà condizione: sono arrivati nuovi pacchi da favorite connections?
        if true {
            //log this notification
            // ...
            if !already_emitted_sound && notifications.on_favorite_notification.sound.ne(&Sound::None) {
                // emit sound
                play_sound(notifications.on_favorite_notification.sound);
            }
        }
    }
}
