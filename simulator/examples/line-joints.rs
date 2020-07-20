use embedded_graphics::{
    fonts::*, pixelcolor::Rgb888, prelude::*, primitives::line::Side, primitives::*, style::*,
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
    let start = Point::new(50, 130);

    let fixed = Line::new(start, mid);

    // let tstyle = PrimitiveStyle::with_stroke(Rgb888::RED, 1);
    let tstyle = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        .fill_color(Rgb888::GREEN)
        .build();

    let linestyle = PrimitiveStyle::with_stroke(Rgb888::GREEN, width);

    let l = Line::new(mid, end_point);

    // Miter length limit is the line width (but squared to avoid sqrt() costs)
    let miter_limit = (width * 2).pow(2);

    // Left and right edges of thick second segment
    let (ext_l, ext_r) = l.extents(width as i32);
    // Left and right edges of thick first segment
    let (fixed_ext_l, fixed_ext_r) = fixed.extents(width as i32);

    if let (Some((l_intersection, l_on_lines)), Some((r_intersection, r_on_lines))) = (
        ext_l.intersection(&fixed_ext_l),
        ext_r.intersection(&fixed_ext_r),
    ) {
        let (is_degenerate_l, is_degenerate_r) = {
            let first_segment_start_cap = Line::new(fixed_ext_l.start, fixed_ext_r.start);
            let second_segment_end_cap = Line::new(ext_l.end, ext_r.end);

            let is_degenerate_l = first_segment_start_cap
                .intersection(&ext_l)
                .filter(|(_, on_both)| *on_both)
                .is_some()
                || second_segment_end_cap
                    .intersection(&fixed_ext_l)
                    .filter(|(_, on_both)| *on_both)
                    .is_some();

            let is_degenerate_r = first_segment_start_cap
                .intersection(&ext_r)
                .filter(|(_, on_both)| *on_both)
                .is_some()
                || second_segment_end_cap
                    .intersection(&fixed_ext_r)
                    .filter(|(_, on_both)| *on_both)
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

            (is_degenerate_l, is_degenerate_r)
        };

        let is_degenerate = is_degenerate_l || is_degenerate_r;

        // The side that will have the miter/bevel on it
        let outer_side = if l_on_lines || is_degenerate_l {
            Side::Right
        } else if r_on_lines || is_degenerate_r {
            Side::Left
        } else {
            Side::Right
        };

        let (inside_intersection, outside_intersection) = match outer_side {
            Side::Right => (l_intersection, r_intersection),
            Side::Left => (r_intersection, l_intersection),
        };

        // Distance from midpoint to miter end point
        let miter_length_squared = Line::new(mid, outside_intersection).length_squared();

        // Degenerate debugger
        Text::new(
            &format!(
                "L: {} R: {}: over m lim: {}",
                if is_degenerate_l { "X" } else { "-" },
                if is_degenerate_r { "X" } else { "-" },
                if miter_length_squared <= miter_limit {
                    "N"
                } else {
                    "Y"
                },
            ),
            Point::zero(),
        )
        .into_styled(TextStyle::new(Font6x8, Rgb888::RED))
        .draw(display)
        .unwrap();

        // Normal line: not degenerate (overlapping) and miter length is less than thickness. In
        // this case, draw the full miter as it won't stretch out really far.
        if !is_degenerate {
            if miter_length_squared <= miter_limit {
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
            } else {
                match outer_side {
                    Side::Left => {
                        // Fixed (first) line triangles
                        {
                            // 1
                            Triangle::new(
                                fixed_ext_l.start,
                                fixed_ext_r.start,
                                inside_intersection,
                            )
                            .into_styled(tstyle)
                            .draw(display)
                            .unwrap();

                            // 2
                            Triangle::new(fixed_ext_l.start, fixed_ext_l.end, inside_intersection)
                                .into_styled(tstyle)
                                .draw(display)
                                .unwrap();
                        }

                        // Bevel/joint fill (3)
                        Triangle::new(fixed_ext_l.end, inside_intersection, ext_l.start)
                            .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                            .draw(display)
                            .unwrap();

                        // Movable (second) line triangles
                        {
                            // 4
                            Triangle::new(ext_l.start, ext_l.end, inside_intersection)
                                .into_styled(tstyle)
                                .draw(display)
                                .unwrap();

                            // 5
                            Triangle::new(ext_l.end, ext_r.end, inside_intersection)
                                .into_styled(tstyle)
                                .draw(display)
                                .unwrap();
                        }
                    }
                    Side::Right => {
                        // Fixed (first) line triangles
                        {
                            // 1
                            Triangle::new(
                                fixed_ext_l.start,
                                fixed_ext_r.start,
                                inside_intersection,
                            )
                            .into_styled(tstyle)
                            .draw(display)
                            .unwrap();

                            // 2
                            Triangle::new(fixed_ext_r.start, fixed_ext_r.end, inside_intersection)
                                .into_styled(tstyle)
                                .draw(display)
                                .unwrap();
                        }

                        // Bevel/joint fill (3)
                        Triangle::new(fixed_ext_r.end, inside_intersection, ext_r.start)
                            .into_styled(PrimitiveStyle::with_fill(Rgb888::MAGENTA))
                            .draw(display)
                            .unwrap();

                        // Movable (second) line triangles
                        {
                            // 4
                            Triangle::new(ext_r.start, ext_r.end, inside_intersection)
                                .into_styled(tstyle)
                                .draw(display)
                                .unwrap();

                            // 5
                            Triangle::new(ext_l.end, ext_r.end, inside_intersection)
                                .into_styled(tstyle)
                                .draw(display)
                                .unwrap();
                        }
                    }
                }
            }
        }
        // Line segments overlap (degenerate). Draw normal but overlapping thick lines with extra
        // triangle for bevel cap.
        else {
            // Fixed (first) line
            fixed.into_styled(linestyle).draw(display).unwrap();

            // Moving (second) line
            l.into_styled(linestyle).draw(display).unwrap();

            // Bevel cap
            match outer_side {
                Side::Left => Triangle::new(fixed_ext_l.end, mid, ext_l.start)
                    .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                    .draw(display)
                    .unwrap(),
                Side::Right => Triangle::new(fixed_ext_r.end, mid, ext_r.start)
                    .into_styled(PrimitiveStyle::with_fill(Rgb888::MAGENTA))
                    .draw(display)
                    .unwrap(),
            }
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
    // Lines are colinear (probably)
    else {
        Text::new("Colinear!", Point::zero())
            .into_styled(TextStyle::new(Font6x8, Rgb888::RED))
            .draw(display)
            .unwrap();

        fixed.into_styled(linestyle).draw(display).unwrap();
        l.into_styled(linestyle).draw(display).unwrap();
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
    let mut width = 15u32;

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
