//! Module containing the entry point of application execution.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::{panic, process, thread};

#[cfg(target_os = "linux")]
use iced::window::PlatformSpecific;
use iced::{window, Application, Font, Settings};

use chart::types::chart_type::ChartType;
use chart::types::traffic_chart::TrafficChart;
use cli::parse_cli_args;
use configs::types::config_device::ConfigDevice;
use configs::types::config_settings::ConfigSettings;
use gui::pages::types::running_page::RunningPage;
use gui::styles::style_constants::FONT_SIZE_BODY;
use gui::styles::types::style_type::StyleType;
use gui::types::runtime_data::RunTimeData;
use gui::types::sniffer::Sniffer;
use networking::types::app_protocol::AppProtocol;
use networking::types::byte_multiple::ByteMultiple;
use networking::types::info_traffic::InfoTraffic;
use networking::types::ip_version::IpVersion;
use networking::types::protocol::Protocol;
use report::types::report_sort_type::ReportSortType;
use translations::types::language::Language;
use utils::formatted_strings::print_cli_welcome_message;

use crate::configs::types::config_window::{ConfigWindow, ToPosition};
use crate::configs::types::configs::Configs;
use crate::secondary_threads::check_updates::set_newer_release_status;

mod chart;
mod cli;
mod configs;
mod countries;
mod gui;
mod mmdb;
mod networking;
mod notifications;
mod report;
mod secondary_threads;
mod translations;
mod utils;

pub const SNIFFNET_LOWERCASE: &str = "sniffnet";
pub const SNIFFNET_TITLECASE: &str = "Sniffnet";

/// Entry point of application execution
///
/// It initializes shared variables and loads configuration parameters
pub fn main() -> iced::Result {
    parse_cli_args();

    let configs1 = Arc::new(Mutex::new(Configs::load()));
    let configs2 = configs1.clone();

    let newer_release_available1 = Arc::new(Mutex::new(None));
    let newer_release_available2 = newer_release_available1.clone();

    // kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    // gracefully close the app when receiving SIGINT, SIGTERM, or SIGHUP
    ctrlc::set_handler(move || {
        configs2.lock().unwrap().clone().store();
        process::exit(130);
    })
    .expect("Error setting Ctrl-C handler");

    thread::Builder::new()
        .name("thread_check_updates".to_string())
        .spawn(move || {
            set_newer_release_status(&newer_release_available2);
        })
        .unwrap();

    print_cli_welcome_message();

    let ConfigWindow { size, position } = configs1.lock().unwrap().window;

    Sniffer::run(Settings {
        // id needed for Linux Wayland; should match StartupWMClass in .desktop file; see issue #292
        id: Some(String::from(SNIFFNET_LOWERCASE)),
        window: window::Settings {
            size, // start size
            position: position.to_position(),
            min_size: Some((800, 500)), // min size allowed
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: None,
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: String::from(SNIFFNET_LOWERCASE),
            },
            ..Default::default()
        },
        flags: Sniffer::new(&configs1, newer_release_available1),
        default_font: Font::with_name("Sarasa Mono SC"),
        default_text_size: FONT_SIZE_BODY,
        antialiasing: false,
        exit_on_close_request: false,
    })
}
