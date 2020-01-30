use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::*, primitives::Triangle, style::PrimitiveStyle,
};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

const PAD: i32 = 10;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(512, 128));

    let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

    // no straight lines
    Triangle::new(Point::new(0, 0), Point::new(64, 10), Point::new(15, 64))
        .translate(Point::new(PAD, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // flat top
    Triangle::new(Point::new(5, 0), Point::new(30, 64), Point::new(64, 0))
        .translate(Point::new(64 + PAD, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // flat left
    Triangle::new(Point::new(0, 0), Point::new(0, 64), Point::new(64, 30))
        .translate(Point::new((64 + PAD) * 2, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // flat bottom
    Triangle::new(Point::new(22, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 3, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // flat right
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 4, 0))
        .into_styled(style)
        .draw(&mut display)?;

    // draw filled above stroke, should not be visible
    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, 0))
        .into_styled(style)
        .draw(&mut display)?;

    Triangle::new(Point::new(0, 22), Point::new(64, 0), Point::new(64, 64))
        .translate(Point::new((64 + PAD) * 5, 0))
        .into_styled(style)
        .draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("Triangles")
        .scale(2)
        .build();
    window.show_static(&display);

    Ok(())
}
