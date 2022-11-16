//! Module defining the `Sniffer` struct, which trace gui's component statuses and permits
//! to share data among the different threads.


use std::sync::{Arc, Condvar, Mutex};
use iced::{button, pick_list, scrollable};
use pcap::Device;
use crate::{AppProtocol, InfoTraffic, StyleType, RunTimeData, TrafficChart};
use crate::enums::chart_type::ChartType;
use crate::enums::report_type::ReportType;
use crate::enums::status::Status;
use crate::structs::filters::Filters;

/// Struct on which the gui is based
///
/// It contains gui statuses and network traffic statistics to be shared among the different threads
pub struct Sniffer {
    /// Capture number, incremented at every new run
    pub current_capture_id: Arc<Mutex<u16>>,
    /// Capture data updated by thread parsing packets
    pub info_traffic: Arc<Mutex<InfoTraffic>>,
    /// Capture data displayed in GUI
    pub runtime_data: Arc<Mutex<RunTimeData>>,
    /// Network adapter to be analyzed
    pub device: Arc<Mutex<Device>>,
    /// Active filters on the observed traffic
    pub filters: Arc<Mutex<Filters>>,
    /// Status of the application (init or running) and the associated condition variable
    pub status_pair: Arc<(Mutex<Status>, Condvar)>,
    /// Start button state
    pub start: button::State,
    /// Reset button state
    pub reset: button::State,
    /// Style button state
    pub mode: button::State,
    /// Full report button state
    pub report: button::State,
    /// GitHub button state
    pub git: button::State,
    /// Application protocol picklist state
    pub app: pick_list::State<AppProtocol>,
    /// Adapters scrollbar state
    pub scroll_adapters: scrollable::State,
    /// Packets information scrollbar state
    pub scroll_packets: scrollable::State,
    /// Relevant connections scrollbar state
    pub scroll_report: scrollable::State,
    /// Application style (only values Day and Night are possible for this field)
    pub style: StyleType,
    /// Waiting string
    pub waiting: String,
    /// Chart displayed
    pub traffic_chart: TrafficChart,
    /// Chart type to be displayed
    pub chart_type: ChartType,
    /// Report type to be displayed
    pub report_type: ReportType,
}