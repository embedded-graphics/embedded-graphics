//! Graphics primitives

use crate::{geometry::Dimensions, pixelcolor::PixelColor};

pub mod circle;
pub mod line;
pub mod rectangle;
pub mod triangle;

pub use self::{circle::Circle, line::Line, rectangle::Rectangle, triangle::Triangle};
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
/// use embedded_graphics::{
///     egcircle,
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::Circle,
///     primitive_style,
///     style::{PrimitiveStyle, Styled},
/// };
///
/// let line_circle: Styled<Circle, PrimitiveStyle<Rgb565>> =
///     egcircle!(center = (10, 20), radius = 30);
/// let line_circle: Styled<Circle, PrimitiveStyle<Rgb565>> =
///     egcircle!(center = Point::new(10, 20), radius = 30);
/// let filled_circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
///     center = (10, 20),
///     radius = 30,
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
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
/// use embedded_graphics::primitive_style;
/// use embedded_graphics::{egcircle, pixelcolor::Rgb565, primitives::Circle};
/// use embedded_graphics::style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled};
///
/// let circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
///     center = (10, 20),
///     radius = 30,
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
/// );
/// let circle: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
///     center = Point::new(10, 20),
///     radius = 30,
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
/// );
///
/// let style = PrimitiveStyleBuilder::new()
///     .fill_color(Rgb565::GREEN)
///     .stroke_color(Rgb565::RED)
///     .stroke_width(1)
///     .build();
///
/// let circle: Styled<Circle, PrimitiveStyle<Rgb565>> =
///     Circle::new(Point::new(10, 20), 30).into_styled(style);
/// ```
#[macro_export]
macro_rules! egcircle {
    (center = $center:expr, radius = $r:expr $(,)?) => {{
        $crate::egcircle!(
            center = $center,
            radius = $r,
            style = $crate::style::PrimitiveStyle::default()
        )
    }};
    (center = $center:expr, radius = $r:expr, style = $style:expr $(,)?) => {{
        $crate::primitives::Circle::new($crate::geometry::Point::from($center), $r)
            .into_styled($style)
    }};
}

/// Create a [`Line`](./primitives/line/struct.Line.html) with optional styling using a
/// convenient macro.
///
/// Note that only the `stroke` property has any effect on lines currently.
///
/// ```rust
/// use embedded_graphics::{
///     egline,
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::Line,
///     primitive_style,
///     style::{PrimitiveStyle, Styled},
/// };
///
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(start = (10, 20), end = (30, 40));
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> =
///     egline!(start = Point::new(10, 20), end = Point::new(30, 40));
/// let stroke_line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
///     start = (10, 20),
///     end = (30, 40),
///     style = primitive_style!(stroke_color = Rgb565::BLUE)
/// );
/// ```
///
/// Style properties like `stroke_color` map to the properties in the [`PrimitiveStyle`] struct.
/// For example, the following code makes two identical lines:
///
/// [`PrimitiveStyle`]: style/struct.PrimitiveStyle.html
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled};
/// use embedded_graphics::{primitive_style, egline, pixelcolor::Rgb565, primitives::Line};
///
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
///     start = Point::new(10, 20),
///     end = Point::new(30, 40),
///     style = primitive_style!(
///         stroke_color = Rgb565::BLUE,
///         fill_color = Rgb565::YELLOW
///     )
/// );
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
///     start = (10, 20),
///     end = (30, 40),
///     style = primitive_style!(
///         stroke_color = Rgb565::BLUE,
///         fill_color = Rgb565::YELLOW
///     )
/// );
///
/// let style = PrimitiveStyleBuilder::new()
///     .fill_color(Rgb565::YELLOW)
///     .stroke_color(Rgb565::BLUE)
///     .stroke_width(1)
///     .build();
///
/// let line: Styled<Line, PrimitiveStyle<Rgb565>> =
///     Line::new(Point::new(10, 20), Point::new(30, 40)).into_styled(style);
/// ```
#[macro_export]
macro_rules! egline {
    (start = $start:expr, end = $end:expr $(,)?) => {{
        $crate::egline!(
            start = $start,
            end = $end,
            style = $crate::style::PrimitiveStyle::default()
        )
    }};
    (start = $start:expr, end = $end:expr, style = $style:expr $(,)?) => {{
        $crate::primitives::Line::new(
            $crate::geometry::Point::from($start),
            $crate::geometry::Point::from($end),
        )
        .into_styled($style)
    }};
}

/// Create a [`Rectangle`](./primitives/rectangle/struct.Rectangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::{
///     egrectangle,
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::Rectangle,
///     primitive_style,
///     style::{PrimitiveStyle, Styled},
/// };
///
/// let empty_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> =
///     egrectangle!(top_left = (10, 20), bottom_right = (30, 40));
/// let empty_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     top_left = Point::new(10, 20),
///     bottom_right = Point::new(30, 40)
/// );
/// let filled_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     top_left = (10, 20),
///     bottom_right = (30, 40),
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
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
/// use embedded_graphics::{primitive_style, egrectangle, pixelcolor::Rgb565, primitives::Rectangle};
/// use embedded_graphics::style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled};
///
/// let rectangle: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     top_left = (10, 20),
///     bottom_right = (30, 40),
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
/// );
/// let rectangle: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
///     top_left = Point::new(10, 20),
///     bottom_right = Point::new(30, 40),
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
/// );
///
/// let style = PrimitiveStyleBuilder::new()
///     .fill_color(Rgb565::GREEN)
///     .stroke_color(Rgb565::RED)
///     .stroke_width(1)
///     .build();
///
/// let rectangle: Styled<Rectangle, PrimitiveStyle<Rgb565>> =
///     Rectangle::new(Point::new(10, 20), Point::new(30, 40)).into_styled(style);
/// ```
#[macro_export]
macro_rules! egrectangle {
    (top_left = $top_left:expr, bottom_right = $bottom_right:expr $(,)?) => {{
        $crate::egrectangle!(
            top_left = $top_left,
            bottom_right = $bottom_right,
            style = $crate::style::PrimitiveStyle::default()
        )
    }};
    (top_left = $top_left:expr, bottom_right = $bottom_right:expr, style = $style:expr $(,)?) => {{
        $crate::primitives::Rectangle::new(
            $crate::geometry::Point::from($top_left),
            $crate::geometry::Point::from($bottom_right),
        )
        .into_styled($style)
    }};
}

/// Create a [`Triangle`](./primitives/triangle/struct.Triangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::{
///     egtriangle,
///     pixelcolor::Rgb565,
///     prelude::*,
///     primitives::Triangle,
///     primitive_style,
///     style::{PrimitiveStyle, Styled},
/// };
///
/// let empty_triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> =
///     egtriangle!(points = [(10, 20), (30, 40), (50, 60)]);
/// let empty_triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> =
///     egtriangle!(points = [(10, 20), (30, 40), (50, 60)]);
/// let filled_triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!(
///     points = [(10, 20), (30, 40), (50, 60)],
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
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
/// use embedded_graphics::{egtriangle, primitive_style, pixelcolor::Rgb565, primitives::Triangle};
/// use embedded_graphics::style::{PrimitiveStyle, PrimitiveStyleBuilder, Styled};
///
/// let triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!(
///     points = [(10, 20), (30, 40), (50, 60)],
///     style = primitive_style!(
///         stroke_color = Rgb565::RED,
///         fill_color = Rgb565::GREEN
///     )
/// );
///
/// let style = PrimitiveStyleBuilder::new()
///     .fill_color(Rgb565::GREEN)
///     .stroke_color(Rgb565::RED)
///     .stroke_width(1)
///     .build();
///
/// let triangle: Styled<Triangle, PrimitiveStyle<Rgb565>> =
///     Triangle::new(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60))
///         .into_styled(style);
/// ```
#[macro_export]
macro_rules! egtriangle {
    (points = $points:expr $(,)?) => {{
        $crate::egtriangle!(
            points = $points,
            style = $crate::style::PrimitiveStyle::default()
        )
    }};
    (points = $points:expr, style = $style:expr $(,)?) => {{
        $crate::primitives::Triangle::from_points($points).into_styled($style)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Point,
        pixelcolor::{Rgb565, RgbColor},
        primitive_style,
        style::PrimitiveStyle,
    };

    #[test]
    fn circle() {
        let _c: Styled<Circle, PrimitiveStyle<Rgb565>> =
            egcircle!(center = Point::new(10, 20), radius = 30);
        let _c: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(center = (10, 20), radius = 30);
        let _c: Styled<Circle, PrimitiveStyle<Rgb565>> = egcircle!(
            center = (10, 20),
            radius = 30,
            style = primitive_style!(stroke_color = Rgb565::RED, fill_color = Rgb565::GREEN),
        );
    }

    #[test]
    fn line() {
        let _l: Styled<Line, PrimitiveStyle<Rgb565>> =
            egline!(start = Point::new(10, 20), end = Point::new(30, 40),);
        let _l: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(start = (10, 20), end = (30, 40));
        let _l: Styled<Line, PrimitiveStyle<Rgb565>> = egline!(
            start = (10, 20),
            end = (30, 40),
            style = primitive_style!(stroke_color = Rgb565::RED, fill_color = Rgb565::GREEN),
        );
    }

    #[test]
    fn rectangle() {
        let _r: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
            top_left = Point::new(10, 20),
            bottom_right = Point::new(30, 40),
        );
        let _r: Styled<Rectangle, PrimitiveStyle<Rgb565>> =
            egrectangle!(top_left = (10, 20), bottom_right = (30, 40),);
        let _r: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
            top_left = (10, 20),
            bottom_right = (30, 40),
            style = primitive_style!(stroke_color = Rgb565::RED, fill_color = Rgb565::GREEN)
        );
    }

    #[test]
    fn triangle() {
        let _t: Styled<Triangle, PrimitiveStyle<Rgb565>> =
            egtriangle!(points = [Point::new(10, 20), Point::new(30, 40), Point::new(50, 60)],);
        let _t: Styled<Triangle, PrimitiveStyle<Rgb565>> =
            egtriangle!(points = [(10, 20), (30, 40), (50, 60)]);
        let _t: Styled<Triangle, PrimitiveStyle<Rgb565>> = egtriangle!(
            points = [(10, 20), (30, 40), (50, 60)],
            style = primitive_style!(stroke_color = Rgb565::RED, fill_color = Rgb565::GREEN)
        );
    }
}
