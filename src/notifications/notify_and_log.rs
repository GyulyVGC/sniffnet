use crate::InfoTraffic;
use crate::chart::types::chart_type::ChartType;
use crate::networking::types::capture_context::CaptureSource;
use crate::networking::types::data_info::DataInfo;
use crate::networking::types::data_info_host::DataInfoHost;
use crate::networking::types::host::Host;
use crate::networking::types::service::Service;
use crate::notifications::types::logged_notification::{
    DataThresholdExceeded, FavoriteTransmitted, LoggedNotification,
};
use crate::notifications::types::notifications::Notifications;
use crate::notifications::types::sound::{Sound, play};
use crate::report::types::sort_type::SortType;
use crate::utils::formatted_strings::get_formatted_timestamp;
use std::cmp::min;
use std::collections::{HashSet, VecDeque};

/// Checks if one or more notifications have to be emitted and logs them.
///
/// It returns the number of new notifications emitted
pub fn notify_and_log(
    logged_notifications: &mut (VecDeque<LoggedNotification>, usize),
    notifications: Notifications,
    info_traffic_msg: &InfoTraffic,
    favorites: &HashSet<Host>,
    cs: &CaptureSource,
) -> usize {
    let mut sound_to_play = Sound::None;
    let emitted_notifications_prev = logged_notifications.1;
    let timestamp = info_traffic_msg.last_packet_timestamp;
    let data_info = info_traffic_msg.tot_data_info;
    // data threshold
    if let Some(threshold) = notifications.data_notification.threshold {
        let chart_type = notifications.data_notification.chart_type;
        if data_info.tot_data(chart_type) > u128::from(threshold) {
            //log this notification
            logged_notifications.1 += 1;
            if logged_notifications.0.len() >= 30 {
                logged_notifications.0.pop_back();
            }
            logged_notifications
                .0
                .push_front(LoggedNotification::DataThresholdExceeded(
                    DataThresholdExceeded {
                        id: logged_notifications.1,
                        chart_type,
                        threshold: notifications.data_notification.previous_threshold,
                        data_info,
                        timestamp: get_formatted_timestamp(timestamp),
                        is_expanded: false,
                        hosts: hosts_list(info_traffic_msg, chart_type),
                        services: services_list(info_traffic_msg, chart_type),
                    },
                ));
            if sound_to_play.eq(&Sound::None) {
                sound_to_play = notifications.data_notification.sound;
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
                logged_notifications.1 += 1;
                if logged_notifications.0.len() >= 30 {
                    logged_notifications.0.pop_back();
                }

                logged_notifications
                    .0
                    .push_front(LoggedNotification::FavoriteTransmitted(
                        FavoriteTransmitted {
                            id: logged_notifications.1,
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

    logged_notifications.1 - emitted_notifications_prev
}

fn hosts_list(info_traffic_msg: &InfoTraffic, chart_type: ChartType) -> Vec<(Host, DataInfoHost)> {
    let mut hosts: Vec<(Host, DataInfoHost)> = info_traffic_msg
        .hosts
        .iter()
        .map(|(h, data)| (h.clone(), *data))
        .collect();
    hosts.sort_by(|(_, a), (_, b)| {
        a.data_info
            .compare(&b.data_info, SortType::Descending, chart_type)
    });
    let n_entry = min(hosts.len(), 4);
    hosts
        .get(..n_entry)
        .unwrap_or_default()
        .to_owned()
        .into_iter()
        .collect()
}

fn services_list(
    info_traffic_msg: &InfoTraffic,
    chart_type: ChartType,
) -> Vec<(Service, DataInfo)> {
    let mut services: Vec<(Service, DataInfo)> = info_traffic_msg
        .services
        .iter()
        .filter(|(service, _)| service != &&Service::NotApplicable)
        .map(|(s, data)| (*s, *data))
        .collect();
    services.sort_by(|(_, a), (_, b)| a.compare(b, SortType::Descending, chart_type));
    let n_entry = min(services.len(), 4);
    services
        .get(..n_entry)
        .unwrap_or_default()
        .to_owned()
        .into_iter()
        .collect()
}
