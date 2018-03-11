use super::super::drawable::*;

// TODO: Impl Default so people can leave the color bit out
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub top_left: Coord,
    pub bottom_right: Coord,
    pub color: Color,
}

impl Rect {
    pub fn new(top_left: Coord, bottom_right: Coord, color: u8) -> Self {
        Rect {
            top_left,
            bottom_right,
            color,
        }
    }
}

impl<'a> IntoIterator for &'a Rect {
    type Item = Pixel;
    type IntoIter = RectIterator;

    fn into_iter(self) -> Self::IntoIter {
        RectIterator {
            top_left: self.top_left,
            bottom_right: self.bottom_right,
            color: self.color,
            x: self.top_left.0,
            y: self.top_left.1,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct RectIterator {
    top_left: Coord,
    bottom_right: Coord,
    color: Color,
    x: u32,
    y: u32,
}

impl Iterator for RectIterator {
    type Item = (Coord, Color);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.bottom_right.0 && self.y >= self.bottom_right.1 {
            return None;
        }

        let coord = (self.x, self.y);

        // Step across 1 if rendering top/bottom lines
        if self.y == self.top_left.1 || self.y == self.bottom_right.1 {
            self.x += 1;
        }
        // Skip across rect empty space if rendering left/right lines
        else {
            self.x += self.bottom_right.0 - self.top_left.0;
        }

        // Reached end of row? Jump down one line
        if self.x > self.bottom_right.0 {
            self.x = self.top_left.0;
            self.y += 1;
        }

        Some((coord, self.color))
    }
}

impl Drawable for Rect {}
