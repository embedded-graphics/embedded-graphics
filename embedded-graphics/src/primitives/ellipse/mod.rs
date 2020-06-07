//! The ellipse primitive

mod points_iterator;
mod styled;

use crate::{
    geometry::{Dimensions, Point, Size},
    primitives::{circle, ContainsPoint, Primitive, Rectangle},
    transform::Transform,
};
pub use points_iterator::Points;
pub use styled::StyledEllipseIterator;

/// Ellipse primitive
///
/// # Examples
///
/// ## Create some ellipses with different styles
///
/// ```rust
/// use embedded_graphics::{
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::Ellipse,
///     style::{PrimitiveStyle, PrimitiveStyleBuilder},
/// };
/// # use embedded_graphics::mock_display::MockDisplay;
///
/// // Ellipse with 1 pixel wide white stroke with top-left point at (10, 20) with a size of (30, 40)
/// # let mut display = MockDisplay::default();
/// Ellipse::new(Point::new(10, 20), Size::new(30, 40))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display)?;
///
/// // Ellipse with styled stroke and fill with top-left point at (20, 30) with a size of (40, 30)
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// # let mut display = MockDisplay::default();
/// Ellipse::new(Point::new(20, 30), Size::new(40, 30))
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Ellipse with blue fill and no stroke with a translation applied
/// # let mut display = MockDisplay::default();
/// Ellipse::new(Point::new(10, 20), Size::new(20, 40))
///     .translate(Point::new(10, -15))
///     .into_styled(PrimitiveStyle::with_fill(Rgb565::BLUE))
///     .draw(&mut display)?;
/// # Ok::<(), core::convert::Infallible>(())
/// ```
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default)]
pub struct Ellipse {
    /// Top-left point of ellipse's bounding box
    pub top_left: Point,

    /// Size of the ellipse
    pub size: Size,
}

impl Ellipse {
    /// Create a new ellipse delimited with a top-left point with a specific size
    pub const fn new(top_left: Point, size: Size) -> Self {
        Ellipse { top_left, size }
    }

    /// Create a new ellipse centered around a given point with a specific size
    pub fn with_center(center: Point, size: Size) -> Self {
        let top_left = center - size.center_offset();

        Ellipse { top_left, size }
    }

    /// Return the center point of the ellipse
    pub fn center(&self) -> Point {
        self.bounding_box().center()
    }

    /// Return the center point of the ellipse scaled by a factor of 2
    ///
    /// This method is used to accurately calculate the outside edge of the ellipse.
    /// The result is not equivalent to `self.center() * 2` because of rounding.
    fn center_2x(&self) -> Point {
        center_2x(self.top_left, self.size)
    }

    fn expand(&self, offset: u32) -> Self {
        let size = self.size.saturating_add(Size::new(offset * 2, offset * 2));
        Self::with_center(self.center(), size)
    }

    fn shrink(&self, offset: u32) -> Self {
        let size = self.size.saturating_sub(Size::new(offset * 2, offset * 2));
        Self::with_center(self.center(), size)
    }
}

/// Return the center point of the ellipse scaled by a factor of 2
///
/// This method is used to accurately calculate the outside edge of the ellipse.
/// The result is not equivalent to `Ellipse::center() * 2` because of rounding.
pub(crate) fn center_2x(top_left: Point, size: Size) -> Point {
    let radius = size.saturating_sub(Size::new(1, 1));

    top_left * 2 + radius
}

impl Primitive for Ellipse {
    type PointsIter = Points;

    fn points(&self) -> Self::PointsIter {
        Points::new(self)
    }
}

impl ContainsPoint for Ellipse {
    fn contains(&self, point: Point) -> bool {
        let (size_sq, threshold) = compute_threshold(self.size);

        is_point_inside_ellipse(size_sq, point * 2 - self.center_2x(), threshold)
    }
}

impl Dimensions for Ellipse {
    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.size)
    }
}

impl Transform for Ellipse {
    /// Translate the ellipse from its current position to a new position by (x, y) pixels,
    /// returning a new `Ellipse`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Ellipse;
    /// # use embedded_graphics::prelude::*;
    /// let ellipse = Ellipse::new(Point::new(5, 10), Size::new(10, 15));
    /// let moved = ellipse.translate(Point::new(10, 10));
    ///
    /// assert_eq!(moved.top_left, Point::new(15, 20));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            top_left: self.top_left + by,
            ..*self
        }
    }

    /// Translate the ellipse from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::primitives::Ellipse;
    /// # use embedded_graphics::prelude::*;
    /// let mut ellipse = Ellipse::new(Point::new(5, 10), Size::new(10, 15));
    /// ellipse.translate_mut(Point::new(10, 10));
    ///
    /// assert_eq!(ellipse.top_left, Point::new(15, 20));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.top_left += by;

        self
    }
}

pub(crate) fn compute_threshold(size: Size) -> (Size, u32) {
    let Size { width, height } = size;

    let a = width.pow(2);
    let b = height.pow(2);

    // Special case for circles, where width and height are equal
    let threshold = if width == height {
        circle::diameter_to_threshold(width)
    } else {
        b * a
    };

    (Size::new(a, b), threshold)
}

/// Uses the ellipse equation b^2 * x^2 + a^2 * y^2 - a^2 * b^2 to return a value signifying whether
/// a given point lies inside (`true`) or outside (`false`) an ellipse centered around `(0, 0)` with
/// width and height defined by the `size` parameter.
pub(crate) fn is_point_inside_ellipse(size: Size, point: Point, threshold: u32) -> bool {
    let Size {
        width: a,
        height: b,
    } = size;

    let Point { x, y } = point;

    let x = x.pow(2) as u32;
    let y = y.pow(2) as u32;

    // Special case for circles, where width and height are equal
    if a == b {
        x + y < threshold
    } else {
        b * x + a * y < threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::{Drawable, Pixel},
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, ContainsPoint, Primitive},
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

    fn test_ellipse(size: Size, style: PrimitiveStyle<BinaryColor>, pattern: &[&str]) {
        let mut display = MockDisplay::new();

        Ellipse::new(Point::new(0, 0), size)
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::from_pattern(pattern));
    }

    fn test_circles(style: PrimitiveStyle<BinaryColor>) {
        for diameter in 0..50 {
            let top_left = Point::new_equal(style.stroke_width_i32());

            let mut expected = MockDisplay::new();
            Circle::new(top_left, diameter)
                .into_styled(style)
                .draw(&mut expected)
                .unwrap();

            let mut display = MockDisplay::new();
            Ellipse::new(top_left, Size::new(diameter, diameter))
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

            assert_eq!(display, expected, "diameter = {}", diameter);
        }
    }

    #[test]
    fn contains() {
        let mut expected = MockDisplay::new();
        let ellipse = Ellipse::new(Point::zero(), Size::new(40, 20));

        let mut display = MockDisplay::new();

        Rectangle::new(Point::zero(), Size::new(40, 20))
            .points()
            .filter(|p| ellipse.contains(*p))
            .map(|p| Pixel(p, BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        ellipse
            .points()
            .map(|p| Pixel(p, BinaryColor::On))
            .draw(&mut expected)
            .unwrap();

        assert_eq!(display, expected);
    }

    #[test]
    fn ellipse_equals_circle_fill() {
        test_circles(PrimitiveStyle::with_fill(BinaryColor::On));
    }

    #[test]
    fn ellipse_equals_circle_stroke_1px() {
        test_circles(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    }

    #[test]
    fn ellipse_equals_circle_stroke_10px() {
        test_circles(PrimitiveStyle::with_stroke(BinaryColor::On, 10));
    }

    #[test]
    fn filled_ellipse() {
        #[rustfmt::skip]
        test_ellipse(Size::new(20, 10), PrimitiveStyle::with_fill(BinaryColor::On), &[
            "      ########      ",
            "   ##############   ",
            " ################## ",
            "####################",
            "####################",
            "####################",
            "####################",
            " ################## ",
            "   ##############   ",
            "      ########      ",
        ],);
    }

    #[test]
    fn thick_stroke_glitch() {
        test_ellipse(
            Size::new(11, 21),
            PrimitiveStyleBuilder::new()
                .stroke_width(10)
                .stroke_color(BinaryColor::On)
                .stroke_alignment(StrokeAlignment::Inside)
                .fill_color(BinaryColor::Off)
                .build(),
            &[
                "    ###    ",
                "   #####   ",
                "  #######  ",
                " ######### ",
                " ######### ",
                " ######### ",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                " ######### ",
                " ######### ",
                " ######### ",
                "  #######  ",
                "   #####   ",
                "    ###    ",
            ],
        );
    }

    #[test]
    fn thin_stroked_ellipse() {
        #[rustfmt::skip]
        test_ellipse(Size::new(20, 10), PrimitiveStyle::with_stroke(BinaryColor::On, 1), &[
            "      ########      ",
            "   ###        ###   ",
            " ##              ## ",
            "##                ##",
            "#                  #",
            "#                  #",
            "##                ##",
            " ##              ## ",
            "   ###        ###   ",
            "      ########      ",
        ],);
    }

    #[test]
    fn fill_and_stroke() {
        test_ellipse(
            Size::new(20, 10),
            PrimitiveStyleBuilder::new()
                .stroke_width(3)
                .stroke_color(BinaryColor::Off)
                .stroke_alignment(StrokeAlignment::Inside)
                .fill_color(BinaryColor::On)
                .build(),
            &[
                "      ........      ",
                "   ..............   ",
                " .................. ",
                ".....##########.....",
                "...##############...",
                "...##############...",
                ".....##########.....",
                " .................. ",
                "   ..............   ",
                "      ........      ",
            ],
        );
    }

    #[test]
    fn translate() {
        let mut display = MockDisplay::new();

        Ellipse::new(Point::new(4, 6), Size::new(5, 8))
            .translate(Point::new(3, 5))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "            ",
                "        ### ",
                "        # # ",
                "       #   #",
                "       #   #",
                "       #   #",
                "       #   #",
                "        # # ",
                "        ### ",
            ])
        );
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Ellipse::with_center(CENTER, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Ellipse::with_center(CENTER, SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Ellipse::with_center(CENTER, SIZE - Size::new(4, 4))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            )
            .draw(&mut display_outside)
            .unwrap();

        assert_eq!(display_center, display_inside);
        assert_eq!(display_center, display_outside);
    }
}
