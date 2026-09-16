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
