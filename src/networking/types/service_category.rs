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
    /// Web content, generic web APIs, and web transfer protocols.
    Web,
    /// Email delivery, access, notification, and mail-specific routing.
    Email,
    /// Human messaging, chat, discussion/news, SMS and paging.
    Chat,
    /// Audio/video delivery, calls, conferencing, and audiovisual sessions.
    Media,
    /// File/object transfer, synchronization, sharing, and network filesystems.
    Files,
    /// Storage infrastructure, block/tape access, backup, restore, and protected replication.
    Storage,
    /// Database/query engines, data caches, database access and replication.
    Database,
    /// Interactive computer/desktop/application access and remote execution.
    Remote,
    /// IT monitoring, logs, diagnostics, configuration, deployment, availability, and resource management.
    Management,
    /// Name resolution and locating network devices/services.
    Discovery,
    /// Address assignment, network boot, time synchronization, routing, switching, mobility, and telecom/network control.
    Network,
    /// General connection tunneling, network relays, VPN negotiation, and proxies.
    Vpn,
    /// Authentication, authorization, directories, identity lookup, and credential/key/certificate services.
    Identity,
    /// Printing, document scanning, fax, and their dedicated peripheral protocols.
    Printing,
    /// Game sessions, multiplayer transports, game lobbies, and game-specific communication.
    Gaming,
    /// Physical process/equipment control, building/home automation, industrial telemetry and metering.
    Industrial,
    /// Source control, compilation/build/test tools, debugging, and development support.
    Development,
    /// General application messaging, RPC, distributed objects, transaction coordination and shared application infrastructure.
    Middleware,
    /// Threat/vulnerability detection, prevention, integrity checking, security policy enforcement and incident exchange.
    Security,
    /// Software license distribution, entitlement verification, activation, and floating-license services.
    Licensing,
    /// Fallback for specialized applications outside this taxonomy, ambiguous identities, or insufficiently documented purposes.
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
