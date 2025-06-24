use clap::Parser;
use iced::{Task, window};

use crate::CONFIGS;
use crate::Configs;
use crate::SNIFFNET_LOWERCASE;
use crate::gui::types::message::Message;
use crate::utils::formatted_strings::APP_VERSION;

#[derive(Parser, Debug)]
#[command(
    name = SNIFFNET_LOWERCASE,
    bin_name = SNIFFNET_LOWERCASE,
    version = APP_VERSION,
    about = "Application to comfortably monitor your network traffic"
)]
struct Args {
    /// Start sniffing packets from the supplied network adapter
    #[arg(short, long, value_name = "NAME", default_missing_value = CONFIGS.device.device_name.as_str(), num_args = 0..=1)]
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
        if Configs::default().store().is_ok() {
            println!("Restored default settings");
        }
        std::process::exit(0);
    }

    let mut boot_task_chain = window::get_latest()
        .map(Message::StartApp)
        .chain(Task::done(Message::Periodic));
    if let Some(adapter) = args.adapter {
        boot_task_chain = boot_task_chain
            .chain(Task::done(Message::DeviceSelection(adapter)))
            .chain(Task::done(Message::Start));
    }

    boot_task_chain
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::configs::types::config_window::{PositionTuple, SizeTuple};
    use crate::gui::styles::types::custom_palette::ExtraStyles;
    use crate::gui::styles::types::gradient_type::GradientType;
    use crate::notifications::types::notifications::Notifications;
    use crate::{ConfigDevice, ConfigSettings, ConfigWindow, Language, Sniffer, StyleType};

    use super::*;

    #[test]
    #[serial]
    fn test_restore_default_configs() {
        // initial configs stored are the default ones
        assert_eq!(Configs::load(), Configs::default());
        let modified_configs = Configs {
            settings: ConfigSettings {
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
        };
        // we want to be sure that modified config is different from defaults
        assert_ne!(Configs::default(), modified_configs);
        //store modified configs
        modified_configs.clone().store().unwrap();
        // assert they've been stored
        assert_eq!(Configs::load(), modified_configs);
        // restore defaults
        Configs::default().store().unwrap();
        // assert that defaults are stored
        assert_eq!(Configs::load(), Configs::default());

        // only needed because it will delete config files via its Drop implementation
        Sniffer::new(Configs::default());
    }
}
