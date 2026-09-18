use crate::countries::flags_pictures::{ICONS_SIZE_BIG, ICONS_SIZE_SMALL};
use crate::gui::styles::container::ContainerType;
use crate::gui::styles::style_constants::{FONT_SIZE_FOOTER, TOOLTIP_DELAY};
use crate::gui::styles::types::style_type::StyleType;
use crate::gui::types::message::Message;
use iced::advanced::svg::Handle;
use iced::widget::tooltip::Position;
use iced::widget::{Svg, Text, Tooltip};
use std::fmt::Display;

const CHAT: &[u8] = include_bytes!("../../../resources/embedded_icons/service_categories/chat.svg");
const DATABASE: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/database.svg");
const DEVELOPMENT: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/development.svg");
const DISCOVERY: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/discovery.svg");
const EMAIL: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/email.svg");
const FILES: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/files.svg");
const GAMING: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/gaming.svg");
const IDENTITY: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/identity.svg");
const INDUSTRIAL: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/industrial.svg");
const LICENSING: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/licensing.svg");
const MANAGEMENT: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/management.svg");
const MEDIA: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/media.svg");
const MIDDLEWARE: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/middleware.svg");
const NETWORK: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/network.svg");
const PRINTING: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/printing.svg");
const REMOTE: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/remote.svg");
const SECURITY: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/security.svg");
const STORAGE: &[u8] =
    include_bytes!("../../../resources/embedded_icons/service_categories/storage.svg");
const VPN: &[u8] = include_bytes!("../../../resources/embedded_icons/service_categories/vpn.svg");
const WEB: &[u8] = include_bytes!("../../../resources/embedded_icons/service_categories/web.svg");
const OTHER: &[u8] = include_bytes!("../../../resources/embedded_icons/unknown.svg");

/// General purpose of an upper layer service.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum ServiceCategory {
    Web,
    Email,
    /// Human chat and messaging.
    Chat,
    /// Voice and audiovisual media.
    Media,
    /// File transfer and sharing.
    Files,
    /// Storage infrastructure and backup.
    Storage,
    /// Databases and caches.
    Database,
    /// Remote access and execution.
    Remote,
    /// Monitoring and administration.
    Management,
    /// Naming and service discovery.
    Discovery,
    /// Network infrastructure.
    Network,
    /// VPNs, tunnels and proxies.
    Vpn,
    /// Identity and directory services.
    Identity,
    /// Printing, scanning and fax.
    Printing,
    Gaming,
    /// Industrial and device automation.
    Industrial,
    /// Software development.
    Development,
    /// Application messaging, RPC and coordination.
    Middleware,
    /// Dedicated security tools.
    Security,
    /// Software licensing.
    Licensing,
    /// Specialized or insufficiently established purposes.
    #[default]
    Other,
}

impl Display for ServiceCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let category_str = match self {
            ServiceCategory::Web => "Web",
            ServiceCategory::Email => "Email",
            ServiceCategory::Chat => "Chat & Messaging",
            ServiceCategory::Media => "Voice & Media",
            ServiceCategory::Files => "File Transfer & Sharing",
            ServiceCategory::Storage => "Storage & Backup",
            ServiceCategory::Database => "Databases & Caches",
            ServiceCategory::Remote => "Remote Access & Execution",
            ServiceCategory::Management => "Monitoring & Administration",
            ServiceCategory::Discovery => "Naming & Service Discovery",
            ServiceCategory::Network => "Network Infrastructure",
            ServiceCategory::Vpn => "VPNs, Tunnels & Proxies",
            ServiceCategory::Identity => "Identity & Directory",
            ServiceCategory::Printing => "Printing & Scanning",
            ServiceCategory::Gaming => "Gaming",
            ServiceCategory::Industrial => "Industrial & Automation",
            ServiceCategory::Development => "Software Development",
            ServiceCategory::Middleware => "Middleware",
            ServiceCategory::Security => "Security",
            ServiceCategory::Licensing => "Software Licensing",
            ServiceCategory::Other => "",
        };
        write!(f, "{category_str}")
    }
}

impl ServiceCategory {
    pub fn get_icon_tooltip<'a>(
        self,
        thumbnail: bool,
        opacity: f32,
    ) -> Tooltip<'a, Message, StyleType> {
        let size = if thumbnail {
            ICONS_SIZE_SMALL
        } else {
            ICONS_SIZE_BIG
        };

        let tooltip = if thumbnail || self == ServiceCategory::Other {
            String::new()
        } else {
            self.to_string()
        };
        let tooltip_style = if tooltip.is_empty() {
            ContainerType::Standard
        } else {
            ContainerType::Tooltip
        };

        let svg = Svg::new(Handle::from_memory(match self {
            ServiceCategory::Web => WEB,
            ServiceCategory::Email => EMAIL,
            ServiceCategory::Chat => CHAT,
            ServiceCategory::Media => MEDIA,
            ServiceCategory::Files => FILES,
            ServiceCategory::Storage => STORAGE,
            ServiceCategory::Database => DATABASE,
            ServiceCategory::Remote => REMOTE,
            ServiceCategory::Management => MANAGEMENT,
            ServiceCategory::Discovery => DISCOVERY,
            ServiceCategory::Network => NETWORK,
            ServiceCategory::Vpn => VPN,
            ServiceCategory::Identity => IDENTITY,
            ServiceCategory::Printing => PRINTING,
            ServiceCategory::Gaming => GAMING,
            ServiceCategory::Industrial => INDUSTRIAL,
            ServiceCategory::Development => DEVELOPMENT,
            ServiceCategory::Middleware => MIDDLEWARE,
            ServiceCategory::Security => SECURITY,
            ServiceCategory::Licensing => LICENSING,
            ServiceCategory::Other => OTHER,
        }))
        .opacity(opacity)
        .width(size)
        .height(size);

        Tooltip::new(
            svg,
            Text::new(tooltip).size(FONT_SIZE_FOOTER),
            Position::FollowCursor,
        )
        .snap_within_viewport(true)
        .class(tooltip_style)
        .delay(TOOLTIP_DELAY)
    }
}
