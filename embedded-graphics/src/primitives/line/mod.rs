//! The line primitive

mod bresenham;
mod points;
mod styled;
mod thick_points;

use crate::{
    geometry::{Dimensions, Point},
    primitives::{Primitive, Rectangle},
    transform::Transform,
};
pub use points::Points;
pub use styled::StyledPixels;
pub use thick_points::ThickPoints;

/// Line primitive
///
/// # Examples
///
/// ## Create some lines with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565, prelude::*, primitives::Line, style::PrimitiveStyle,
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
/// # let mut display = MockDisplay::default();
///
/// // Red 1 pixel wide line from (50, 20) to (60, 35)
/// Line::new(Point::new(50, 20), Point::new(60, 35))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
///     .draw(&mut display)?;
///
/// // Green 10 pixel wide line with translation applied
/// Line::new(Point::new(50, 20), Point::new(60, 35))
///     .translate(Point::new(-30, 10))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::GREEN, 10))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Line {
    /// Start point
    pub start: Point,

    /// End point
    pub end: Point,
}

impl Primitive for Line {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl Dimensions for Line {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::with_corners(self.start, self.end)
    }
}

impl Line {
    /// Create a new line
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    /// Returns a perpendicular line.
    ///
    /// The returned line is rotated 90 degree counter clockwise and shares the start point with the
    /// original line.
    fn perpendicular(&self) -> Self {
        let delta = self.end - self.start;
        let delta = Point::new(delta.y, -delta.x);

        Line::new(self.start, self.start + delta)
    }
}

impl Transform for Line {
    /// Translate the line from its current position to a new position by (x, y) pixels, returning
    /// a new `Line`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// let line = Line::new(Point::new(5, 10), Point::new(15, 20));
    /// let moved = line.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.start, Point::new(15, 20));
    /// assert_eq!(moved.end, Point::new(25, 30));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            start: self.start + by,
            end: self.end + by,
        }
    }

    /// Translate the line from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Line;
    /// # use embedded_graphics::prelude::*;
    /// let mut line = Line::new(Point::new(5, 10), Point::new(15, 20));
    /// line.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(line.start, Point::new(15, 20));
    /// assert_eq!(line.end, Point::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.start += by;
        self.end += by;

        self
    }
}

/// Pixel iterator for each pixel in the line
#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::{Drawable, Pixel},
        geometry::Size,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        style::PrimitiveStyle,
    };
    use arrayvec::ArrayVec;

    #[test]
    fn bounding_box() {
        let start = Point::new(10, 10);
        let end = Point::new(19, 29);

        let line: Line = Line::new(start, end);
        let backwards_line: Line = Line::new(end, start);

        assert_eq!(
            line.bounding_box(),
            Rectangle::new(start, Size::new(10, 20))
        );
        assert_eq!(
            backwards_line.bounding_box(),
            Rectangle::new(start, Size::new(10, 20))
        );
    }

    #[test]
    fn no_stroke_width_no_line() {
        let start = Point::new(2, 3);
        let end = Point::new(3, 2);

        let line =
            Line::new(start, end).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 0));

        assert!(line.into_iter().eq(core::iter::empty()));
    }

    #[test]
    fn thick_line_octant_1() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "   #                   ",
                "  #####                ",
                "  ########             ",
                "  ###########          ",
                " ###############       ",
                "    ###############    ",
                "       ############### ",
                "          ###########  ",
                "             ########  ",
                "                #####  ",
                "                   #   ",
            ])
        );
    }

    #[test]
    fn thick_line_2px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 2), Point::new(10, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(&mut display)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 5), Point::new(2, 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 2))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "  ######### ",
                "  ######### ",
                "            ",
                "            ",
                "  ..        ",
                "  ..        ",
                "  ..        ",
                "  ..        ",
                "  ..        ",
                "  ..        ",
            ])
        );
    }

    // Check that 45 degree lines don't draw their right side 1px too long
    #[test]
    fn diagonal() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(3, 2), Point::new(10, 9))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 7))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     #        ",
                "    ###       ",
                "   #####      ",
                "  #######     ",
                " #########    ",
                "  #########   ",
                "   #########  ",
                "    ######### ",
                "     #######  ",
                "      #####   ",
                "       ###    ",
                "        #     ",
            ])
        );
    }

    #[test]
    fn thick_line_3px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 2), Point::new(10, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut display)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 5), Point::new(2, 10))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::Off, 3))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "  ######### ",
                "  ######### ",
                "  ######### ",
                "            ",
                " ...        ",
                " ...        ",
                " ...        ",
                " ...        ",
                " ...        ",
                " ...        ",
            ])
        );
    }

    #[test]
    fn thick_line_0px() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(2, 2), Point::new(2, 2))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 3))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "   ", //
                "  #", //
                "  #", //
                "  #", //
            ])
        );
    }

    #[test]
    fn event_width_offset() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        // Horizontal line
        Line::new(Point::new(2, 3), Point::new(10, 3))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
            .draw(&mut display)
            .unwrap();

        // Vertical line
        Line::new(Point::new(2, 9), Point::new(10, 8))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 4))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "  ######### ",
                "  ######### ",
                "  ######### ",
                "  ######### ",
                "            ",
                "       #### ",
                "  ######### ",
                "  ######### ",
                "  ######### ",
                "  #####     ",
            ])
        );
    }

    #[test]
    fn points_iter() {
        let line = Line::new(Point::new(10, 10), Point::new(20, 30));

        let styled_points: ArrayVec<[_; 32]> = line
            .clone()
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_iter()
            .map(|Pixel(p, _)| p)
            .collect();

        let points: ArrayVec<[_; 32]> = line.points().collect();

        assert_eq!(points, styled_points);
    }

    #[test]
    fn perpendicular() {
        assert_eq!(
            Line::new(Point::zero(), Point::new(10, 0)).perpendicular(),
            Line::new(Point::zero(), Point::new(0, -10))
        );

        assert_eq!(
            Line::new(Point::new(10, 20), Point::new(20, 10)).perpendicular(),
            Line::new(Point::new(10, 20), Point::new(0, 10))
        );

        assert_eq!(
            Line::new(Point::zero(), Point::new(0, -10)).perpendicular(),
            Line::new(Point::zero(), Point::new(-10, 0))
        );
    }
}
