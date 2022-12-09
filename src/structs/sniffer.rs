//! Module defining the `Sniffer` struct, which trace gui's component statuses and permits
//! to share data among the different threads.

use std::sync::{Arc, Condvar, Mutex};

use pcap::Device;

use crate::enums::report_type::ReportType;
use crate::enums::status::Status;
use crate::structs::filters::Filters;
use crate::{InfoTraffic, RunTimeData, StyleType, TrafficChart};

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
    /// Signals if a pcap error occurred
    pub pcap_error: Arc<Mutex<Option<String>>>,
    /// Application style (only values Day and Night are possible for this field)
    pub style: StyleType,
    /// Waiting string
    pub waiting: String,
    /// Chart displayed
    pub traffic_chart: TrafficChart,
    /// Report type to be displayed
    pub report_type: ReportType,
}
