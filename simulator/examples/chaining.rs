//! Demonstrate the chaining abilities of embedded graphics iterators

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Line},
    style::{PrimitiveStyle, TextStyle},
};
use embedded_graphics_simulator::{BinaryColorTheme, SimulatorDisplay, WindowBuilder};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(129, 129));
    let mut objects = Circle::new(Point::new(64, 64), 64)
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .into_iter()
        .chain(
            &Line::new(Point::new(64, 64), Point::new(0, 64))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
        )
        .chain(
            &Line::new(Point::new(64, 64), Point::new(80, 80))
                .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
        )
        .chain(
            &Text::new("Hello World!", Point::new(5, 50))
                .into_styled(TextStyle::new(Font6x8, BinaryColor::On)),
        );

    objects.draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("Chained drawing")
        .theme(BinaryColorTheme::OledBlue)
        .build();

    window.show_static(&display);

    Ok(())
}
