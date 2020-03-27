//! # Example: Chaining
//!
//! Demonstrate the chaining abilities of embedded graphics iterators
//!
//! This example displays the same end result as the `hello-world.rs` example, but does it using a
//! single chained iterator.

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 3);
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);

    let yoffset = 10;

    let bottom_right = Point::zero() + display.size() - Point::new(1, 1);

    // Draw an 3px wide outline around the display.
    Rectangle::new(Point::zero(), bottom_right)
        .into_styled(thick_stroke)
        .into_iter()
        .chain(
            // Draw a triangle.
            Triangle::new(
                Point::new(16, 16 + yoffset),
                Point::new(16 + 16, 16 + yoffset),
                Point::new(16 + 8, yoffset),
            )
            .into_styled(thin_stroke)
            .into_iter(),
        )
        .chain(
            // Draw a filled square
            Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
                .into_styled(fill)
                .into_iter(),
        )
        .chain(
            // Draw a square with a 3px wide stroke.
            Circle::new(Point::new(96, yoffset + 8), 8)
                .into_styled(thick_stroke)
                .into_iter(),
        )
        .chain({
            // Draw centered text.
            let text = "embedded-graphics";
            let width = text.len() as i32 * 6;

            Text::new(text, Point::new(64 - width / 2, 40))
                .into_styled(text_style)
                .into_iter()
        })
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();

    Window::new("Chained drawing", &output_settings).show_static(&display);

    Ok(())
}
