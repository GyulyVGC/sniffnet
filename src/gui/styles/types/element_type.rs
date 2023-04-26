/// Used to specify the kind of `iced` element, to be able to choose the appropriate style for it
#[derive(Clone, Copy)]
pub enum ElementType {
    Standard,
    Headers,
    BorderedRound,
    BorderedRoundSelected,
    TabActive,
    TabInactive,
    Starred,
    Neutral,
    Alert,
    Tooltip,
    Badge,
    Incoming,
    Outgoing,
}
