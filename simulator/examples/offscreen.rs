use embedded_graphics::{
    pixelcolor::BinaryColor, prelude::*, primitives::Rectangle, style::PrimitiveStyle,
};
use embedded_graphics_simulator::{SimulatorDisplay, WindowBuilder};

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = SimulatorDisplay::new(Size::new(32, 32));

    // Outline
    Rectangle::new(Point::new(0, 0), Point::new(16, 16))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
        .translate(Point::new(-8, -8))
        .draw(&mut display)?;

    let mut window = WindowBuilder::new(&display)
        .title("Offscreen")
        .scale(4)
        .build();

    window.show_static(&display);

    Ok(())
}
