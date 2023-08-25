use iced::window::Position;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct ConfigWindow {
    pub position: (i32, i32),
    pub size: (u32, u32),
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            position: (0, 0),
            size: (1190, 670),
        }
    }
}

pub trait ToPosition {
    fn to_position(self) -> Position;
}

impl ToPosition for (i32, i32) {
    fn to_position(self) -> Position {
        Position::Specific(self.0, self.1)
    }
}
