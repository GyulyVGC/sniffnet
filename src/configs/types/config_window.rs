use iced::window::Position;
use iced::{Point, Size};
use serde::{Deserialize, Serialize};

#[cfg(not(test))]
use crate::SNIFFNET_LOWERCASE;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
pub struct ConfigWindow {
    pub position: (i32, i32),
    pub size: (u32, u32),
    pub thumbnail_position: (i32, i32),
}

impl ConfigWindow {
    pub const DEFAULT_SIZE: (u32, u32) = (1190, 670);
    const THUMBNAIL_SIZE: (u32, u32) = (360, 222);

    const MIN_POS_X: i32 = -50;
    const MIN_POS_Y: i32 = -50;
    const MAX_POS_X: i32 = 1100;
    const MAX_POS_Y: i32 = 700;

    const MIN_SIZE_X: u32 = 100;
    const MIN_SIZE_Y: u32 = 100;

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

    pub fn thumbnail_size(factor: f64) -> (u32, u32) {
        Self::THUMBNAIL_SIZE.scale_and_check(factor)
    }
}

impl Default for ConfigWindow {
    fn default() -> Self {
        Self {
            position: (0, 0),
            size: ConfigWindow::DEFAULT_SIZE,
            thumbnail_position: (0, 0),
        }
    }
}

pub trait ToPosition {
    fn to_position(self) -> Position;
}

impl ToPosition for (i32, i32) {
    fn to_position(self) -> Position {
        #[allow(clippy::cast_precision_loss)]
        Position::Specific(Point {
            x: self.0 as f32,
            y: self.1 as f32,
        })
    }
}

pub trait ToPoint {
    fn to_point(self) -> Point;
}

impl ToPoint for (i32, i32) {
    fn to_point(self) -> Point {
        #[allow(clippy::cast_precision_loss)]
        Point {
            x: self.0 as f32,
            y: self.1 as f32,
        }
    }
}

pub trait ToSize {
    fn to_size(self) -> Size;
}

impl ToSize for (u32, u32) {
    fn to_size(self) -> Size {
        #[allow(clippy::cast_precision_loss)]
        Size {
            width: self.0 as f32,
            height: self.1 as f32,
        }
    }
}

pub trait ScaleAndCheck {
    fn scale_and_check(self, factor: f64) -> Self;
}

impl ScaleAndCheck for (u32, u32) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn scale_and_check(self, factor: f64) -> (u32, u32) {
        let mut x = (f64::from(self.0) * factor) as u32;
        let mut y = (f64::from(self.1) * factor) as u32;
        if x < ConfigWindow::MIN_SIZE_X {
            x = ConfigWindow::MIN_SIZE_X;
        }
        if y < ConfigWindow::MIN_SIZE_Y {
            y = ConfigWindow::MIN_SIZE_Y;
        }
        (x, y)
    }
}

impl ScaleAndCheck for (i32, i32) {
    #[allow(clippy::cast_possible_truncation)]
    fn scale_and_check(self, factor: f64) -> (i32, i32) {
        let mut x = (f64::from(self.0) * factor) as i32;
        let mut y = (f64::from(self.1) * factor) as i32;
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
