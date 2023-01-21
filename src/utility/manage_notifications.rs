use crate::structs::notifications::Notifications;
use crate::utility::sounds::play_sound;
use crate::RunTimeData;
use std::cell::Ref;

pub fn notify(runtime_data: Ref<RunTimeData>, notifications: Notifications) {
    if notifications.packets_threshold.is_some() {
        let sent_packets_entry = runtime_data.tot_sent_packets_prev - runtime_data.tot_sent_packets;
        let received_packets_entry =
            runtime_data.tot_received_packets - runtime_data.tot_received_packets_prev;
        if received_packets_entry - sent_packets_entry
            > notifications.packets_threshold.unwrap() as i128
        {
            play_sound();
        }
    }
}
