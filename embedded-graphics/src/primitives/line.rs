//! The line primitive

use crate::{
    draw_target::DrawTarget, drawable::Drawable, drawable::Pixel, geometry::Dimensions,
    geometry::Point, geometry::Size, pixelcolor::PixelColor, primitives::Primitive,
    style::PrimitiveStyle, style::Styled, transform::Transform,
};

/// Line primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egline.html) make for more concise code.
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
///     .translate(Point::new(65, 35))
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
impl Primitive for Line {}

impl Dimensions for Line {
    fn top_left(&self) -> Point {
        Point::new(self.start.x.min(self.end.x), self.start.y.min(self.end.y))
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        Size::from_bounding_box(self.start, self.end)
    }
}

impl Line {
    /// Create a new line
    pub const fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }
}

/// Which side of the center line to draw on
///
/// Imagine standing on `start`, looking ahead to where `end` is. `Left` is to your left, `Right` to
/// your right.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum Side {
    Left,
    Right,
}

impl Side {
    fn swap(&mut self) {
        match self {
            Self::Left => *self = Self::Right,
            Self::Right => *self = Self::Left,
        }
    }
}

/// Current state of each parallel line drawn
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct ParallelLineState {
    start: Point,
    error: i32,
    dx_accum: u32,
}

impl ParallelLineState {
    fn new(start: Point, initial_error: i32) -> Self {
        Self {
            start,
            error: initial_error,
            dx_accum: 0,
        }
    }
}

/// Current side state
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
struct SideState {
    /// Parallel line start point
    start: Point,

    /// Error accumulator
    error: i32,

    /// Perpendicular error accumulator
    p_error: i32,
}

impl SideState {
    fn new(start: Point) -> Self {
        Self {
            start,
            error: 0,
            p_error: 0,
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct LineIterator {
    /// Bresenham error threshold
    ///
    /// If this is exceeded, a "minor" move is made
    threshold: i32,

    /// "Major" error component
    e_diag: i32,

    /// "Minor" error component
    e_square: i32,

    /// The "major" (greater) delta. Swapped with `dy` if dy is greater than dx
    dx: u32,

    /// The "minor" (lesser) delta. Swapped with `dx` if dx is greater than dy
    dy: u32,

    /// Line thickness in arbitrary units
    ///
    /// Thickness is calculated according to the section titled "Fixing the Thickness" in [this
    /// article](http://kt8216.unixcab.org/murphy/index.html). The difference in this implementation
    /// is that both sides of the comparison are squared, removing the need for an expensive
    /// `sqrt()` call.
    thickness: u32,

    /// Step direction
    direction: Point,

    /// Line start point
    start: Point,

    /// Line end point
    end: Point,

    /// The "major" step
    ///
    /// The X or Y component with the larger delta is considered "major". This is the most common
    /// direction to move in.
    step_major: Point,

    /// The "minor" step
    ///
    /// The X or Y component with the smaller delta is considered "minor". This is the less common
    /// direction to move in.
    step_minor: Point,

    /// Thickness of pixels drawn so far
    ///
    /// Compared against `thickness` for width limit
    thickness_accum: u32,

    /// Which side the _next_ parallel line will be on
    ///
    /// Lines start down the center, then alternate between left, then right. For lines with an even
    /// width, the line is unbalanced by 1px to the left.
    next_side: Side,

    /// State of the parallel line currently being iterated over
    parallel: ParallelLineState,

    /// Left side state
    left: SideState,

    /// Right side state
    right: SideState,
}

impl LineIterator {
    /// Create a new line iterator from a `Line` and a stroke width
    ///
    /// Lines with a thickness greater than 1px are filled using multiple parallel lines to the
    /// left/right of the central original line.
    pub(crate) fn new(line: &Line, stroke_width: u32) -> Self {
        let dx: i32 = line.end.x - line.start.x;
        let dy: i32 = line.end.y - line.start.y;

        let direction = match (dx >= 0, dy >= 0) {
            (true, true) => Point::new(1, 1),
            (true, false) => Point::new(1, -1),
            (false, true) => Point::new(-1, 1),
            (false, false) => Point::new(-1, -1),
        };

        // Thickness threshold, taking into account that fewer pixels are required to draw a
        // diagonal line of the same perceived width.
        let thickness = 4 * stroke_width.pow(2) * (dx.pow(2) as u32 + dy.pow(2) as u32);

        let mut dx = dx.abs();
        let mut dy = dy.abs();

        // Swap components if line is Y-major. dx is always the "major" direction delta.
        let (step_major, step_minor) = if dy > dx {
            core::mem::swap(&mut dx, &mut dy);

            (Point::new(0, direction.y), Point::new(direction.x, 0))
        } else {
            (Point::new(direction.x, 0), Point::new(0, direction.y))
        };

        let threshold = dx - 2 * dy;
        let e_diag = -2 * dx;
        let e_square = 2 * dy;

        // Safe due to abs() call above
        let dx = dx as u32;
        let dy = dy as u32;

        Self {
            step_major,
            step_minor,
            dx: dx,
            dy: dy,
            start: line.start,
            end: line.end,
            threshold,
            e_diag,
            e_square,
            thickness,
            direction,
            thickness_accum: dx + dy,
            // Next side after current line in `state` is drawn will be right side
            next_side: Side::Right,
            parallel: ParallelLineState::new(line.start, 0),
            left: SideState::new(line.start),
            right: SideState::new(line.start),
        }
    }
}

impl Iterator for LineIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        // Quit iterator if width threshold is reached or the line has no length
        if self.thickness_accum.pow(2) > self.thickness || self.dx == 0 {
            return None;
        }

        self.parallel.dx_accum += 1;

        if self.parallel.dx_accum <= self.dx + 1 {
            let p = self.parallel.start;

            if self.parallel.error > self.threshold {
                self.parallel.start += self.step_minor;
                self.parallel.error += self.e_diag;
            }

            self.parallel.start += self.step_major;
            self.parallel.error += self.e_square;

            Some(p)
        } else {
            let side = match self.next_side {
                Side::Left => &mut self.left,
                Side::Right => &mut self.right,
            };

            let mut extra = false;
            let start = side.start;

            if side.error > self.threshold {
                match self.next_side {
                    Side::Left => side.start += self.step_major,
                    Side::Right => side.start -= self.step_major,
                };
                side.error += self.e_diag;
                self.thickness_accum += 2 * self.dy;

                if side.p_error > self.threshold {
                    extra = true;

                    let (start, p_error) = match self.next_side {
                        Side::Left => (start, side.p_error + self.e_diag),
                        Side::Right => (start + self.step_minor, -side.p_error),
                    };
                    self.parallel = ParallelLineState::new(start, p_error);

                    side.p_error += self.e_diag;
                }

                side.p_error += self.e_square;
            }

            if !extra {
                match self.next_side {
                    Side::Left => side.start -= self.step_minor,
                    Side::Right => side.start += self.step_minor,
                };
                side.error += self.e_square;
                self.thickness_accum += 2 * self.dx;

                let p_error = match self.next_side {
                    Side::Left => side.p_error,
                    Side::Right => -side.p_error,
                };
                self.parallel = ParallelLineState::new(side.start, p_error);
            }

            // Switch to opposite side of line to keep it balanced
            self.next_side.swap();

            Self::next(self)
        }
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
            ..*self
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

impl<'a, C> IntoIterator for &'a Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledLineIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledLineIterator {
            style: self.style,

            line_iter: LineIterator::new(&self.primitive, self.style.stroke_width),
        }
    }
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledLineIterator<C>
where
    C: PixelColor,
{
    style: PrimitiveStyle<C>,

    line_iter: LineIterator,
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C: PixelColor> Iterator for StyledLineIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Break if stroke width is zero
        if self.style.stroke_width == 0 {
            return None;
        }

        // Return none if stroke color is none
        let stroke_color = self.style.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_line(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{drawable::Pixel, mock_display::MockDisplay, pixelcolor::BinaryColor};

    fn test_expected_line(start: Point, end: Point, expected: &[(i32, i32)]) {
        let line =
            Line::new(start, end).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
        let mut expected_iter = expected.iter();
        for Pixel(coord, _) in line.into_iter() {
            match expected_iter.next() {
                Some(point) => assert_eq!(coord, Point::from(*point)),
                // expected runs out of points before line does
                None => unreachable!(),
            }
        }
        // check that expected has no points left
        assert!(expected_iter.next().is_none())
    }

    #[test]
    fn bounding_box() {
        let start = Point::new(10, 10);
        let end = Point::new(20, 20);

        let line: Line = Line::new(start, end);
        let backwards_line: Line = Line::new(end, start);

        assert_eq!(line.top_left(), start);
        assert_eq!(line.bottom_right(), end);
        assert_eq!(line.size(), Size::new(10, 10));

        assert_eq!(backwards_line.top_left(), start);
        assert_eq!(backwards_line.bottom_right(), end);
        assert_eq!(backwards_line.size(), Size::new(10, 10));
    }

    #[test]
    fn draws_no_dot() {
        let start = Point::new(10, 10);
        let end = Point::new(10, 10);
        let expected = [];
        test_expected_line(start, end, &expected);
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
    fn draws_short_correctly() {
        let start = Point::new(2, 3);
        let end = Point::new(3, 2);
        let expected = [(2, 3), (3, 2)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_1_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(15, 13);
        let expected = [(10, 10), (11, 11), (12, 11), (13, 12), (14, 12), (15, 13)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_2_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(13, 15);
        let expected = [(10, 10), (11, 11), (11, 12), (12, 13), (12, 14), (13, 15)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_3_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(7, 15);
        let expected = [(10, 10), (9, 11), (9, 12), (8, 13), (8, 14), (7, 15)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_4_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(5, 13);
        let expected = [(10, 10), (9, 11), (8, 11), (7, 12), (6, 12), (5, 13)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_5_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(5, 7);
        let expected = [(10, 10), (9, 9), (8, 9), (7, 8), (6, 8), (5, 7)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_6_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(7, 5);
        let expected = [(10, 10), (9, 9), (9, 8), (8, 7), (8, 6), (7, 5)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_7_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(13, 5);
        let expected = [(10, 10), (11, 9), (11, 8), (12, 7), (12, 6), (13, 5)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn draws_octant_8_correctly() {
        let start = Point::new(10, 10);
        let end = Point::new(15, 7);
        let expected = [(10, 10), (11, 9), (12, 9), (13, 8), (14, 8), (15, 7)];
        test_expected_line(start, end, &expected);
    }

    #[test]
    fn thick_line_octant_1() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Line::new(Point::new(2, 2), Point::new(20, 8))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5))
            .draw(&mut display)
            .unwrap();

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                      ",
                "  ##                  ",
                "  #####               ",
                "  ########            ",
                " ############         ",
                "    ############      ",
                "       ############   ",
                "          ########### ",
                "             ######## ",
                "                ##### ",
                "                   #  ",
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

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "            ",
                "  ######### ",
                "  ######### ",
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

        #[rustfmt::skip]
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
}
