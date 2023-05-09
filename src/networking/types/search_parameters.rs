/// Used to express the search filters applied to GUI inspect page
#[derive(Clone, Debug, Default, Hash)]
pub struct SearchParameters {
    /// Application protocol
    pub app: String,
    /// Domain
    pub domain: String,
    /// Country
    pub country: String,
    /// Autonomous System name
    pub as_name: String,
    /// Whether to display only favorites
    pub only_favorites: bool,
}

pub enum FilterInputType {
    App,
    Domain,
    Country,
    AS,
}
