use crate::{
    geometry::Point,
    primitives::{
        common::Scanline,
        rounded_rectangle::{RoundedRectangle, RoundedRectangleContains},
        ContainsPoint,
    },
};

/// Iterator over all points inside the rounded rectangle.
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Points {
    scanlines: Scanlines,
    current_scanline: Scanline,
}

impl Points {
    pub(in crate::primitives) fn new(rounded_rectangle: &RoundedRectangle) -> Self {
        Self {
            scanlines: Scanlines::new(rounded_rectangle),
            current_scanline: Scanline::new_empty(0),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_scanline.next().or_else(|| {
            self.current_scanline = self.scanlines.next()?;
            self.current_scanline.next()
        })
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct Scanlines {
    rounded_rectangle: RoundedRectangleContains,
}

impl Scanlines {
    pub fn new(rounded_rectangle: &RoundedRectangle) -> Self {
        Self {
            rounded_rectangle: RoundedRectangleContains::new(rounded_rectangle),
        }
    }
}

impl Iterator for Scanlines {
    type Item = Scanline;

    fn next(&mut self) -> Option<Self::Item> {
        let columns = self.rounded_rectangle.columns.clone();
        let y = self.rounded_rectangle.rows.next()?;

        let x_start = if y < self.rounded_rectangle.straight_rows_left.start {
            columns
                .clone()
                .find(|x| self.rounded_rectangle.top_left.contains(Point::new(*x, y)))
        } else if y >= self.rounded_rectangle.straight_rows_left.end {
            columns.clone().find(|x| {
                self.rounded_rectangle
                    .bottom_left
                    .contains(Point::new(*x, y))
            })
        } else {
            None
        }
        .unwrap_or(columns.start);

        let x_end = if y < self.rounded_rectangle.straight_rows_right.start {
            columns
                .clone()
                .rfind(|x| self.rounded_rectangle.top_right.contains(Point::new(*x, y)))
        } else if y >= self.rounded_rectangle.straight_rows_right.end {
            columns.clone().rfind(|x| {
                self.rounded_rectangle
                    .bottom_right
                    .contains(Point::new(*x, y))
            })
        } else {
            None
        }
        .map(|x| x + 1)
        .unwrap_or(columns.end);

        Some(Scanline::new(y, x_start..x_end))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Size,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{PointsIter, Primitive, PrimitiveStyle, Rectangle},
        Drawable,
    };

    #[test]
    fn points_equals_filled() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        );

        let mut expected = MockDisplay::new();
        rounded_rect
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut expected)
            .unwrap();

        MockDisplay::from_points(rounded_rect.points(), BinaryColor::On).assert_eq(&expected);
    }
}
