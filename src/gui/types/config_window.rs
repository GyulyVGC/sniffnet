use crate::gui::types::conf::deserialize_or_default;
use iced::window::Position;
use iced::{Point, Size};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug, Default)]
pub struct PositionTuple(pub f32, pub f32);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct SizeTuple(pub f32, pub f32);

impl Default for SizeTuple {
    fn default() -> Self {
        Self(1190.0, 670.0)
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug, Default)]
#[serde(default)]
pub struct ConfigWindow {
    #[serde(deserialize_with = "deserialize_or_default")]
    pub position: PositionTuple,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub size: SizeTuple,
    #[serde(deserialize_with = "deserialize_or_default")]
    pub thumbnail_position: PositionTuple,
}

impl ConfigWindow {
    const THUMBNAIL_SIZE: SizeTuple = SizeTuple(360.0, 222.0);

    const MIN_POS_X: f32 = -50.0;
    const MIN_POS_Y: f32 = -50.0;
    const MAX_POS_X: f32 = 1100.0;
    const MAX_POS_Y: f32 = 700.0;

    const MIN_SIZE_X: f32 = 100.0;
    const MIN_SIZE_Y: f32 = 100.0;

    pub fn thumbnail_size(factor: f32) -> SizeTuple {
        Self::THUMBNAIL_SIZE.scale_and_check(factor)
    }
}

pub trait ToPosition {
    fn to_position(self) -> Position;
}

impl ToPosition for PositionTuple {
    fn to_position(self) -> Position {
        Position::Specific(Point {
            x: self.0,
            y: self.1,
        })
    }
}

pub trait ToPoint {
    fn to_point(self) -> Point;
}

impl ToPoint for PositionTuple {
    fn to_point(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}

pub trait ToSize {
    fn to_size(self) -> Size;
}

impl ToSize for SizeTuple {
    fn to_size(self) -> Size {
        Size {
            width: self.0,
            height: self.1,
        }
    }
}

pub trait ScaleAndCheck {
    fn scale_and_check(self, factor: f32) -> Self;
}

impl ScaleAndCheck for SizeTuple {
    fn scale_and_check(self, factor: f32) -> SizeTuple {
        let mut x = self.0 * factor;
        let mut y = self.1 * factor;
        if x < ConfigWindow::MIN_SIZE_X {
            x = ConfigWindow::MIN_SIZE_X;
        }
        if y < ConfigWindow::MIN_SIZE_Y {
            y = ConfigWindow::MIN_SIZE_Y;
        }
        SizeTuple(x, y)
    }
}

impl ScaleAndCheck for PositionTuple {
    fn scale_and_check(self, factor: f32) -> PositionTuple {
        let mut x = self.0 * factor;
        let mut y = self.1 * factor;
        if x < ConfigWindow::MIN_POS_X {
            x = ConfigWindow::MIN_POS_X;
        }
        if y < ConfigWindow::MIN_POS_Y {
            y = ConfigWindow::MIN_POS_Y;
        }
        if x > ConfigWindow::MAX_POS_X {
            x = ConfigWindow::MAX_POS_X;
        }
        if y > ConfigWindow::MAX_POS_Y {
            y = ConfigWindow::MAX_POS_Y;
        }
        PositionTuple(x, y)
    }
}
