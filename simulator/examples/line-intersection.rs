use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{line::Intersection, *},
    style::PrimitiveStyle,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};

fn draw(e2: Point, display: &mut SimulatorDisplay<Rgb888>) {
    display.clear(Rgb888::BLACK).unwrap();

    let s1 = Point::new(10, 100);
    let e1 = Point::new(130, 40);

    let s2 = Point::new(50, 10);
    // let s2 = Point::new(200, 200) - e2;
    // let s2 = e1;

    // First static line
    let l1 = Line::new(s1, e1);

    l1.into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(display)
        .unwrap();

    // Second movable line
    let l2 = Line::new(s2, e2);

    l2.into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
        .draw(display)
        .unwrap();

    match l1.line_intersection(&l2) {
        Intersection::Point { point, .. } => {
            // Draw intersection point
            Circle::with_center(point, 5)
                .into_styled(PrimitiveStyle::with_fill(Rgb888::MAGENTA))
                .draw(display)
                .unwrap();
        }
        Intersection::Colinear => {
            //
        }
    }
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Rounded rectangle debugger", &output_settings);

    let mut end_point = Point::new(20, 20);

    let mut mouse_down = false;

    draw(end_point, &mut display);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    end_point = point;
                }
                // SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                //     _ => (),
                // },
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        end_point = point;
                    }
                }
                _ => {}
            }

            draw(end_point, &mut display);
        }
    }

    Ok(())
}
