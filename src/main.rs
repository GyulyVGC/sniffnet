//! Module containing the entry point of application execution.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Condvar, Mutex};
use std::{panic, process, thread};

use iced::window::{PlatformSpecific, Position};
use iced::{window, Application, Settings};

use chart::types::chart_type::ChartType;
use chart::types::traffic_chart::TrafficChart;
use configs::types::config_device::ConfigDevice;
use configs::types::config_settings::ConfigSettings;
use gui::pages::types::running_page::RunningPage;
use gui::styles::style_constants::FONT_SIZE_BODY;
use gui::styles::types::palette::get_colors;
use gui::styles::types::style_type::StyleType;
use gui::types::runtime_data::RunTimeData;
use gui::types::sniffer::Sniffer;
use gui::types::status::Status;
use networking::types::app_protocol::AppProtocol;
use networking::types::byte_multiple::ByteMultiple;
use networking::types::info_traffic::InfoTraffic;
use networking::types::ip_version::IpVersion;
use networking::types::trans_protocol::TransProtocol;
use report::types::report_sort_type::ReportSortType;
use secondary_threads::write_report_file::sleep_and_write_report_loop;
use translations::types::language::Language;
use utils::formatted_strings::print_cli_welcome_message;

use crate::secondary_threads::check_updates::set_newer_release_status;

mod chart;
mod configs;
mod countries;
mod gui;
mod networking;
mod notifications;
mod report;
mod secondary_threads;
mod translations;
mod utils;

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

    let newer_release_available1 = Arc::new(Mutex::new(Err(String::new())));
    let newer_release_available2 = newer_release_available1.clone();

    // to kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let config_settings = if let Ok(setting) = confy::load::<ConfigSettings>("sniffnet", "settings")
    {
        setting
    } else {
        confy::store("sniffnet", "settings", ConfigSettings::default()).unwrap_or(());
        ConfigSettings::default()
    };

    let config_device = if let Ok(device) = confy::load::<ConfigDevice>("sniffnet", "device") {
        device
    } else {
        confy::store("sniffnet", "device", ConfigDevice::default()).unwrap_or(());
        ConfigDevice::default()
    };

    thread::Builder::new()
        .name("thread_check_updates".to_string())
        .spawn(move || {
            set_newer_release_status(&newer_release_available2);
        })
        .unwrap();

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
            platform_specific: PlatformSpecific::default(),
        },
        flags: Sniffer::new(
            current_capture_id1,
            mutex_map1,
            status_pair1,
            &config_settings,
            &config_device,
            newer_release_available1,
        ),
        default_font: Some(include_bytes!(
            "../resources/fonts/subset/sarasa-mono-sc-regular.subset.ttf"
        )),
        default_text_size: FONT_SIZE_BODY,
        text_multithreading: true,
        antialiasing: false,
        exit_on_close_request: true,
        try_opengles_first: false,
    })
}
