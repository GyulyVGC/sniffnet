//! Module containing the entry point of application execution.

use std::{panic, process, thread};
use std::sync::{Arc, Condvar, Mutex};

use iced::{Application, button, pick_list, scrollable, Settings, window};
use pcap::Device;

use gui::style::{FONT_SIZE_BODY, Mode};

use crate::structs::info_address_port_pair::{AppProtocol, TransProtocol};
use crate::structs::info_traffic::InfoTraffic;
use crate::structs::runtime_data::RunTimeData;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_write_report::sleep_and_write_report_loop;

mod thread_parse_packets;
mod thread_write_report;
mod gui;
mod structs;
mod utility;

/// Possible filters applicable to network traffic
pub struct Filters {
    ip: String,
    transport: TransProtocol,
    application: AppProtocol,
}

/// Struct on which the gui is based
///
/// It contains gui statuses and network traffic statistics to be shared among the different threads
pub struct Sniffer {
    current_capture_id: Arc<Mutex<u16>>,
    info_traffic: Arc<Mutex<InfoTraffic>>,
    runtime_data: Arc<Mutex<RunTimeData>>,
    device: Arc<Mutex<Device>>,
    filters: Arc<Mutex<Filters>>,
    status_pair: Arc<(Mutex<Status>, Condvar)>,
    start: button::State,
    reset: button::State,
    mode: button::State,
    report: button::State,
    git: button::State,
    app: pick_list::State<AppProtocol>,
    scroll_adapters: scrollable::State,
    scroll_packets: scrollable::State,
    scroll_report: scrollable::State,
    style: Mode,
    waiting: String,
    traffic_chart: TrafficChart,
    chart_packets: bool,
    report_type: String,
}


/// This enum represents the sniffing process status.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Status {
    /// Sniffnet has just been launched/restarted and gui is in the main screen.
    Init,
    /// The sniffing process is running: the application parses packets and periodically update the output report.
    Running,
}

/// Entry point of application execution
///
/// It initialized shared variables and gui parameters
pub fn main() -> iced::Result {
    let current_capture_id1 = Arc::new(Mutex::new(0));
    let current_capture_id2 = current_capture_id1.clone();

    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2 = mutex_map1.clone();

    let runtime_data1 = Arc::new(Mutex::new(RunTimeData::new()));
    let runtime_data2 = runtime_data1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Init), Condvar::new()));
    let status_pair2 = status_pair1.clone();

    let found_device = Arc::new(Mutex::new(Device::lookup().unwrap().unwrap()));

    let filters = Arc::new(Mutex::new(Filters {
        ip: "no filter".to_string(),
        transport: TransProtocol::Other,
        application: AppProtocol::Other,
    }));

    // to kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    thread::Builder::new().name("thread_write_report".to_string()).spawn(move || {
        sleep_and_write_report_loop(current_capture_id2,
                                    mutex_map2, status_pair2);
    }).unwrap();

    Sniffer::run(Settings {
        id: None,
        window: window::Settings {
            size: (1190, 715), // start size
            position: Default::default(),
            min_size: Some((1190, 715)), // min size allowed
            max_size: None,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
        },
        flags: Sniffer {
            current_capture_id: current_capture_id1,
            info_traffic: mutex_map1,
            runtime_data: runtime_data1,
            device: found_device,
            filters,
            status_pair: status_pair1,
            start: button::State::new(),
            reset: button::State::new(),
            mode: button::State::new(),
            report: button::State::new(),
            git: button::State::new(),
            app: pick_list::State::new(),
            scroll_adapters: scrollable::State::new(),
            scroll_packets: scrollable::State::new(),
            scroll_report: scrollable::State::new(),
            style: Mode::Night,
            waiting: String::new(),
            traffic_chart: TrafficChart::new(runtime_data2),
            chart_packets: true,
            report_type: "latest".to_string(),
        },
        default_font: None,
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: false,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}