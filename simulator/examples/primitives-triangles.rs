//! # Example: Triangles
//!
//! Shows multiple triangles with different properties

use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::*, primitives::Triangle, style::PrimitiveStyle,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

const PAD: i32 = 10;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(512, 128));

    let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    // No straight lines
    Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64))
        .translate(Point::new(PAD, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // Flat top
    Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0))
        .translate(Point::new(64 + PAD, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // Flat left
    Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30))
        .translate(Point::new((64 + PAD) * 2, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // Flat bottom
    Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 3, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // Flat right
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 4, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // Draw filled above stroke, should not be visible
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, 0))
        .into_styled(style)
        .draw(&mut display)?;

    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, 0))
        .into_styled(style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Triangles", &output_settings).show_static(&display);

    Ok(())
}
