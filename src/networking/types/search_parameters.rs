use crate::AppProtocol;

/// Used to express the search filters applied to GUI inspect page
#[derive(Clone, Debug, Default)]
pub struct SearchParameters {
    /// Application protocol
    pub app: Option<AppProtocol>,
}