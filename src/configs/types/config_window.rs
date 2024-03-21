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
    pub const MIN_SIZE: (u32, u32) = (800, 500);
    pub const THUMBNAIL_SIZE: (u32, u32) = (360, 222);

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

pub trait Scale {
    fn scale(self, factor: f64) -> Self;
}

impl Scale for (u32, u32) {
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn scale(self, factor: f64) -> (u32, u32) {
        let x = (f64::from(self.0) * factor) as u32;
        let y = (f64::from(self.1) * factor) as u32;
        (x, y)
    }
}

impl Scale for (i32, i32) {
    #[allow(clippy::cast_possible_truncation)]
    fn scale(self, factor: f64) -> (i32, i32) {
        let x = (f64::from(self.0) * factor) as i32;
        let y = (f64::from(self.1) * factor) as i32;
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
