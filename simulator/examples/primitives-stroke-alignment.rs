//! # Example: Stroke alignment
//!
//! This example demonstrates the different stroke alignments available for primitives.
//!
//! The stroke alignment property only applies to closed shapes and is currently
//! not supported for triangles.

use core::convert::Infallible;
use embedded_graphics::{
    fonts::{Font6x12, Text},
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Circle, Ellipse, Rectangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, TextStyle},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::{thread, time::Duration};

const SIZE: u32 = 64;
const PADDING: u32 = 32;

fn draw_primitives(
    display: &mut SimulatorDisplay<Rgb888>,
    position: Point,
    style: PrimitiveStyle<Rgb888>,
) -> Result<(), Infallible> {
    let row_offset = Size::new(0, SIZE + PADDING);

    Rectangle::new(position, Size::new(SIZE, SIZE))
        .into_styled(style.clone())
        .draw(display)?;

    Circle::new(position + row_offset, SIZE)
        .into_styled(style.clone())
        .draw(display)?;

    Ellipse::new(position + row_offset * 2, Size::new(SIZE, SIZE + 16))
        .into_styled(style)
        .draw(display)
}

fn update(
    display: &mut SimulatorDisplay<Rgb888>,
    show_shape_outline: bool,
) -> Result<(), Infallible> {
    display.clear(Rgb888::BLACK)?;

    let stroke_inside = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::CYAN)
        .stroke_width(15)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();

    let stroke_center = PrimitiveStyleBuilder::from(&stroke_inside)
        .stroke_alignment(StrokeAlignment::Center)
        .build();

    let stroke_outside = PrimitiveStyleBuilder::from(&stroke_inside)
        .stroke_alignment(StrokeAlignment::Outside)
        .build();

    let shape_outline_style = PrimitiveStyle::with_stroke(Rgb888::RED, 1);

    let mut position = Point::zero() + Size::new(PADDING, PADDING + 16);
    for &style in [stroke_inside, stroke_center, stroke_outside].iter() {
        draw_primitives(display, position, style)?;

        if show_shape_outline {
            draw_primitives(display, position, shape_outline_style)?;
        }

        position += Size::new(SIZE + PADDING, 0);
    }

    let text_offset = Point::new(0, 8) + Size::new(PADDING, 0);
    let column_offset = Size::new(SIZE + PADDING, 0);
    Text::new("Inside", text_offset)
        .into_styled(TextStyle::new(Font6x12, Rgb888::WHITE))
        .draw(display)?;

    Text::new("Center\n(Default)", text_offset + column_offset)
        .into_styled(TextStyle::new(Font6x12, Rgb888::WHITE))
        .draw(display)?;

    Text::new("Outside", text_offset + column_offset * 2)
        .into_styled(TextStyle::new(Font6x12, Rgb888::WHITE))
        .draw(display)?;

    Text::new(
        "Click to toggle shape outline",
        Point::new(PADDING as i32, 330),
    )
    .into_styled(TextStyle::new(Font6x12, Rgb888::WHITE))
    .draw(display)
}

fn main() -> Result<(), Infallible> {
    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    let mut window = Window::new("Stroke Alignment", &output_settings);

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(350, 350));
    let mut show_shape_outline = true;

    'main_loop: loop {
        update(&mut display, show_shape_outline)?;
        window.update(&display);

        'event_loop: loop {
            for event in window.events() {
                match event {
                    SimulatorEvent::Quit => break 'main_loop,
                    SimulatorEvent::MouseButtonDown { .. } => {
                        show_shape_outline = !show_shape_outline;
                        break 'event_loop;
                    }
                    _ => {}
                }
            }

            thread::sleep(Duration::from_millis(20));
        }
    }

    Ok(())
}
