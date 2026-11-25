use crate::SNIFFNET_LOWERCASE;
use crate::gui::sniffer::Sniffer;
use crate::gui::types::conf::{CONF, Conf};
use crate::gui::types::message::Message;
use crate::networking::types::capture_context::{CaptureSource, CaptureSourcePicklist};
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
pub(crate) struct Args {
    /// Start sniffing packets from the supplied network adapter
    #[arg(short, long, value_name = "NAME", default_missing_value = CONF.device.device_name.as_str(), num_args = 0..=1)]
    adapter: Option<String>,
    /// Read a live pcap stream from standard input
    #[arg(long, conflicts_with = "adapter")]
    stdin: bool,
    /// Print the path to the configuration file
    #[arg(short, long, exclusive = true)]
    config_path: bool,
    #[cfg(all(windows, not(debug_assertions)))]
    /// Show the logs (stdout and stderr) of the most recent application run
    #[arg(short, long, exclusive = true)]
    logs: bool,
    /// Restore default settings
    #[arg(short, long, exclusive = true)]
    restore_default: bool,
}

impl Args {
    /// Handle and return CLI arguments
    #[allow(clippy::print_stdout, clippy::print_stderr)]
    pub fn handle() -> Self {
        let args = Args::parse();

        #[cfg(all(windows, not(debug_assertions)))]
        if let Some(logs_file) = crate::utils::formatted_strings::get_logs_file_path() {
            use crate::utils::error_logger::{ErrorLogger, Location};
            if args.logs {
                if let Ok(mut explorer) = std::process::Command::new("explorer")
                    .arg(logs_file)
                    .spawn()
                    .log_err(crate::location!())
                {
                    let _ = explorer.wait().log_err(crate::location!());
                    std::process::exit(0);
                }
                std::process::exit(1);
            }
            // else truncate logs file
            let _ = std::fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(logs_file)
                .log_err(crate::location!());
        }

        if args.restore_default {
            if Conf::default().store().is_ok() {
                println!("Restored default settings");
                std::process::exit(0);
            }
            eprintln!("Could not restore default settings");
            std::process::exit(1);
        }

        if args.config_path {
            if let Ok(config_path) =
                confy::get_configuration_file_path(SNIFFNET_LOWERCASE, Conf::FILE_NAME)
            {
                println!("{}", config_path.display());
                std::process::exit(0);
            }
            eprintln!("Could not retrieve configuration file path");
            std::process::exit(1);
        }

        args
    }

    pub fn boot(&self, conf: Conf) -> (Sniffer, Task<Message>) {
        let mut sniffer = Sniffer::new(conf);
        let show_previews = !self.stdin && self.adapter.is_none();
        let mut boot_task_chain =
            window::latest().map(move |id| Message::StartApp(id, show_previews));
        if self.stdin {
            sniffer.capture_source = CaptureSource::Stdin(None);
        } else if let Some(adapter) = self.adapter.clone() {
            boot_task_chain = boot_task_chain
                .chain(Task::done(Message::SetCaptureSource(
                    CaptureSourcePicklist::Device,
                )))
                .chain(Task::done(Message::DeviceSelection(adapter)));
        }
        boot_task_chain = boot_task_chain
            .chain(sniffer.load_ip_blacklist(sniffer.conf.settings.ip_blacklist.clone()));
        if !show_previews {
            boot_task_chain = boot_task_chain.chain(Task::done(Message::Start));
        }

        (sniffer, boot_task_chain)
    }
}

#[cfg(test)]
mod tests {
    use serial_test::serial;

    use crate::gui::pages::types::running_page::RunningPage;
    use crate::gui::pages::types::settings_page::SettingsPage;
    use crate::gui::styles::types::gradient_type::GradientType;
    use crate::gui::types::conf::Conf;
    use crate::gui::types::config_updates::ConfigUpdates;
    use crate::gui::types::config_window::ConfigWindow;
    use crate::gui::types::export_pcap::ExportPcap;
    use crate::gui::types::favorite::{FavoriteKey, Favorites};
    use crate::gui::types::filters::Filters;
    use crate::gui::types::settings::Settings;
    use crate::networking::types::capture_context::CaptureSourcePicklist;
    use crate::networking::types::config_device::ConfigDevice;
    use crate::networking::types::data_representation::DataRepr;
    use crate::networking::types::service::Service;
    use crate::notifications::types::notifications::Notifications;
    use crate::report::types::sort_type::SortType;
    use crate::{Language, Sniffer, StyleType};

    #[test]
    #[serial]
    fn stdin_is_an_explicit_alternative_to_adapter() {
        use super::Args;
        use clap::{Parser, error::ErrorKind};

        assert!(Args::try_parse_from(["sniffnet", "--stdin"]).unwrap().stdin);
        assert!(!Args::try_parse_from(["sniffnet"]).unwrap().stdin);
        let err = Args::try_parse_from(["sniffnet", "--stdin", "--adapter", "eth0"]).unwrap_err();
        assert_eq!(err.kind(), ErrorKind::ArgumentConflict);

        // Parsing the adapter default loads the test configuration.
        Sniffer::new(Conf::default());
    }

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
                expanded_view: true,
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
                    remote_notifications: Default::default(),
                    ip_blacklist_notification: Default::default(),
                },
                style: StyleType::DraculaDark,
                ip_blacklist: "some-path".to_string(),
            },
            favorites: Favorites::from([FavoriteKey::Service(Service::Name("https"))]),
            device: ConfigDevice {
                device_name: "hey-hey".to_string(),
            },
            window: ConfigWindow::new((452.0, 870.0), (440.0, 99.0), (20.0, 20.0)),
            updates: ConfigUpdates::default(),
            capture_source_picklist: CaptureSourcePicklist::File,
            report_sort_type: SortType::Ascending,
            host_favorites_filter: false,
            service_favorites_filter: true,
            program_favorites_filter: false,
            host_sort_type: SortType::Descending,
            service_sort_type: SortType::Neutral,
            program_sort_type: SortType::Neutral,
            filters: Filters {
                bpf: "tcp".to_string(),
                expanded: true,
            },
            import_pcap_path: "whole_day.pcapng".to_string(),
            ipfix_socket: Default::default(),
            export_pcap: ExportPcap::default(),
            last_opened_setting: SettingsPage::General,
            last_opened_page: RunningPage::Inspect,
            data_repr: DataRepr::default(),
        };
        // we want to be sure that modified config is different from defaults
        assert_ne!(Conf::default(), modified_conf);
        //store modified configs
        modified_conf.store().unwrap();
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
