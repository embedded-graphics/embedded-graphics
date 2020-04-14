//! The ellipse primitive

use crate::{
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    pixelcolor::PixelColor,
    primitives::{ContainsPoint, Primitive, Rectangle, Styled},
    style::PrimitiveStyle,
    transform::Transform,
    DrawTarget,
};

/// Ellipse primitive
///
/// # Examples
///
/// The [macro examples](../../macro.egellipse.html) make for more concise code.
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
/// # let mut display = MockDisplay::default();
///
/// // Ellipse with 1 pixel wide white stroke with top-left point at (10, 20) with a size of (30, 50)
/// Ellipse::new(Point::new(10, 20), Size::new(30, 50))
///     .into_styled(PrimitiveStyle::with_stroke(Rgb565::WHITE, 1))
///     .draw(&mut display)?;
///
/// // Ellipse with styled stroke and fill with top-left point at (50, 20) with a size of (40, 30)
/// let style = PrimitiveStyleBuilder::new()
///     .stroke_color(Rgb565::RED)
///     .stroke_width(3)
///     .fill_color(Rgb565::GREEN)
///     .build();
///
/// Ellipse::new(Point::new(50, 20), Size::new(40, 30))
///     .into_styled(style)
///     .draw(&mut display)?;
///
/// // Ellipse with blue fill and no stroke with a translation applied
/// Ellipse::new(Point::new(10, 20), Size::new(20, 40))
///     .translate(Point::new(65, 35))
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
        let offset = size.saturating_sub(Size::new(1, 1)) / 2;
        let top_left = center - offset;

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
        let radius = self.size.saturating_sub(Size::new(1, 1));

        self.top_left * 2 + radius
    }

    fn bounding_box(&self) -> Rectangle {
        Rectangle::new(self.top_left, self.bottom_right())
    }
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
    fn top_left(&self) -> Point {
        self.top_left
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    fn size(&self) -> Size {
        self.size
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

/// Iterator over all points inside the ellipse
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct Points {
    iter: super::rectangle::Points,
    center_2x: Point,
    size_sq: Size,
    threshold: u32,
}

impl Points {
    fn new(ellipse: &Ellipse) -> Self {
        let (size_sq, threshold) = compute_threshold(ellipse.size);

        Self {
            iter: ellipse.bounding_box().points(),
            center_2x: ellipse.center_2x(),
            size_sq,
            threshold,
        }
    }

    fn empty() -> Self {
        Self {
            iter: Rectangle::new(Point::zero(), Point::zero()).points(),
            center_2x: Point::zero(),
            size_sq: Size::zero(),
            threshold: 0,
        }
    }
}

impl Iterator for Points {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            if is_point_inside_ellipse(self.size_sq, point * 2 - self.center_2x, self.threshold) {
                return Some(point);
            }
        }

        None
    }
}

/// Pixel iterator for each pixel in the ellipse border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledEllipseIterator<C>
where
    C: PixelColor,
{
    iter: Points,
    outer_color: Option<C>,
    inner_size_sq: Size,
    inner_color: Option<C>,
    center: Point,
    threshold: u32,
}

impl<C> StyledEllipseIterator<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Ellipse, PrimitiveStyle<C>>) -> Self {
        // Always use a stroke width of 0 if no stroke color was set.
        let stroke_width = styled.style.effective_stroke_width();

        let outer_size = styled.primitive.size;
        let inner_size = outer_size.saturating_sub(Size::new(2 * stroke_width, 2 * stroke_width));
        let iter = if !styled.style.is_transparent() {
            Points::new(&styled.primitive)
        } else {
            Points::empty()
        };

        let (inner_size_sq, threshold) = compute_threshold(inner_size);

        Self {
            iter,
            outer_color: styled.style.stroke_color,
            inner_size_sq,
            inner_color: styled.style.fill_color,
            center: styled.primitive.center_2x(),
            threshold,
        }
    }
}

impl<C> Iterator for StyledEllipseIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            let inside_border = is_point_inside_ellipse(
                self.inner_size_sq,
                point * 2 - self.center,
                self.threshold,
            );

            let color = if inside_border {
                self.inner_color
            } else {
                self.outer_color
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_ellipse(self)
    }
}

fn diameter_to_threshold(diameter: u32) -> u32 {
    if diameter <= 4 {
        diameter.pow(2) - diameter / 2
    } else {
        diameter.pow(2)
    }
}

fn compute_threshold(size: Size) -> (Size, u32) {
    let Size { width, height } = size;

    let a = width.pow(2);
    let b = height.pow(2);

    // Special case for circles, where width and height are equal
    let threshold = if width == height {
        diameter_to_threshold(width)
    } else {
        b * a
    };

    (Size::new(a, b), threshold)
}

/// Uses the ellipse equation b^2 * x^2 + a^2 * y^2 - a^2 * b^2 to return a value signifying whether
/// a given point lies inside (`true`) or outside (`false`) an ellipse centered around `(0, 0)` with
/// width and height defined by the `size` parameter.
fn is_point_inside_ellipse(size: Size, point: Point, threshold: u32) -> bool {
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

impl<'a, C> IntoIterator for &'a Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledEllipseIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledEllipseIterator::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        mock_display::MockDisplay, pixelcolor::BinaryColor, primitives::Circle,
        style::PrimitiveStyleBuilder,
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
            let mut expected = MockDisplay::new();
            Circle::new(Point::new(0, 0), diameter)
                .into_styled(style)
                .draw(&mut expected)
                .unwrap();

            let mut display = MockDisplay::new();
            Ellipse::new(Point::new(0, 0), Size::new(diameter, diameter))
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

        Rectangle::new(Point::zero(), Point::new(40, 20))
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
    fn circles_points() {
        for diameter in 0..50 {
            let circle_points = Circle::new(Point::new(0, 0), diameter).points();

            let ellipse_points =
                Ellipse::new(Point::new(0, 0), Size::new(diameter, diameter)).points();

            assert!(circle_points.eq(ellipse_points), "diameter = {}", diameter);
        }
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
}
