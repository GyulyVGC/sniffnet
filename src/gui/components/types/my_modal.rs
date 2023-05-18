/// This enum defines the currently displayed modal.
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum MyModal {
    /// Quit modal.
    Quit,
    /// Clear all modal.
    ClearAll,
    /// Connection details modal.
    ConnectionDetails(usize),
}
