//! Graphics primitives

use crate::geometry::Dimensions;
use crate::pixelcolor::PixelColor;

pub mod circle;
pub mod line;
pub mod rectangle;
pub mod triangle;

pub use self::circle::Circle;
pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::triangle::Triangle;
use crate::style::{PrimitiveStyle, Styled};

/// Primitive trait
pub trait Primitive: Dimensions {
    /// Converts this primitive into a `Styled`.
    fn into_styled<C>(self, style: PrimitiveStyle<C>) -> Styled<Self, PrimitiveStyle<C>>
    where
        C: PixelColor,
        Self: Sized,
    {
        Styled::new(self, style)
    }
}

/// Create a [`Circle`](./primitives/circle/struct.Circle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::pixelcolor::Rgb565;
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egcircle, primitives::Circle};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let line_circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!((10, 20), 30);
/// let line_circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(Point::new(10, 20), 30);
/// let filled_circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
///     (10, 20),
///     30,
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke_color` map to the properties in the [`PrimitiveStyle`] struct.
/// For example, the following code makes two identical circles:
///
/// [`PrimitiveStyle`]: style/struct.PrimitiveStyle.html
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egcircle, pixelcolor::Rgb565, primitives::Circle};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
///     (10, 20),
///     30,
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
///     Point::new(10, 20),
///     30,
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
///
/// let mut style = PrimitiveStyle::fill(Rgb565::GREEN);
/// style.stroke_color = Some(Rgb565::RED);
///
/// let circle: Styled<Circle, PrimitiveStyle<Rgb565>> = Circle::new(Point::new(10, 20), 30)
///     .into_styled(style);
/// ```
#[macro_export]
macro_rules! egcircle {
    ($center:expr, $r:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        use $crate::style::PrimitiveStyle;

        #[allow(unused_mut)]
        let mut style = PrimitiveStyle::default();
        $( style.$style_key = $style_value; )*

        $crate::primitives::Circle::new($crate::geometry::Point::from($center), $r)
            .into_styled(style)
    }};
}

/// Create a [`Line`](./primitives/line/struct.Line.html) with optional styling using a
/// convenient macro.
///
/// Note that only the `stroke` property has any effect on lines currently.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egline, pixelcolor::Rgb565, primitives::Line};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!((10, 20), (30, 40));
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(Point::new(10, 20), Point::new(30, 40));
/// let stroke_line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!((10, 20), (30, 40), stroke_color = Some(Rgb565::BLUE));
/// ```
///
/// Style properties like `stroke_color` map to the properties in the [`PrimitiveStyle`] struct.
/// For example, the following code makes two identical lines:
///
/// [`PrimitiveStyle`]: style/struct.PrimitiveStyle.html
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egline, pixelcolor::Rgb565, primitives::Line};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
///     Point::new(10, 20),
///     Point::new(30, 40),
///     stroke_color = Some(Rgb565::BLUE),
///     fill_color = Some(Rgb565::YELLOW)
/// );
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
///     (10, 20),
///     (30, 40),
///     stroke_color = Some(Rgb565::BLUE),
///     fill_color = Some(Rgb565::YELLOW)
/// );
///
/// let mut style = PrimitiveStyle::fill(Rgb565::YELLOW);
/// style.stroke_color = Some(Rgb565::BLUE);
///
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> =
///     Line::new(Point::new(10, 20), Point::new(30, 40))
///     .into_styled(style);
/// ```
#[macro_export]
macro_rules! egline {
    ($start:expr, $end:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        use $crate::style::PrimitiveStyle;

        #[allow(unused_mut)]
        let mut style = PrimitiveStyle::default();
        $( style.$style_key = $style_value; )*

        $crate::primitives::Line::new(
            $crate::geometry::Point::from($start),
            $crate::geometry::Point::from($end)
        )
            .into_styled(style)
    }};
}

/// Create a [`Rectangle`](./primitives/rectangle/struct.Rectangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egrectangle, pixelcolor::Rgb565, primitives::Rectangle};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let empty_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!((10, 20), (30, 40));
/// let empty_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(Point::new(10, 20), Point::new(30, 40));
/// let filled_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     (10, 20),
///     (30, 40),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke_color` map to the properties in the [`PrimitiveStyle`] struct.
/// For example, the following code makes two identical rectangles:
///
/// [`PrimitiveStyle`]: style/struct.PrimitiveStyle.html
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egrectangle, pixelcolor::Rgb565, primitives::Rectangle};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let rectangle: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     (10, 20),
///     (30, 40),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let rectangle: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     Point::new(10, 20),
///     Point::new(30, 40),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
///
/// let mut style = PrimitiveStyle::fill(Rgb565::GREEN);
/// style.stroke_color = Some(Rgb565::RED);
///
/// let rectangle: Styled<Rectangle, PrimitiveStyle<Rgb565>> = Rectangle::new(Point::new(10, 20), Point::new(30, 40))
///     .into_styled(style);
/// ```
#[macro_export]
macro_rules! egrectangle {
    ($top_left:expr, $bottom_right:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        use $crate::style::PrimitiveStyle;

        #[allow(unused_mut)]
        let mut style = PrimitiveStyle::default();
        $( style.$style_key = $style_value; )*

        $crate::primitives::Rectangle::new(
            $crate::geometry::Point::from($top_left),
            $crate::geometry::Point::from($bottom_right)
        )
            .into_styled(style)
    }};
}

/// Create a [`Triangle`](./primitives/triangle/struct.Triangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egtriangle, pixelcolor::Rgb565, primitives::Triangle};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let empty_triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!((10, 20), (30, 40), (50, 60));
/// let empty_triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> =
///     egtriangle!(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60));
/// let filled_triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!(
///     (10, 20),
///     (30, 40),
///     (50, 60),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// ```
///
/// Style properties like `stroke_color` map to the properties in the [`PrimitiveStyle`] struct.
/// For example, the following code makes two identical triangles:
///
/// [`PrimitiveStyle`]: style/struct.PrimitiveStyle.html
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egtriangle, pixelcolor::Rgb565, primitives::Triangle};
/// use embedded_graphics::style::{PrimitiveStyle, Styled};
///
/// let triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!(
///     (10, 20),
///     (30, 40),
///     (50, 60),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
///
/// let mut style = PrimitiveStyle::fill(Rgb565::GREEN);
/// style.stroke_color = Some(Rgb565::RED);
///
/// let triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> =
///     Triangle::new(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60))
///         .into_styled(style);
/// ```
#[macro_export]
macro_rules! egtriangle {
    ($p1:expr, $p2:expr, $p3:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        use $crate::style::PrimitiveStyle;

        #[allow(unused_mut)]
        let mut style = PrimitiveStyle::default();
        $( style.$style_key = $style_value; )*

        $crate::primitives::Triangle::new(
            $crate::geometry::Point::from($p1),
            $crate::geometry::Point::from($p2),
            $crate::geometry::Point::from($p3)
        )
            .into_styled(style)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::pixelcolor::{Rgb565, RgbColor};
    use crate::style::PrimitiveStyle;

    #[test]
    fn circle() {
        let _c: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(Point::new(10, 20), 30);
        let _c: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!((10, 20), 30);
        let _c: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
            (10, 20),
            30,
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
    }

    #[test]
    fn line() {
        let _l: Styled<Line, PrimitiveStyle<Rgb565>> =
            egline!(Point::new(10, 20), Point::new(30, 40));
        let _l: Styled<Line, PrimitiveStyle<Rgb565>> = egline!((10, 20), (30, 40));
        let _l: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
            (10, 20),
            (30, 40),
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
    }

    #[test]
    fn rectangle() {
        let _r: Styled<Rectangle, PrimitiveStyle<Rgb565>> =
            egrectangle!(Point::new(10, 20), Point::new(30, 40));
        let _r: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!((10, 20), (30, 40));
        let _r: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
            (10, 20),
            (30, 40),
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
    }

    #[test]
    fn triangle() {
        let _t: Styled<Triangle, PrimitiveStyle<Rgb565>> =
            egtriangle!(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60));
        let _t: Styled<Triangle, PrimitiveStyle<Rgb565>> =
            egtriangle!((10, 20), (30, 40), (50, 60));
        let _t: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!(
            (10, 20),
            (30, 40),
            (50, 60),
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
    }
}
