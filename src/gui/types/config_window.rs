use crate::gui::types::conf::deserialize_or_default;
use iced::{Point, Size};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug, Default)]
#[serde(default)]
pub struct ConfigWindow {
    #[serde(deserialize_with = "deserialize_or_default")]
    size: SizeTuple,
    #[serde(deserialize_with = "deserialize_or_default")]
    position: PositionTuple,
    #[serde(deserialize_with = "deserialize_or_default")]
    thumbnail_position: PositionTuple,
}

impl ConfigWindow {
    #[cfg(test)]
    pub(crate) fn new(
        size: (f32, f32),
        position: (f32, f32),
        thumbnail_position: (f32, f32),
    ) -> Self {
        Self {
            size: SizeTuple(size.0, size.1),
            position: PositionTuple(position.0, position.1),
            thumbnail_position: PositionTuple(thumbnail_position.0, thumbnail_position.1),
        }
    }

    #[cfg(not(test))]
    pub(crate) fn sanitize(&mut self, scale_factor: f32) {
        self.size.sanitize(scale_factor);
        self.position.sanitize();
        self.thumbnail_position.sanitize();
    }

    pub(crate) fn set_size(&mut self, width: f32, height: f32, scale_factor: f32) {
        let mut size = SizeTuple(width, height);
        size.sanitize(scale_factor);
        self.size = size;
    }

    pub(crate) fn scale_size(&mut self, old_factor: f32, new_factor: f32) {
        self.size.scale_and_sanitize(old_factor, new_factor);
    }

    pub(crate) fn set_position(&mut self, x: f32, y: f32, factor: f32) {
        let mut position = PositionTuple(x, y);
        position.scale_and_sanitize(factor);
        self.position = position;
    }

    pub(crate) fn set_thumbnail_position(&mut self, x: f32, y: f32, factor: f32) {
        let mut position = PositionTuple(x, y);
        position.scale_and_sanitize(factor);
        self.thumbnail_position = position;
    }

    pub(crate) fn size(&self) -> Size {
        self.size.to_size()
    }

    pub(crate) fn position(&self) -> Point {
        self.position.to_point()
    }

    pub(crate) fn thumbnail_position(&self) -> Point {
        self.thumbnail_position.to_point()
    }
}

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug, Default)]
struct PositionTuple(f32, f32);

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Debug)]
struct SizeTuple(f32, f32);

impl Default for SizeTuple {
    fn default() -> Self {
        Self(1190.0, 670.0)
    }
}

impl SizeTuple {
    // TODO: improve based on monitor size
    fn sanitize(&mut self, scale_factor: f32) {
        let min_size = 100.0 / scale_factor;
        let max_size = 8192.0 / scale_factor;
        let mut x = self.0;
        let mut y = self.1;
        if x < min_size {
            x = min_size;
        }
        if y < min_size {
            y = min_size;
        }
        if x > max_size {
            x = max_size;
        }
        if y > max_size {
            y = max_size;
        }
        self.0 = x;
        self.1 = y;
    }

    fn scale_and_sanitize(&mut self, old_factor: f32, new_factor: f32) {
        self.0 *= old_factor / new_factor;
        self.1 *= old_factor / new_factor;
        self.sanitize(new_factor);
    }

    fn to_size(self) -> Size {
        Size {
            width: self.0,
            height: self.1,
        }
    }
}

impl PositionTuple {
    // TODO: improve based on monitor size (and sanitized window size)
    fn sanitize(&mut self) {
        let min_pos = -50.0;
        let max_pos_x = 1100.0;
        let max_pos_y = 700.0;
        let mut x = self.0;
        let mut y = self.1;
        if x < min_pos {
            x = min_pos;
        }
        if y < min_pos {
            y = min_pos;
        }
        if x > max_pos_x {
            x = max_pos_x;
        }
        if y > max_pos_y {
            y = max_pos_y;
        }
        self.0 = x;
        self.1 = y;
    }

    fn scale_and_sanitize(&mut self, factor: f32) {
        self.0 *= factor;
        self.1 *= factor;
        self.sanitize();
    }

    fn to_point(self) -> Point {
        Point {
            x: self.0,
            y: self.1,
        }
    }
}
