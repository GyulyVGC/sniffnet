use crate::AppProtocol;

/// Used to express the search filters applied to GUI inspect page
#[derive(Clone, Debug, Default, Hash)]
pub struct SearchParameters {
    /// Application protocol
    pub app: Option<AppProtocol>,
    /// Domain
    pub domain: Option<String>,
    /// Country
    pub country: Option<String>,
    /// Autonomous System name
    pub as_name: Option<String>,
}
