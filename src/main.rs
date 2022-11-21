//! Module containing the entry point of application execution.

use std::{panic, process, thread};
use std::sync::{Arc, Condvar, Mutex};

use iced::{Application, button, pick_list, scrollable, Settings, window};
use iced::window::Position;
use pcap::Device;

use gui::style::{FONT_SIZE_BODY, StyleType};

use crate::enums::app_protocol::AppProtocol;
use crate::enums::chart_type::ChartType;
use crate::enums::ip_version::IpVersion;
use crate::enums::report_type::ReportType;
use crate::enums::status::Status;
use crate::enums::trans_protocol::TransProtocol;
use crate::structs::filters::Filters;
use crate::structs::info_traffic::InfoTraffic;
use crate::structs::runtime_data::RunTimeData;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_write_report::sleep_and_write_report_loop;

mod thread_parse_packets;
mod thread_write_report;
mod gui;
mod structs;
mod utility;
mod enums;


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

    let pcap_error = Arc::new(Mutex::new(None)); // None means no error

    let filters = Arc::new(Mutex::new(Filters {
        ip: IpVersion::Other,
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
            size: (1190, 670), // start size
            position: Position::Centered,
            min_size: Some((1190, 500)), // min size allowed
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
            pcap_error,
            start: button::State::new(),
            reset: button::State::new(),
            mode: button::State::new(),
            report: button::State::new(),
            git: button::State::new(),
            app: pick_list::State::new(),
            scroll_adapters: scrollable::State::new(),
            scroll_packets: scrollable::State::new(),
            scroll_report: scrollable::State::new(),
            style: StyleType::Night,
            waiting: String::new(),
            traffic_chart: TrafficChart::new(runtime_data2),
            chart_type: ChartType::Packets,
            report_type: ReportType::MostRecent,
        },
        default_font: None,
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: false,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}