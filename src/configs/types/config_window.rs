use iced::window::Position;
use iced::{Point, Size};
use serde::{Deserialize, Serialize};

#[cfg(not(test))]
use crate::SNIFFNET_LOWERCASE;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct ConfigWindow {
    pub position: (f32, f32),
    pub size: (f32, f32),
    pub thumbnail_position: (f32, f32),
}

impl ConfigWindow {
    pub const DEFAULT_SIZE: (f32, f32) = (1190.0, 670.0);
    const THUMBNAIL_SIZE: (f32, f32) = (360.0, 222.0);

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
            confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, ConfigWindow::default())
                .unwrap_or(());
            ConfigWindow::default()
        }
    }

    #[cfg(not(test))]
    pub fn store(self) {
        confy::store(SNIFFNET_LOWERCASE, Self::FILE_NAME, self).unwrap_or(());
    }

    pub fn thumbnail_size(factor: f64) -> (f32, f32) {
        Self::THUMBNAIL_SIZE.scale_and_check(factor)
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            position: (0.0, 0.0),
            size: ConfigWindow::DEFAULT_SIZE,
            thumbnail_position: (0.0, 0.0),
        }
    }
}

pub trait ToPosition {
    fn to_position(self) -> Position;
}

impl ToPosition for (f32, f32) {
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

impl ToPoint for (f32, f32) {
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

impl ToSize for (f32, f32) {
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

impl ScaleAndCheck for (f32, f32) {
    fn scale_and_check(self, factor: f64) -> (f32, f32) {
        let factor = factor as f32;
        let mut x = self.0 * factor;
        let mut y = self.1 * factor;
        if x < ConfigWindow::MIN_SIZE_X {
            x = ConfigWindow::MIN_SIZE_X;
        }
        if y < ConfigWindow::MIN_SIZE_Y {
            y = ConfigWindow::MIN_SIZE_Y;
        }
        (x, y)
    }
}

impl ScaleAndCheck for (f32, f32) {
    fn scale_and_check(self, factor: f64) -> (f32, f32) {
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
        (x, y)
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

        pub fn store(self) {
            confy::store_path(ConfigWindow::test_path(), self).unwrap_or(());
        }
    }
}
