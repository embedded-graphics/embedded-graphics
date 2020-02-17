//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code

use embedded_graphics::{
    egcircle, egrectangle, egtriangle, pixelcolor::BinaryColor, prelude::*, primitive_style,
};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

static CIRCLE_SIZE: i32 = 32;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(384, 128));

    egcircle!(
        center = (CIRCLE_SIZE, CIRCLE_SIZE),
        radius = CIRCLE_SIZE as u32,
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1,)
    )
    .draw(&mut display)?;

    egcircle!(
        center = (CIRCLE_SIZE, CIRCLE_SIZE),
        radius = CIRCLE_SIZE as u32,
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::On,
        )
    )
    .translate(Point::new(16, 16))
    .draw(&mut display)?;

    egcircle!(
        center = (CIRCLE_SIZE, CIRCLE_SIZE),
        radius = CIRCLE_SIZE as u32,
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::Off
        )
    )
    .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE))
    .draw(&mut display)?;

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (64, 64),
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1,)
    )
    .translate(Point::new(96, 0))
    .draw(&mut display)?;

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (64, 64),
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::On
        )
    )
    .translate(Point::new(96 + 16, 16))
    .draw(&mut display)?;

    egrectangle!(
        top_left = (0, 0),
        bottom_right = (64, 64),
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::Off
        )
    )
    .translate(Point::new(96 + 32, 32))
    .draw(&mut display)?;

    egtriangle!(
        points = [(32, 0), (0, 64), (64, 64)],
        style = primitive_style!(stroke_color = BinaryColor::On, stroke_width = 1,)
    )
    .translate(Point::new(96 * 2, 0))
    .draw(&mut display)?;

    egtriangle!(
        points = [(32, 0), (0, 64), (64, 64)],
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::On,
        )
    )
    .translate(Point::new(96 * 2 + 16, 16))
    .draw(&mut display)?;

    egtriangle!(
        points = [(32, 0), (0, 64), (64, 64)],
        style = primitive_style!(
            stroke_color = BinaryColor::Off,
            stroke_width = 1,
            fill_color = BinaryColor::Off,
        )
    )
    .translate(Point::new(96 * 2 + 32, 32))
    .draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("Filled primitives using macros")
        .scale(2)
        .build();
    window.show_static(&display);

    Ok(())
}
