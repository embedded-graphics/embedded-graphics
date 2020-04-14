//! # Example: Primitive stroke styles
//!
//! This example demonstrates the different stroke styles available for primitives.

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Ellipse, Line, Rectangle, Triangle},
    style::PrimitiveStyle,
};
use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(420, 256));

    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let medium_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 10);

    let triangle = Triangle::new(Point::new(0, 64), Point::new(64, 0), Point::new(64, 64));
    let rectangle =
        Rectangle::new(Point::new(0, 0), Size::new(64, 64)).translate(Point::new(64 + PADDING, 0));
    let line = Line::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new((64 + PADDING) * 2, 0));
    let circle = Circle::new(Point::new(0, 0), 64).translate(Point::new((64 + PADDING) * 3, 0));
    let ellipse = Ellipse::new(Point::new(0, 0), Size::new(96, 64))
        .translate(Point::new((64 + PADDING) * 4, 0));

    circle
        .into_styled(thin_stroke)
        .into_iter()
        .chain(rectangle.into_styled(thin_stroke).into_iter())
        .chain(line.into_styled(thin_stroke).into_iter())
        .chain(triangle.into_styled(thin_stroke).into_iter())
        .chain(ellipse.into_styled(thin_stroke).into_iter())
        .draw(&mut display)?;

    let offset = Point::new(0, 64 + PADDING);
    circle
        .translate(offset)
        .into_styled(medium_stroke)
        .into_iter()
        .chain(
            rectangle
                .translate(offset)
                .into_styled(medium_stroke)
                .into_iter(),
        )
        .chain(
            line.translate(offset)
                .into_styled(medium_stroke)
                .into_iter(),
        )
        .chain(
            triangle
                .translate(offset)
                .into_styled(medium_stroke)
                .into_iter(),
        )
        .chain(
            ellipse
                .translate(offset)
                .into_styled(medium_stroke)
                .into_iter(),
        )
        .draw(&mut display)?;

    let offset = Point::new(0, (64 + PADDING) * 2);
    circle
        .translate(offset)
        .into_styled(thick_stroke)
        .into_iter()
        .chain(
            rectangle
                .translate(offset)
                .into_styled(thick_stroke)
                .into_iter(),
        )
        .chain(line.translate(offset).into_styled(thick_stroke).into_iter())
        .chain(
            triangle
                .translate(offset)
                .into_styled(thick_stroke)
                .into_iter(),
        )
        .chain(
            ellipse
                .translate(offset)
                .into_styled(thick_stroke)
                .into_iter(),
        )
        .draw(&mut display)?;

    Window::new("Strokes", &OutputSettings::default()).show_static(&display);

    Ok(())
}
