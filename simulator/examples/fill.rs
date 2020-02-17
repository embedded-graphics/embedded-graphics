use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

static CIRCLE_SIZE: i32 = 32;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(304, 128));

    let stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    let stroke_off_fill_off = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::Off)
        .stroke_width(1)
        .fill_color(BinaryColor::Off)
        .build();

    let stroke_off_fill_on = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::Off)
        .stroke_width(1)
        .fill_color(BinaryColor::On)
        .build();

    Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
        .into_styled(stroke)
        .draw(&mut display)?;

    Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
        .translate(Point::new(16, 16))
        .into_styled(stroke_off_fill_on)
        .draw(&mut display)?;

    Circle::new(Point::new(CIRCLE_SIZE, CIRCLE_SIZE), CIRCLE_SIZE as u32)
        .translate(Point::new(CIRCLE_SIZE, CIRCLE_SIZE))
        .into_styled(stroke_off_fill_off)
        .draw(&mut display)?;

    Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new(96, 0))
        .into_styled(stroke)
        .draw(&mut display)?;

    Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new(96 + 16, 16))
        .into_styled(stroke_off_fill_on)
        .draw(&mut display)?;

    Rectangle::new(Point::new(0, 0), Point::new(64, 64))
        .translate(Point::new(96 + 32, 32))
        .into_styled(stroke_off_fill_off)
        .draw(&mut display)?;

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2, 0))
        .into_styled(stroke)
        .draw(&mut display)?;

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2 + 16, 16))
        .into_styled(stroke_off_fill_on)
        .draw(&mut display)?;

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2 + 32, 32))
        .into_styled(stroke_off_fill_off)
        .draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("Filled primitives")
        .scale(2)
        .build();
    window.show_static(&display);

    Ok(())
}
