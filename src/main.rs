//! Module containing the entry point of application execution.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::borrow::Cow;

use iced::advanced::graphics::image::image_rs::ImageFormat;
#[cfg(target_os = "linux")]
use iced::window::settings::PlatformSpecific;
use iced::{Font, Pixels, Settings, application, window};

use chart::types::chart_type::ChartType;
use chart::types::traffic_chart::TrafficChart;
use cli::handle_cli_args;
use configs::types::config_device::ConfigDevice;
use configs::types::config_settings::ConfigSettings;
use gui::pages::types::running_page::RunningPage;
use gui::sniffer::Sniffer;
use gui::styles::style_constants::FONT_SIZE_BODY;
use gui::styles::types::style_type::StyleType;
use networking::types::byte_multiple::ByteMultiple;
use networking::types::info_traffic::InfoTraffic;
use networking::types::ip_version::IpVersion;
use networking::types::protocol::Protocol;
use networking::types::service::Service;
use report::types::report_sort_type::ReportSortType;
use translations::types::language::Language;
use utils::formatted_strings::print_cli_welcome_message;

use crate::configs::types::config_window::{ConfigWindow, ToPosition, ToSize};
use crate::configs::types::configs::{CONFIGS, Configs};
use crate::gui::sniffer::FONT_FAMILY_NAME;
use crate::gui::styles::style_constants::{ICONS_BYTES, SARASA_MONO_BOLD_BYTES, SARASA_MONO_BYTES};

mod chart;
mod cli;
mod configs;
mod countries;
mod gui;
mod mmdb;
mod networking;
mod notifications;
mod report;
mod translations;
mod utils;

pub const SNIFFNET_LOWERCASE: &str = "sniffnet";
pub const SNIFFNET_TITLECASE: &str = "Sniffnet";

const WINDOW_ICON: &[u8] = include_bytes!("../resources/logos/raw/icon.png");

/// Entry point of application execution
///
/// It initializes variables and loads configuration parameters
pub fn main() -> iced::Result {
    #[cfg(all(windows, not(debug_assertions)))]
    let _gag1: gag::Redirect<std::fs::File>;
    #[cfg(all(windows, not(debug_assertions)))]
    let _gag2: gag::Redirect<std::fs::File>;
    #[cfg(all(windows, not(debug_assertions)))]
    if let Some((gag1, gag2)) = utils::formatted_strings::redirect_stdout_stderr_to_file() {
        _gag1 = gag1;
        _gag2 = gag2;
    }

    let configs = CONFIGS.clone();
    let boot_task_chain = handle_cli_args();

    #[cfg(debug_assertions)]
    {
        // kill the main thread as soon as a secondary thread panics
        let orig_hook = std::panic::take_hook();
        std::panic::set_hook(Box::new(move |panic_info| {
            // invoke the default handler and exit the process
            orig_hook(panic_info);
            std::process::exit(1);
        }));
    }

    print_cli_welcome_message();

    let ConfigWindow { size, position, .. } = configs.window;

    application(SNIFFNET_TITLECASE, Sniffer::update, Sniffer::view)
        .settings(Settings {
            // id needed for Linux Wayland; should match StartupWMClass in .desktop file; see issue #292
            id: Some(String::from(SNIFFNET_LOWERCASE)),
            fonts: vec![
                Cow::Borrowed(SARASA_MONO_BYTES),
                Cow::Borrowed(SARASA_MONO_BOLD_BYTES),
                Cow::Borrowed(ICONS_BYTES),
            ],
            default_font: Font::with_name(FONT_FAMILY_NAME),
            default_text_size: Pixels(FONT_SIZE_BODY),
            antialiasing: true,
        })
        .window(window::Settings {
            size: size.to_size(), // start size
            position: position.to_position(),
            min_size: None, // Some(ConfigWindow::MIN_SIZE.to_size()), // min size allowed
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            icon: window::icon::from_file_data(WINDOW_ICON, Some(ImageFormat::Png)).ok(),
            #[cfg(target_os = "linux")]
            platform_specific: PlatformSpecific {
                application_id: String::from(SNIFFNET_LOWERCASE),
                ..PlatformSpecific::default()
            },
            exit_on_close_request: false,
            ..Default::default()
        })
        .subscription(Sniffer::subscription)
        .theme(Sniffer::theme)
        .scale_factor(Sniffer::scale_factor)
        .run_with(move || (Sniffer::new(configs), boot_task_chain))
}
