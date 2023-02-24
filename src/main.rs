//! Module containing the entry point of application execution.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Condvar, Mutex};
use std::{panic, process, thread};

use iced::window::Position;
use iced::{window, Application, Settings};

use utility::style_constants::FONT_SIZE_BODY;

use crate::enums::app_protocol::AppProtocol;
use crate::enums::byte_multiple::ByteMultiple;
use crate::enums::chart_type::ChartType;
use crate::enums::ip_version::IpVersion;
use crate::enums::language::Language;
use crate::enums::report_type::ReportType;
use crate::enums::running_page::RunningPage;
use crate::enums::status::Status;
use crate::enums::style_type::StyleType;
use crate::enums::trans_protocol::TransProtocol;
use crate::structs::configs::{ConfigDevice, ConfigSettings};
use crate::structs::info_traffic::InfoTraffic;
use crate::structs::palette::get_colors;
use crate::structs::runtime_data::RunTimeData;
use crate::structs::sniffer::Sniffer;
use crate::structs::traffic_chart::TrafficChart;
use crate::thread_write_report::sleep_and_write_report_loop;
use crate::utility::get_formatted_strings::print_cli_welcome_message;

mod enums;
mod gui;
mod structs;
mod thread_parse_packets;
mod thread_write_report;
mod utility;

/// Entry point of application execution
///
/// It initializes shared variables and loads configuration parameters
pub fn main() -> iced::Result {
    let current_capture_id1 = Arc::new(Mutex::new(0));
    let current_capture_id2 = current_capture_id1.clone();

    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    let mutex_map2 = mutex_map1.clone();

    let status_pair1 = Arc::new((Mutex::new(Status::Init), Condvar::new()));
    let status_pair2 = status_pair1.clone();

    let runtime_data = Rc::new(RefCell::new(RunTimeData::new()));

    // to kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let config_settings_result = confy::load::<ConfigSettings>("sniffnet", "settings");
    if config_settings_result.is_err() {
        // it happens when changing the ConfigSettings struct fields during development or after new releases
        confy::store("sniffnet", "settings", ConfigSettings::default()).unwrap_or(());
    }
    let config_settings = config_settings_result.unwrap_or(ConfigSettings::default());

    let config_device_result = confy::load::<ConfigDevice>("sniffnet", "device");
    if config_device_result.is_err() {
        // it happens when changing the ConfigDevice struct fields during development or after new releases
        confy::store("sniffnet", "device", ConfigDevice::default()).unwrap_or(());
    }
    let config_device = config_device_result.unwrap_or(ConfigDevice::default());

    thread::Builder::new()
        .name("thread_write_report".to_string())
        .spawn(move || {
            sleep_and_write_report_loop(&current_capture_id2, &mutex_map2, &status_pair2);
        })
        .unwrap();

    print_cli_welcome_message();

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
        flags: Sniffer::new(
            current_capture_id1,
            mutex_map1,
            runtime_data,
            status_pair1,
            &config_settings,
            &config_device,
        ),
        default_font: Some(include_bytes!("../resources/fonts/inconsolata-regular.ttf")),
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: true,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}
