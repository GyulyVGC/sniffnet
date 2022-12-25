/// Used to specify the kind of `iced` element, to be able to choose the appropriate style for it
#[derive(Clone, Copy, Eq, PartialEq)]
pub enum ElementType {
    Standard,
    Headers,
    BorderedRound,
    TabActive,
    TabInactive,
}
