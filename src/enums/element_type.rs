/// Used to specify the kind of `iced` element, to be able to choose the appropriate style for it
#[derive(Copy, Eq, PartialEq)]
pub enum ElementType {
    Standard,
    Headers,
    BorderedRound,
    TabActive,
    TabInactive,
    SelectedRadio,
}

impl Clone for ElementType {
    fn clone(&self) -> Self {
        *self
    }
}
