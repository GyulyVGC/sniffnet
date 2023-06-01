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

impl SearchParameters {
    pub fn is_some_filter_active(&self) -> bool {
        self.only_favorites
            || !self.app.is_empty()
            || !self.domain.is_empty()
            || !self.country.is_empty()
            || !self.as_name.is_empty()
    }
}

pub enum FilterInputType {
    App,
    Domain,
    Country,
    AS,
}
