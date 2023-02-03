use crate::enums::sound::{play_sound, Sound};
use crate::structs::notifications::Notifications;
use crate::RunTimeData;
use std::cell::Ref;

pub fn notify(runtime_data: &Ref<RunTimeData>, notifications: Notifications) {
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
            // ...
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
        let sent_bytes_entry = runtime_data.tot_sent_bytes_prev - runtime_data.tot_sent_bytes;
        let received_bytes_entry =
            runtime_data.tot_received_bytes - runtime_data.tot_received_bytes_prev;
        if received_bytes_entry + sent_bytes_entry
            > u128::from(notifications.bytes_notification.threshold.unwrap())
        {
            //log this notification
            // ...
            if !already_emitted_sound && notifications.bytes_notification.sound.ne(&Sound::None) {
                // emit sound
                play_sound(notifications.bytes_notification.sound, notifications.volume);
                already_emitted_sound = true;
            }
        }
    }
    // from favorites
    if notifications.favorite_notification.notify_on_favorite
        && runtime_data.favorite_featured_last_interval
    {
        //log this notification
        // ...
        if !already_emitted_sound && notifications.favorite_notification.sound.ne(&Sound::None) {
            // emit sound
            play_sound(
                notifications.favorite_notification.sound,
                notifications.volume,
            );
        }
    }
}
