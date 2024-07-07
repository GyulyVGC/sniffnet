/// Struct to represent an Autonomous System
#[derive(Default, Clone, PartialEq, Eq, Hash, Debug)]
pub struct Asn {
    /// Autonomous System number
    pub code: String,
    /// Autonomous System name
    pub name: String,
}
