//! Graphics primitives

use crate::geometry::Dimensions;

pub mod circle;
pub mod line;
pub mod rectangle;
pub mod triangle;

/// Primitive trait
pub trait Primitive: Dimensions {}

pub use self::circle::Circle;
pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::triangle::Triangle;

/// Create a [`Circle`](./primitives/circle/struct.Circle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::pixelcolor::Rgb565;
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egcircle, primitives::Circle, style::Style};
///
/// let line_circle: Circle<Rgb565> = egcircle!((10, 20), 30);
/// let line_circle: Circle<Rgb565> = egcircle!(Point::new(10, 20), 30);
/// let filled_circle: Circle<Rgb565> = egcircle!(
///     (10, 20),
///     30,
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let default_style: Circle<Rgb565> = egcircle!((10, 20), 30, style = Style::default());
/// ```
///
/// Style properties like `stroke_color` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical circles:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egcircle, pixelcolor::Rgb565, primitives::Circle, style::Style};
///
/// let circle: Circle<Rgb565> = egcircle!(
///     (10, 20),
///     30,
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let circle: Circle<Rgb565> = egcircle!(
///     Point::new(10, 20),
///     30,
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let circle: Circle<Rgb565> = Circle::new(Point::new(10, 20), 30)
///     .stroke_color(Some(Rgb565::RED))
///     .fill_color(Some(Rgb565::GREEN));
/// ```
#[macro_export]
macro_rules! egcircle {
    ($center:expr, $r:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Circle::new($crate::geometry::Point::from($center), $r)
            $( .$style_key($style_value) )*
    }};
}

/// Create a [`Line`](./primitives/line/struct.Line.html) with optional styling using a
/// convenient macro.
///
/// Note that only the `stroke` property has any effect on lines currently.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egline, pixelcolor::Rgb565, primitives::Line, style::Style};
///
/// let line: Line<Rgb565> = egline!((10, 20), (30, 40));
/// let line: Line<Rgb565> = egline!(Point::new(10, 20), Point::new(30, 40));
/// let stroke_line: Line<Rgb565> = egline!((10, 20), (30, 40), stroke_color = Some(Rgb565::BLUE));
/// ```
///
/// Style properties like `stroke` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical lines:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egline, pixelcolor::Rgb565, primitives::Line, style::Style};
///
/// let Line: Line<Rgb565> = egline!(
///     Point::new(10, 20),
///     Point::new(30, 40),
///     stroke_color = Some(Rgb565::BLUE),
///     fill_color = Some(Rgb565::YELLOW)
/// );
/// let Line: Line<Rgb565> = egline!(
///     (10, 20),
///     (30, 40),
///     stroke_color = Some(Rgb565::BLUE),
///     fill_color = Some(Rgb565::YELLOW)
/// );
/// let Line: Line<Rgb565> = Line::new(Point::new(10, 20), Point::new(30, 40))
///     .stroke_color(Some(Rgb565::BLUE))
///     .fill_color(Some(Rgb565::YELLOW));
/// ```
#[macro_export]
macro_rules! egline {
    ($start:expr, $end:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Line::new(
            $crate::geometry::Point::from($start),
            $crate::geometry::Point::from($end)
        )
            $( .$style_key($style_value) )*
    }};
}

/// Create a [`Rectangle`](./primitives/rectangle/struct.Rectangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egrectangle, pixelcolor::Rgb565, primitives::Rectangle, style::Style};
///
/// let empty_rect: Rectangle<Rgb565> = egrectangle!((10, 20), (30, 40));
/// let empty_rect: Rectangle<Rgb565> = egrectangle!(Point::new(10, 20), Point::new(30, 40));
/// let filled_rect: Rectangle<Rgb565> = egrectangle!(
///     (10, 20),
///     (30, 40),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let rect_default_style: Rectangle<Rgb565> =
///     egrectangle!((10, 20), (30, 40), style = Style::default());
/// ```
///
/// Style properties like `stroke_color` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical rectangles:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egrectangle, pixelcolor::Rgb565, primitives::Rectangle, style::Style};
///
/// let Rectangle: Rectangle<Rgb565> = egrectangle!(
///     (10, 20),
///     (30, 40),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let Rectangle: Rectangle<Rgb565> = egrectangle!(
///     Point::new(10, 20),
///     Point::new(30, 40),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let Rectangle: Rectangle<Rgb565> = Rectangle::new(Point::new(10, 20), Point::new(30, 40))
///     .stroke_color(Some(Rgb565::RED))
///     .fill_color(Some(Rgb565::GREEN));
/// ```
#[macro_export]
macro_rules! egrectangle {
    ($top_left:expr, $bottom_right:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Rectangle::new(
            $crate::geometry::Point::from($top_left),
            $crate::geometry::Point::from($bottom_right)
        )
            $( .$style_key($style_value) )*
    }};
}

/// Create a [`Triangle`](./primitives/triangle/struct.Triangle.html) with optional styling using a
/// convenient macro.
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egtriangle, pixelcolor::Rgb565, primitives::Triangle, style::Style};
///
/// let empty_triangle: Triangle<Rgb565> = egtriangle!((10, 20), (30, 40), (50, 60));
/// let empty_triangle: Triangle<Rgb565> =
///     egtriangle!(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60));
/// let filled_triangle: Triangle<Rgb565> = egtriangle!(
///     (10, 20),
///     (30, 40),
///     (50, 60),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let triangle_default_style: Triangle<Rgb565> =
///     egtriangle!((10, 20), (30, 40), (50, 60), style = Style::default());
/// ```
///
/// Style properties like `stroke_color` map to the method calls on the
/// [`WithStyle`](style/trait.WithStyle.html) trait. For example, the following code makes two
/// identical triangles:
///
/// ```rust
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{egtriangle, pixelcolor::Rgb565, primitives::Triangle, style::Style};
///
/// let Triangle: Triangle<Rgb565> = egtriangle!(
///     (10, 20),
///     (30, 40),
///     (50, 60),
///     stroke_color = Some(Rgb565::RED),
///     fill_color = Some(Rgb565::GREEN)
/// );
/// let Triangle: Triangle<Rgb565> =
///     Triangle::new(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60))
///         .stroke_color(Some(Rgb565::RED))
///         .fill_color(Some(Rgb565::GREEN));
/// ```
#[macro_export]
macro_rules! egtriangle {
    ($p1:expr, $p2:expr, $p3:expr $(, $style_key:ident = $style_value:expr )* $(,)?) => {{
        #[allow(unused_imports)]
        use $crate::style::WithStyle;
        $crate::primitives::Triangle::new(
            $crate::geometry::Point::from($p1),
            $crate::geometry::Point::from($p2),
            $crate::geometry::Point::from($p3)
        )
            $( .$style_key($style_value) )*
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Point;
    use crate::pixelcolor::{Rgb565, RgbColor};
    use crate::style::Style;

    #[test]
    fn circle() {
        let _c: Circle<Rgb565> = egcircle!(Point::new(10, 20), 30);
        let _c: Circle<Rgb565> = egcircle!((10, 20), 30);
        let _c: Circle<Rgb565> = egcircle!(
            (10, 20),
            30,
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
        let _c: Circle<Rgb565> = egcircle!((10, 20), 30, style = Style::default());
    }

    #[test]
    fn line() {
        let _l: Line<Rgb565> = egline!(Point::new(10, 20), Point::new(30, 40));
        let _l: Line<Rgb565> = egline!((10, 20), (30, 40));
        let _l: Line<Rgb565> = egline!(
            (10, 20),
            (30, 40),
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
        let _l: Line<Rgb565> = egline!((10, 20), (30, 40), style = Style::default());
    }

    #[test]
    fn rectangle() {
        let _r: Rectangle<Rgb565> = egrectangle!(Point::new(10, 20), Point::new(30, 40));
        let _r: Rectangle<Rgb565> = egrectangle!((10, 20), (30, 40));
        let _r: Rectangle<Rgb565> = egrectangle!(
            (10, 20),
            (30, 40),
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
        let _r: Rectangle<Rgb565> = egrectangle!((10, 20), (30, 40), style = Style::default());
    }

    #[test]
    fn triangle() {
        let _t: Triangle<Rgb565> =
            egtriangle!(Point::new(10, 20), Point::new(30, 40), Point::new(50, 60));
        let _t: Triangle<Rgb565> = egtriangle!((10, 20), (30, 40), (50, 60));
        let _t: Triangle<Rgb565> = egtriangle!(
            (10, 20),
            (30, 40),
            (50, 60),
            stroke_color = Some(Rgb565::RED),
            fill_color = Some(Rgb565::GREEN)
        );
        let _t: Triangle<Rgb565> =
            egtriangle!((10, 20), (30, 40), (50, 60), style = Style::default());
    }
}
