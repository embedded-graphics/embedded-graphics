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

fn draw(
    start: Point,
    mid: Point,
    end: Point,
    width: u32,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let fixed = Line::new(start, mid);
    let l = Line::new(mid, end);

    // let tstyle = PrimitiveStyle::with_stroke(Rgb888::RED, 1);
    let tstyle = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        .fill_color(Rgb888::GREEN)
        .build();

    let linestyle = PrimitiveStyle::with_stroke(Rgb888::GREEN, width);

    // Miter length limit is dobule the line width (but squared to avoid sqrt() costs)
    let miter_limit = (width * 2).pow(2);

    // Left and right edges of thick first segment
    let (fixed_ext_l, fixed_ext_r) = fixed.extents(width as i32);
    // Left and right edges of thick second segment
    let (ext_l, ext_r) = l.extents(width as i32);

    if let (Some((l_intersection, l_on_lines)), Some((r_intersection, r_on_lines))) = (
        ext_l.intersection(&fixed_ext_l),
        ext_r.intersection(&fixed_ext_r),
    ) {
        let (outer_side, is_self_intersecting) = {
            let first_segment_start_edge = Line::new(fixed_ext_l.start, fixed_ext_r.start);
            let second_segment_end_edge = Line::new(ext_l.end, ext_r.end);

            let self_intersection_l = first_segment_start_edge
                .intersection(&ext_l)
                .filter(|(_, on_both)| *on_both)
                .or_else(|| {
                    second_segment_end_edge
                        .intersection(&fixed_ext_l)
                        .filter(|(_, on_both)| *on_both)
                });

            let self_intersection_r = first_segment_start_edge
                .intersection(&ext_r)
                .filter(|(_, on_both)| *on_both)
                .or_else(|| {
                    second_segment_end_edge
                        .intersection(&fixed_ext_r)
                        .filter(|(_, on_both)| *on_both)
                });

            let (self_intersection_l, self_intersection_r) =
                (self_intersection_l.is_some(), self_intersection_r.is_some());

            // The side that will have the miter/bevel on it, relative to first line
            let outer_side = if l_on_lines || self_intersection_l {
                Side::Right
            } else if r_on_lines || self_intersection_r {
                Side::Left
            } else {
                // Default to some randomly chosen side when lines are colinear
                Side::Right
            };

            (outer_side, self_intersection_l || self_intersection_r)
        };

        let (
            outside_intersection,
            inside_intersection,
            ext_outside,
            ext_inside,
            fixed_outside,
            fixed_inside,
        ) = match outer_side {
            Side::Right => (
                r_intersection,
                l_intersection,
                ext_r,
                ext_l,
                fixed_ext_r,
                fixed_ext_l,
            ),
            Side::Left => (
                l_intersection,
                r_intersection,
                ext_l,
                ext_r,
                fixed_ext_l,
                fixed_ext_r,
            ),
        };

        // Distance from midpoint to miter end point
        let miter_length_squared = Line::new(mid, outside_intersection).length_squared();

        // // Degenerate debugger
        // Text::new(
        //     &format!(
        //         "L: {} R: {}: over m lim: {}",
        //         if is_degenerate_l { "X" } else { "-" },
        //         if is_degenerate_r { "X" } else { "-" },
        //         if miter_length_squared <= miter_limit {
        //             "N"
        //         } else {
        //             "Y"
        //         },
        //     ),
        //     Point::zero(),
        // )
        // .into_styled(TextStyle::new(Font6x8, Rgb888::RED))
        // .draw(display)?;

        // Normal line: not degenerate (overlapping) and miter length is less than thickness. In
        // this case, draw the full miter as it won't stretch out really far.
        if !is_self_intersecting {
            if miter_length_squared <= miter_limit {
                // Fixed (first) line triangles
                {
                    Triangle::new(
                        fixed_outside.start,
                        outside_intersection,
                        inside_intersection,
                    )
                    .into_styled(tstyle)
                    .draw(display)?;
                    Triangle::new(fixed_outside.start, inside_intersection, fixed_inside.start)
                        .into_styled(tstyle)
                        .draw(display)?;
                }

                // Movable (second) line triangles
                {
                    Triangle::new(outside_intersection, ext_outside.end, inside_intersection)
                        .into_styled(tstyle)
                        .draw(display)?;
                    Triangle::new(ext_outside.end, ext_inside.end, inside_intersection)
                        .into_styled(tstyle)
                        .draw(display)?;
                }
            } else {
                // 1 (fill in beginning of first line)
                Triangle::new(fixed_outside.start, fixed_inside.start, inside_intersection)
                    .into_styled(tstyle)
                    .draw(display)?;

                // 2 (adjacent to bevel)
                Triangle::new(fixed_outside.start, fixed_outside.end, inside_intersection)
                    .into_styled(tstyle)
                    .draw(display)?;

                // 3 (bevel)
                Triangle::new(fixed_outside.end, ext_outside.start, inside_intersection)
                    .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                    .draw(display)?;

                // 4 (adjacent to bevel)
                Triangle::new(ext_outside.start, inside_intersection, ext_outside.end)
                    .into_styled(tstyle)
                    .draw(display)?;

                // 5 (fill in end of second line)
                Triangle::new(inside_intersection, ext_outside.end, ext_inside.end)
                    .into_styled(tstyle)
                    .draw(display)?;
            }
        }
        // Line segments overlap (degenerate). Draw normal but overlapping thick lines with extra
        // triangle for bevel cap.
        else {
            // Fixed (first) line
            fixed.into_styled(linestyle).draw(display)?;

            // Moving (second) line
            l.into_styled(linestyle).draw(display)?;

            // Bevel cap
            match outer_side {
                Side::Left => Triangle::new(fixed_ext_l.end, mid, ext_l.start)
                    .into_styled(PrimitiveStyle::with_fill(Rgb888::RED))
                    .draw(display)?,
                Side::Right => Triangle::new(fixed_ext_r.end, mid, ext_r.start)
                    .into_styled(PrimitiveStyle::with_fill(Rgb888::MAGENTA))
                    .draw(display)?,
            }
        }

        // Debugging points
        {
            Circle::with_center(l_intersection, 5)
                .into_styled(if l_on_lines {
                    PrimitiveStyle::with_fill(Rgb888::YELLOW)
                } else {
                    PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1)
                })
                .draw(display)?;

            Circle::with_center(r_intersection, 5)
                .into_styled(if r_on_lines {
                    PrimitiveStyle::with_fill(Rgb888::new(0, 127, 255))
                } else {
                    PrimitiveStyle::with_stroke(Rgb888::new(0, 127, 255), 1)
                })
                .draw(display)?;
        }
    }
    // Lines are colinear (probably). Draw a single thick line from start of first line to end of
    // second line.
    else {
        Text::new("Colinear!", Point::zero())
            .into_styled(TextStyle::new(Font6x8, Rgb888::RED))
            .draw(display)?;

        Line::new(fixed.start, l.end)
            .into_styled(linestyle)
            .draw(display)?;
    }

    // empty_crosshair(ext_l.start, Rgb888::RED, display);
    // empty_crosshair(ext_l.end, Rgb888::RED, display);
    // empty_crosshair(ext_r.start, Rgb888::new(255, 127, 0), display);
    // empty_crosshair(ext_r.end, Rgb888::new(255, 127, 0), display);

    Ok(())
}

fn trongle(
    moving_point: Point,
    width: u32,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK).unwrap();

    let p1 = Point::new(100, 100);
    let p2 = Point::new(50, 130);
    let p3 = moving_point;

    let trongle = Triangle::new(p1, p2, p3);

    let l1 = Line::new(trongle.p1, trongle.p2);
    let l2 = Line::new(trongle.p1, trongle.p3);
    let l3 = Line::new(trongle.p2, trongle.p3);

    let l1_mid = l1.midpoint();
    let l2_mid = l2.midpoint();
    let l3_mid = l3.midpoint();

    draw(l1_mid, p1, l2_mid, width, display)?;
    draw(l1_mid, p2, l3_mid, width, display)?;
    draw(l2_mid, p3, l3_mid, width, display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(200, 200));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        .pixel_spacing(1)
        .build();
    let mut window = Window::new("Line joints debugger", &output_settings);

    let mut end_point = Point::new(20, 20);
    let mut width = 15u32;

    let mut mouse_down = false;

    let mid = Point::new(100, 100);
    let start = Point::new(50, 130);

    trongle(end_point, width, &mut display);

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

            trongle(end_point, width, &mut display);
        }
    }

    Ok(())
}
