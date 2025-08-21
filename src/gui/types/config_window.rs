#[cfg(not(test))]
use crate::utils::error_logger::{ErrorLogger, Location};
#[cfg(not(test))]
use crate::{SNIFFNET_LOWERCASE, location};
use iced::window::Position;
use iced::{Point, Size};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct PositionTuple(pub f32, pub f32);
#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct SizeTuple(pub f32, pub f32);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct ConfigWindow {
    pub position: PositionTuple,
    pub size: SizeTuple,
    pub thumbnail_position: PositionTuple,
}

impl ConfigWindow {
    pub const DEFAULT_SIZE: SizeTuple = SizeTuple(1190.0, 670.0);
    const THUMBNAIL_SIZE: SizeTuple = SizeTuple(360.0, 222.0);

    const MIN_POS_X: f32 = -50.0;
    const MIN_POS_Y: f32 = -50.0;
    const MAX_POS_X: f32 = 1100.0;
    const MAX_POS_Y: f32 = 700.0;

    const MIN_SIZE_X: f32 = 100.0;
    const MIN_SIZE_Y: f32 = 100.0;

    const FILE_NAME: &'static str = "window";
    #[cfg(not(test))]
    pub fn load() -> Self {
        if let Ok(window) = confy::load::<ConfigWindow>(SNIFFNET_LOWERCASE, Self::FILE_NAME) {
            window
        } else {
            let _ = confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, ConfigWindow::default())
                .log_err(location!());
            ConfigWindow::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) -> Result<(), confy::ConfyError> {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).log_err(location!())
    }

    pub fn thumbnail_size(factor: f64) -> SizeTuple {
        Self::THUMBNAIL_SIZE.scale_and_check(factor)
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            position: PositionTuple(0.0, 0.0),
            size: ConfigWindow::DEFAULT_SIZE,
            thumbnail_position: PositionTuple(0.0, 0.0),
        }
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
    fn scale_and_check(self, factor: f64) -> Self;
}

impl ScaleAndCheck for SizeTuple {
    fn scale_and_check(self, factor: f64) -> SizeTuple {
        #[allow(clippy::cast_possible_truncation)]
        let factor = factor as f32;
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
    fn scale_and_check(self, factor: f64) -> PositionTuple {
        #[allow(clippy::cast_possible_truncation)]
        let factor = factor as f32;
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

#[cfg(test)]
mod tests {
    use crate::ConfigWindow;

    impl ConfigWindow {
        pub fn test_path() -> String {
            format!("{}/{}.toml", env!("CARGO_MANIFEST_DIR"), Self::FILE_NAME)
        }

        pub fn load() -> Self {
            confy::load_path::<ConfigWindow>(ConfigWindow::test_path())
                .unwrap_or_else(|_| ConfigWindow::default())
        }

        pub fn store(self) -> Result<(), confy::ConfyError> {
            confy::store_path(ConfigWindow::test_path(), self)
        }
    }
}
