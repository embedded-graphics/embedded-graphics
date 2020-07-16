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

    let fixed = Line::new(start, mid);

    // let tstyle = PrimitiveStyle::with_stroke(Rgb888::RED, 1);
    let tstyle = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        .fill_color(Rgb888::GREEN)
        .build();

    let l = Line::new(mid, end_point);

    // {
    //     // Draw first static line
    //     fixed
    //         .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, width))
    //         .draw(display)
    //         .unwrap();

    //     // Draw second movable line
    //     l.into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, width))
    //         .draw(display)
    //         .unwrap();
    // }

    // The maximum length of a mitered corner. After this point, the corner should be come beveled
    let min_len_sq = fixed.length_squared().component_min(l.length_squared()) / 2;

    // Left and right edges of thick second segment
    let (ext_l, ext_r) = l.extents(width as i32);
    // Left and right edges of thick first segment
    let (fixed_ext_l, fixed_ext_r) = fixed.extents(width as i32);

    if let (Some((l_intersection, l_on_lines)), Some((r_intersection, r_on_lines))) = (
        ext_l.intersection(&fixed_ext_l),
        ext_r.intersection(&fixed_ext_r),
    ) {
        let is_degenerate = {
            let first_segment_start_cap = Line::new(fixed_ext_l.start, fixed_ext_r.start);

            let is_degenerate = first_segment_start_cap
                .intersection(&ext_l)
                .filter(|(_, on_both)| *on_both)
                .or_else(|| {
                    first_segment_start_cap
                        .intersection(&ext_r)
                        .filter(|(_, on_both)| *on_both)
                })
                .is_some();

            first_segment_start_cap
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
                .draw(display)
                .unwrap();

            ext_r
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1))
                .draw(display)
                .unwrap();

            ext_l
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLUE, 1))
                .draw(display)
                .unwrap();

            is_degenerate
        };

        // Degenerate debugger
        if is_degenerate {
            Rectangle::new(Point::zero(), Size::new_equal(5))
                .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                .draw(display)
                .unwrap();
        }

        // Fixed (first) line triangles
        {
            Triangle::new(fixed_ext_l.start, l_intersection, r_intersection)
                .into_styled(tstyle)
                .draw(display)
                .unwrap();
            Triangle::new(fixed_ext_l.start, fixed_ext_r.start, r_intersection)
                .into_styled(tstyle)
                .draw(display)
                .unwrap();
        }

        // Movable (second) line triangles
        {
            Triangle::new(ext_l.end, l_intersection, r_intersection)
                .into_styled(tstyle)
                .draw(display)
                .unwrap();
            Triangle::new(ext_l.end, ext_r.end, r_intersection)
                .into_styled(tstyle)
                .draw(display)
                .unwrap();
        }

        Circle::with_center(l_intersection, 5)
            .into_styled(if l_on_lines {
                PrimitiveStyle::with_fill(Rgb888::YELLOW)
            } else {
                PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1)
            })
            .draw(display)
            .unwrap();

        Circle::with_center(r_intersection, 5)
            .into_styled(if r_on_lines {
                PrimitiveStyle::with_fill(Rgb888::new(0, 127, 255))
            } else {
                PrimitiveStyle::with_stroke(Rgb888::new(0, 127, 255), 1)
            })
            .draw(display)
            .unwrap();
    }

    // empty_crosshair(ext_l.start, Rgb888::RED, display);
    // empty_crosshair(ext_l.end, Rgb888::RED, display);
    // empty_crosshair(ext_r.start, Rgb888::new(255, 127, 0), display);
    // empty_crosshair(ext_r.end, Rgb888::new(255, 127, 0), display);
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
