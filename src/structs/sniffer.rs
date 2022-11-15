//! Module defining the `Sniffer` struct, which trace gui's component statuses and permits
//! to share data among the different threads.


use std::sync::{Arc, Condvar, Mutex};
use iced::{button, pick_list, scrollable};
use pcap::Device;
use crate::{AppProtocol, InfoTraffic, Mode, RunTimeData, Status, TrafficChart};
use crate::structs::filters::Filters;

/// Struct on which the gui is based
///
/// It contains gui statuses and network traffic statistics to be shared among the different threads
pub struct Sniffer {
    pub current_capture_id: Arc<Mutex<u16>>,
    pub info_traffic: Arc<Mutex<InfoTraffic>>,
    pub runtime_data: Arc<Mutex<RunTimeData>>,
    pub device: Arc<Mutex<Device>>,
    pub filters: Arc<Mutex<Filters>>,
    pub status_pair: Arc<(Mutex<Status>, Condvar)>,
    pub start: button::State,
    pub reset: button::State,
    pub mode: button::State,
    pub report: button::State,
    pub git: button::State,
    pub app: pick_list::State<AppProtocol>,
    pub scroll_adapters: scrollable::State,
    pub scroll_packets: scrollable::State,
    pub scroll_report: scrollable::State,
    pub style: Mode,
    pub waiting: String,
    pub traffic_chart: TrafficChart,
    pub chart_packets: bool,
    pub report_type: String,
}