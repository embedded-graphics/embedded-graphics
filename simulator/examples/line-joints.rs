use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::*,
    style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

/// Squared normal of a line
///
/// https://stackoverflow.com/a/1243676/383609
fn normal_norm(start: Point, end: Point, mul: u32) -> Point {
    use integer_sqrt::IntegerSquareRoot;

    let dx = start.x - end.x;
    let dy = start.y - end.y;

    // Left side normal
    // let x = dy;
    // let y = -dx;

    // Right side normal
    let x = -dy;
    let y = dx;

    // Point::new(x, y)

    let len = (dx.pow(2) + dy.pow(2)).integer_sqrt();
    let normalised = (Point::new(x, y) * mul as i32) / len;

    normalised
}

fn crosshair(point: Point, color: Rgb888, display: &mut SimulatorDisplay<Rgb888>) {
    let radius = Size::new(4, 4);

    Line::new(point - radius.x_axis(), point + radius.x_axis())
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)
        .unwrap();

    Line::new(point - radius.y_axis(), point + radius.y_axis())
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)
        .unwrap();
}

fn empty_crosshair(point: Point, color: Rgb888, display: &mut SimulatorDisplay<Rgb888>) {
    let radius = Size::new_equal(4);
    let inner_radius = Size::new_equal(2);

    Line::new(point - radius.x_axis(), point - inner_radius.x_axis())
        .points()
        .chain(Line::new(point + radius.x_axis(), point + inner_radius.x_axis()).points())
        .chain(Line::new(point - radius.y_axis(), point - inner_radius.y_axis()).points())
        .chain(Line::new(point + radius.y_axis(), point + inner_radius.y_axis()).points())
        .map(|p| Pixel(p, color))
        .draw(display)
        .unwrap();
}

fn draw(end_point: Point, width: u32, display: &mut SimulatorDisplay<Rgb888>) {
    display.clear(Rgb888::BLACK).unwrap();

    let mid = Point::new(100, 100);
    let start = Point::new(50, 100);

    // let fixed_normal = normal_norm(start, mid, width);

    // First static line
    // Line::new(start, mid)
    //     .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, width))
    //     .draw(display)
    //     .unwrap();

    // Second movable line
    let l = Line::new(mid, end_point);

    l.into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, width))
        .draw(display)
        .unwrap();

    let (ext_l, ext_r) = l.extents(width as i32);

    // let delta_ext_l = l.start - ext_l_start;
    // let delta_ext_r = l.start - ext_r_start;

    // let ext_l_end = l.end - delta_ext_l;
    // let ext_r_end = l.end - delta_ext_r;

    // let ext_l = Line::new(ext_l_start, ext_l_end);
    // let ext_r = Line::new(ext_r_start, ext_r_end);

    // ext_l
    //     .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
    //     .draw(display)
    //     .unwrap();

    // ext_r
    //     .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLUE, 1))
    //     .draw(display)
    //     .unwrap();

    empty_crosshair(ext_l.start, Rgb888::RED, display);
    empty_crosshair(ext_l.end, Rgb888::RED, display);
    empty_crosshair(ext_r.start, Rgb888::new(255, 127, 0), display);
    empty_crosshair(ext_r.end, Rgb888::new(255, 127, 0), display);

    // let movable_normal = normal_norm(mid, end_point, width);

    // // Normal of movable line
    // let l = Line::new(mid, mid + movable_normal);

    // let exts = l.extents(width as i32);

    // l.into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
    //     .draw(display)
    //     .unwrap();

    // // Movable and fixed normal added
    // Line::new(mid, mid + movable_normal + fixed_normal)
    //     .into_styled(PrimitiveStyle::with_stroke(Rgb888::CYAN, 1))
    //     .draw(display)
    //     .unwrap();
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Rounded rectangle debugger", &output_settings);

    let mut end_point = Point::new(20, 20);
    let mut width = 1u32;

    let mut mouse_down = false;

    draw(end_point, width, &mut display);

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    end_point = point;
                }
                SimulatorEvent::KeyDown { keycode, .. } => match keycode {
                    Keycode::Up => width += 1,
                    Keycode::Down => width = width.saturating_sub(1),
                    _ => (),
                },
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        end_point = point;
                    }
                }
                _ => {}
            }

            draw(end_point, width, &mut display);
        }
    }

    Ok(())
}
