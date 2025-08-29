use crate::SNIFFNET_LOWERCASE;
use crate::gui::types::conf::{CONF, Conf};
use crate::gui::types::message::Message;
use crate::networking::types::capture_context::CaptureSourcePicklist;
use crate::utils::formatted_strings::APP_VERSION;
use clap::Parser;
use iced::{Task, window};

#[derive(Parser, Debug)]
#[command(
    name = SNIFFNET_LOWERCASE,
    bin_name = SNIFFNET_LOWERCASE,
    version = APP_VERSION,
    about = "Application to comfortably monitor your network traffic"
)]
struct Args {
    /// Start sniffing packets from the supplied network adapter
    #[arg(short, long, value_name = "NAME", default_missing_value = CONF.device.device_name.as_str(), num_args = 0..=1)]
    adapter: Option<String>,
    #[cfg(all(windows, not(debug_assertions)))]
    /// Show the logs (stdout and stderr) of the most recent application run
    #[arg(short, long, exclusive = true)]
    logs: bool,
    /// Restore default settings
    #[arg(short, long, exclusive = true)]
    restore_default: bool,
}

pub fn handle_cli_args() -> Task<Message> {
    let args = Args::parse();

    #[cfg(all(windows, not(debug_assertions)))]
    if let Some(logs_file) = crate::utils::formatted_strings::get_logs_file_path() {
        if args.logs {
            std::process::Command::new("explorer")
                .arg(logs_file)
                .spawn()
                .unwrap()
                .wait()
                .unwrap_or_default();
            std::process::exit(0);
        } else {
            // truncate logs file
            let _ = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(logs_file);
        }
    }

    if args.restore_default {
        if Conf::default().store().is_ok() {
            println!("Restored default settings");
        }
        std::process::exit(0);
    }

    let mut boot_task_chain = window::get_latest()
        .map(Message::StartApp)
        .chain(Task::done(Message::Periodic));
    if let Some(adapter) = args.adapter {
        // TODO: check if this works once #653 is fixed
        // currently the link type and device name aren't displayed properly when starting from CLI
        boot_task_chain = boot_task_chain
            .chain(Task::done(Message::SetCaptureSource(
                CaptureSourcePicklist::Device,
            )))
            .chain(Task::done(Message::DeviceSelection(adapter)))
            .chain(Task::done(Message::Start));
    }

    boot_task_chain
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::gui::pages::types::running_page::RunningPage;
    use crate::gui::pages::types::settings_page::SettingsPage;
    use crate::gui::styles::types::custom_palette::ExtraStyles;
    use crate::gui::styles::types::gradient_type::GradientType;
    use crate::gui::types::conf::Conf;
    use crate::gui::types::config_window::{PositionTuple, SizeTuple};
    use crate::gui::types::export_pcap::ExportPcap;
    use crate::gui::types::filters::Filters;
    use crate::gui::types::settings::Settings;
    use crate::networking::types::capture_context::CaptureSourcePicklist;
    use crate::networking::types::config_device::ConfigDevice;
    use crate::notifications::types::notifications::Notifications;
    use crate::report::types::sort_type::SortType;
    use crate::{ConfigWindow, Language, Sniffer, StyleType};

    #[test]
    #[serial]
    fn test_restore_default_configs() {
        // initial configs stored are the default ones
        assert_eq!(Conf::load(), Conf::default());
        let modified_conf = Conf {
            settings: Settings {
                color_gradient: GradientType::Wild,
                language: Language::ZH,
                scale_factor: 0.65,
                mmdb_country: "countrymmdb".to_string(),
                mmdb_asn: "asnmmdb".to_string(),
                style_path: format!(
                    "{}/resources/themes/catppuccin.toml",
                    env!("CARGO_MANIFEST_DIR")
                ),
                notifications: Notifications {
                    volume: 100,
                    data_notification: Default::default(),
                    favorite_notification: Default::default(),
                },
                style: StyleType::Custom(ExtraStyles::DraculaDark),
            },
            device: ConfigDevice {
                device_name: "hey-hey".to_string(),
            },
            window: ConfigWindow {
                position: PositionTuple(440.0, 99.0),
                size: SizeTuple(452.0, 870.0),
                thumbnail_position: PositionTuple(20.0, 20.0),
            },
            capture_source_picklist: CaptureSourcePicklist::File,
            report_sort_type: SortType::Ascending,
            host_sort_type: SortType::Descending,
            service_sort_type: SortType::Neutral,
            filters: Filters {
                bpf: "tcp".to_string(),
                expanded: true,
            },
            import_pcap_path: "whole_day.pcapng".to_string(),
            export_pcap: ExportPcap {
                enabled: true,
                file_name: "sniffnet.pcap".to_string(),
                directory: "home".to_string(),
            },
            last_opened_setting: SettingsPage::General,
            last_opened_page: RunningPage::Inspect,
        };
        // we want to be sure that modified config is different from defaults
        assert_ne!(Conf::default(), modified_conf);
        //store modified configs
        modified_conf.clone().store().unwrap();
        // assert they've been stored
        assert_eq!(Conf::load(), modified_conf);
        // restore defaults
        Conf::default().store().unwrap();
        // assert that defaults are stored
        assert_eq!(Conf::load(), Conf::default());

        // only needed because it will delete config files via its Drop implementation
        Sniffer::new(Conf::default());
    }
}
