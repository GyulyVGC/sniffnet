//! Module containing the entry point of application execution.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::{panic, process, thread};

use iced::window::Position;
use iced::{window, Application, Settings};
use pcap::Device;

use utility::style_constants::FONT_SIZE_BODY;

use crate::enums::app_protocol::AppProtocol;
use crate::enums::chart_type::ChartType;
use crate::enums::ip_version::IpVersion;
use crate::enums::language::Language;
use crate::enums::report_type::ReportType;
use crate::enums::running_page::RunningPage;
use crate::enums::status::Status;
use crate::enums::style_type::StyleType;
use crate::enums::trans_protocol::TransProtocol;
use crate::structs::colors::get_colors;
use crate::structs::config::Config;
use crate::structs::filters::Filters;
use crate::structs::info_traffic::InfoTraffic;
use crate::structs::notifications::Notifications;
use crate::structs::runtime_data::RunTimeData;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_write_report::sleep_and_write_report_loop;

mod enums;
mod gui;
mod structs;
mod thread_parse_packets;
mod thread_write_report;
mod utility;

/// Entry point of application execution
///
/// It initialized shared variables and gui parameters
pub fn main() -> iced::Result {
    let current_capture_id1 = Arc::new(Mutex::new(0));
    let current_capture_id2 = current_capture_id1.clone();

    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2 = mutex_map1.clone();

    //shared tuple containing the application status and the relative condition variable
    let status_pair1 = Arc::new((Mutex::new(Status::Init), Condvar::new()));
    let status_pair2 = status_pair1.clone();

    let found_device = Device::lookup().unwrap().unwrap();

    let pcap_error = None; // None means no error

    let runtime_data1 = Rc::new(RefCell::new(RunTimeData::new()));
    let runtime_data2 = runtime_data1.clone();

    let filters = Filters {
        ip: IpVersion::Other,
        transport: TransProtocol::Other,
        application: AppProtocol::Other,
    };

    // to kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    thread::Builder::new()
        .name("thread_write_report".to_string())
        .spawn(move || {
            sleep_and_write_report_loop(&current_capture_id2, &mutex_map2, &status_pair2);
        })
        .unwrap();

    let config_result = confy::load::<Config>("sniffnet", None);
    if config_result.is_err() {
        let store = Config {
            style: StyleType::default(),
            language: Language::default(),
            notifications: Notifications::default(),
        };
        confy::store("sniffnet", None, store).unwrap();
    }
    let config = config_result.unwrap();
    let style = config.style;
    let notifications = config.notifications;
    let language = config.language;

    Sniffer::run(Settings {
        id: None,
        window: window::Settings {
            size: (1190, 670), // start size
            position: Position::Centered,
            min_size: Some((1190, 600)), // min size allowed
            max_size: None,
            visible: true,
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
            style,
            waiting: ".".to_string(),
            traffic_chart: TrafficChart::new(runtime_data2, style, language),
            report_type: ReportType::MostRecent,
            overlay: None,
            notifications,
            running_page: RunningPage::Overview,
            language,
        },
        default_font: Some(include_bytes!("../fonts/inconsolata-regular.ttf")),
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: true,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}
