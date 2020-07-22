//! # Example: Primitive fill styles
//!
//! This example demonstrates the different fill and stroke styles available for primitives.

use core::convert::Infallible;
use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, CornerRadii, Ellipse, Rectangle, RoundedRectangle, Triangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};

static CIRCLE_SIZE: i32 = 65;
static ELLIPSE_SIZE: Size = Size::new(90, 65);

fn draw_shapes<T>(target: &mut T, style: PrimitiveStyle<BinaryColor>) -> Result<(), T::Error>
where
    T: DrawTarget<Color = BinaryColor>,
{
    Circle::new(Point::new(0, 0), CIRCLE_SIZE as u32)
        .into_styled(style)
        .draw(target)?;

    Rectangle::new(Point::new(96, 0), Size::new(64, 64))
        .into_styled(style)
        .draw(target)?;

    Triangle::new(Point::new(32, 0), Point::new(0, 64), Point::new(64, 64))
        .translate(Point::new(96 * 2, 0))
        .into_styled(style)
        .draw(target)?;

    Ellipse::new(Point::new(96 * 3, 0), ELLIPSE_SIZE)
        .into_styled(style)
        .draw(target)?;

    RoundedRectangle::new(
        Rectangle::new(Point::new(32, 0), Size::new(64, 64)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new(96 * 4, 0))
    .into_styled(style)
    .draw(target)
}

fn main() -> Result<(), Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(512, 128));

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

    draw_shapes(&mut display, stroke)?;
    draw_shapes(
        &mut display.translated(Point::new(16, 16)),
        stroke_off_fill_on,
    )?;
    draw_shapes(
        &mut display.translated(Point::new(32, 32)),
        stroke_off_fill_off,
    )?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Filled primitives", &output_settings).show_static(&display);

    Ok(())
}
