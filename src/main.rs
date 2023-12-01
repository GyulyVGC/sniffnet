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
use gui::styles::types::palette::get_colors;
use gui::styles::types::style_type::StyleType;
use gui::types::runtime_data::RunTimeData;
use gui::types::sniffer::Sniffer;
use networking::types::app_protocol::AppProtocol;
use networking::types::byte_multiple::ByteMultiple;
use networking::types::info_traffic::InfoTraffic;
use networking::types::ip_version::IpVersion;
use networking::types::trans_protocol::TransProtocol;
use report::types::report_sort_type::ReportSortType;
use translations::types::language::Language;
use utils::formatted_strings::print_cli_welcome_message;

use crate::configs::types::config_advanced_settings::ConfigAdvancedSettings;
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

/// Entry point of application execution
///
/// It initializes shared variables and loads configuration parameters
pub fn main() -> iced::Result {
    parse_cli_args();

    let mutex_map1 = Arc::new(Mutex::new(InfoTraffic::new()));
    // let mutex_map2 = mutex_map1.clone();

    let newer_release_available1 = Arc::new(Mutex::new(Err(String::new())));
    let newer_release_available2 = newer_release_available1.clone();

    // to kill the main thread as soon as a secondary thread panics
    let orig_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // invoke the default handler and exit the process
        orig_hook(panic_info);
        process::exit(1);
    }));

    let configs = Configs::load();

    thread::Builder::new()
        .name("thread_check_updates".to_string())
        .spawn(move || {
            set_newer_release_status(&newer_release_available2);
        })
        .unwrap();

    print_cli_welcome_message();

    Sniffer::run(Settings {
        // id needed for Linux Wayland; should match StartupWMClass in .desktop file; see issue #292
        id: Some("sniffnet".to_string()),
        window: window::Settings {
            size: configs.window.size, // start size
            position: configs.window.position.to_position(),
            min_size: Some((800, 500)), // min size allowed
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: None,
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: "sniffnet".to_string(),
            },
            ..Default::default()
        },
        flags: Sniffer::new(mutex_map1, &configs, newer_release_available1),
        default_font: Font::with_name("Sarasa Mono SC"),
        default_text_size: FONT_SIZE_BODY,
        antialiasing: false,
        exit_on_close_request: false,
    })
}
