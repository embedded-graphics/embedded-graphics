//! # Example: Primitive hit detection
//!
//! Uses the implementations of `ContainsPoint` for primitives to change the color of a shape when
//! the cursor is inside it.

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, CornerRadii, Ellipse, Line, Rectangle, RoundedRectangle, Triangle},
    style::PrimitiveStyle,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

const PADDING: i32 = 16;

fn update(
    cursor: Point,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), std::convert::Infallible> {
    let style = PrimitiveStyle::with_fill(Rgb888::RED);
    let inside_style = PrimitiveStyle::with_fill(Rgb888::GREEN);
    let cursor_style = PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1);

    let triangle = Triangle::new(Point::new(0, 64), Point::new(64, 0), Point::new(64, 64));
    let rectangle =
        Rectangle::new(Point::new(0, 0), Size::new(64, 64)).translate(Point::new(64 + PADDING, 0));
    let circle = Circle::new(Point::new(0, 0), 64).translate(Point::new((64 + PADDING) * 2, 0));
    let rounded_rectangle = RoundedRectangle::new(
        Rectangle::new(Point::new(0, 0), Size::new(64, 64)),
        CornerRadii::new(Size::new(16, 16)),
    )
    .translate(Point::new((64 + PADDING) * 3, 0));
    let ellipse = Ellipse::new(Point::new(0, 0), Size::new(96, 64))
        .translate(Point::new((64 + PADDING) * 4, 0));

    display.clear(Rgb888::BLACK)?;

    triangle
        .into_styled(if triangle.contains(cursor) {
            inside_style
        } else {
            style
        })
        .draw(display)?;

    rectangle
        .into_styled(if rectangle.contains(cursor) {
            inside_style
        } else {
            style
        })
        .draw(display)?;

    circle
        .into_styled(if circle.contains(cursor) {
            inside_style
        } else {
            style
        })
        .draw(display)?;

    rounded_rectangle
        .into_styled(if rounded_rectangle.contains(cursor) {
            inside_style
        } else {
            style
        })
        .draw(display)?;

    ellipse
        .into_styled(if ellipse.contains(cursor) {
            inside_style
        } else {
            style
        })
        .draw(display)?;

    let crosshair_size = Size::new(10, 10);

    // Draw cursor on top of all other shapes
    Line::new(
        cursor - crosshair_size.x_axis(),
        cursor + crosshair_size.x_axis(),
    )
    .into_styled(cursor_style)
    .into_iter()
    .chain(
        &Line::new(
            cursor - crosshair_size.y_axis(),
            cursor + crosshair_size.y_axis(),
        )
        .into_styled(cursor_style),
    )
    .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(430, 128));

    let mut cursor = Point::zero();

    let output_settings = OutputSettingsBuilder::new().scale(2).build();

    let mut window = Window::new("Hit detection", &output_settings);

    update(cursor, &mut display).unwrap();

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseMove { point, .. } => {
                    cursor = point;
                }
                _ => {}
            }

            update(cursor, &mut display).unwrap();
        }
    }

    Ok(())
}
