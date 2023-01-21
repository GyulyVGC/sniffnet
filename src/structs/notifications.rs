use serde::{Deserialize, Serialize};

/// Used to contain the notifications configuration set by the user
#[derive(Clone, Copy, Eq, PartialEq, Serialize, Deserialize, Debug, Default)]
pub struct Notifications {
    /// Threshold of received + sent packets; if exceeded a notification is emitted
    pub packets_threshold: Option<usize>,
}

// impl ::std::default::Default for Notifications {
//     fn default() -> Self {
//         Notifications {
//             packets_threshold: None
//         }
//     }
// }
