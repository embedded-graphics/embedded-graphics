use crate::{
    geometry::{Dimensions, Point},
    primitives::{
        rectangle::{self, Rectangle},
        rounded_rectangle::{
            ellipse_quadrant::{self, Quadrant},
            RoundedRectangle,
        },
        PointsIter,
    },
};

/// Iterator over all points inside the rounded rectangle.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    rect_iter: rectangle::Points,

    top_left_corner: Rectangle,
    top_right_corner: Rectangle,
    bottom_right_corner: Rectangle,
    bottom_left_corner: Rectangle,

    top_left_iter: ellipse_quadrant::Points,
    top_right_iter: ellipse_quadrant::Points,
    bottom_right_iter: ellipse_quadrant::Points,
    bottom_left_iter: ellipse_quadrant::Points,
}

impl Points {
    pub(in crate::primitives) fn new(shape: &RoundedRectangle) -> Self {
        let top_left_ellipse = shape.get_confined_corner_quadrant(Quadrant::TopLeft);
        let top_right_ellipse = shape.get_confined_corner_quadrant(Quadrant::TopRight);
        let bottom_right_ellipse = shape.get_confined_corner_quadrant(Quadrant::BottomRight);
        let bottom_left_ellipse = shape.get_confined_corner_quadrant(Quadrant::BottomLeft);

        Self {
            rect_iter: shape.rectangle.points(),

            top_left_iter: top_left_ellipse.points(),
            top_right_iter: top_right_ellipse.points(),
            bottom_right_iter: bottom_right_ellipse.points(),
            bottom_left_iter: bottom_left_ellipse.points(),

            top_left_corner: top_left_ellipse.bounding_box(),
            top_right_corner: top_right_ellipse.bounding_box(),
            bottom_right_corner: bottom_right_ellipse.bounding_box(),
            bottom_left_corner: bottom_left_ellipse.bounding_box(),
        }
    }

    pub(in crate::primitives) fn empty() -> Self {
        Self {
            rect_iter: rectangle::Points::empty(),
            top_left_iter: ellipse_quadrant::Points::empty(),
            top_right_iter: ellipse_quadrant::Points::empty(),
            bottom_right_iter: ellipse_quadrant::Points::empty(),
            bottom_left_iter: ellipse_quadrant::Points::empty(),
            top_left_corner: Rectangle::zero(),
            top_right_corner: Rectangle::zero(),
            bottom_right_corner: Rectangle::zero(),
            bottom_left_corner: Rectangle::zero(),
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            top_left_corner,
            top_right_corner,
            bottom_right_corner,
            bottom_left_corner,
            ..
        } = self;

        self.rect_iter
            .find(|p| {
                !top_left_corner.contains(*p)
                    && !top_right_corner.contains(*p)
                    && !bottom_right_corner.contains(*p)
                    && !bottom_left_corner.contains(*p)
            })
            .or_else(|| self.top_left_iter.next())
            .or_else(|| self.top_right_iter.next())
            .or_else(|| self.bottom_right_iter.next())
            .or_else(|| self.bottom_left_iter.next())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Size,
        iterator::IntoPixels,
        pixelcolor::BinaryColor,
        primitives::{Primitive, PrimitiveStyle},
    };

    #[test]
    fn points_equals_filled() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        );

        assert!(rounded_rect.points().eq(rounded_rect
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_pixels()
            .map(|pixel| pixel.0)));
    }
}
