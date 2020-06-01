//! # Example: Rectangle intersection
//!
//! This example draws the intersection of two base rectangles (red and green) in blue.

use embedded_graphics::{pixelcolor::Rgb888, prelude::*, primitives::*, style::PrimitiveStyle};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn draw(
    base_rectangle: Rectangle,
    moving_rectangle: Rectangle,
    display: &mut SimulatorDisplay<Rgb888>,
) {
    display.clear(Rgb888::BLACK).unwrap();

    base_rectangle
        .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
        .draw(display)
        .unwrap();

    moving_rectangle
        .into_styled(PrimitiveStyle::with_fill(Rgb888::GREEN))
        .draw(display)
        .unwrap();

    let intersection = base_rectangle.intersection(&moving_rectangle);

    if intersection.size != Size::zero() {
        intersection
            .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
            .draw(display)
            .unwrap()
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Intersection", &output_settings);

    let mut mouse_down = false;

    let base_rectangle = Rectangle::with_corners(Point::new(20, 20), Point::new(100, 100));
    let mut moving_rectangle = Rectangle::with_corners(Point::new(80, 80), Point::new(150, 150));

    draw(base_rectangle, moving_rectangle, &mut display);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    moving_rectangle = Rectangle::with_corners(point, Point::new(100, 100));
                }
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        moving_rectangle = Rectangle::with_corners(point, Point::new(150, 150));
                    }
                }
                _ => {}
            }

            draw(base_rectangle, moving_rectangle, &mut display);
        }
    }

    Ok(())
}
