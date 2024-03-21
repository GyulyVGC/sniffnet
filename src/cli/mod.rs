use crate::utils::formatted_strings::APP_VERSION;
use crate::{Configs, SNIFFNET_LOWERCASE};

/// Parse CLI arguments, and exit if `--help`, `--version`, or an
/// unknown argument was supplied
pub fn parse_cli_args() {
    let mut args = std::env::args().skip(1);
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "--help" | "-h" => print_help(),
            "--version" | "-v" => print_version(),
            "--restore-default" => restore_default(),
            _ => {
                unknown_argument(&arg);
                std::process::exit(1);
            }
        }
        std::process::exit(0);
    }
}

fn print_help() {
    println!(
        "Application to comfortably monitor your Internet traffic\n\
        Usage: {SNIFFNET_LOWERCASE} [OPTIONS]\n\
        Options:\n\
        \t-h, --help            Print help\n\
        \t--restore-default     Restore default settings\n\
        \t-v, --version         Print version info\n\
        (Run without options to start the app)"
    );
}

fn print_version() {
    println!("{SNIFFNET_LOWERCASE} {APP_VERSION}");
}

fn restore_default() {
    Configs::default().store();
    println!("Default settings have been restored");
}

fn unknown_argument(arg: &str) {
    eprintln!(
        "{SNIFFNET_LOWERCASE}: unknown option '{arg}'\n\
        For more information, try '{SNIFFNET_LOWERCASE} --help'"
    );
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use serial_test::serial;

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
                    packets_notification: Default::default(),
                    bytes_notification: Default::default(),
                    favorite_notification: Default::default(),
                },
                style: StyleType::Custom(ExtraStyles::DraculaDark),
            },
            device: ConfigDevice {
                device_name: "hey-hey".to_string(),
            },
            window: ConfigWindow {
                position: (440, 99),
                size: (452, 870),
                thumbnail_position: (20, 20),
            },
        };
        // we want to be sure that modified config is different from defaults
        assert_ne!(Configs::default(), modified_configs);
        //store modified configs
        modified_configs.clone().store();
        // assert they've been stored
        assert_eq!(Configs::load(), modified_configs);
        // restore defaults
        restore_default();
        // assert that defaults are stored
        assert_eq!(Configs::load(), Configs::default());

        // only needed because it will delete config files via its Drop implementation
        Sniffer::new(
            &Arc::new(Mutex::new(Configs::default())),
            Arc::new(Mutex::new(Some(true))),
        );
    }
}
