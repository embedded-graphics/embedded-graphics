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

#[derive(Copy, Clone, Debug)]
enum JointKind {
    Miter,
    Bevel { filler_triangle: Triangle },
    Degenerate { filler_triangle: Triangle },
    Colinear,
    StartOrEnd,
}

#[derive(Copy, Clone, Debug)]
struct EdgeCorners {
    left: Point,
    right: Point,
}

#[derive(Copy, Clone, Debug)]
struct Joint {
    kind: JointKind,
    first_edge_end: EdgeCorners,
    second_edge_start: EdgeCorners,
}

fn corner(start: Point, mid: Point, end: Point, width: u32) -> Joint {
    let first_line = Line::new(start, mid);
    let second_line = Line::new(mid, end);

    // Miter length limit is dobule the line width (but squared to avoid sqrt() costs)
    let miter_limit = (width * 2).pow(2);

    // Left and right edges of thick first segment
    let (first_edge_left, first_edge_right) = first_line.extents(width as i32);
    // Left and right edges of thick second segment
    let (second_edge_left, second_edge_right) = second_line.extents(width as i32);

    if let (Some((l_intersection, l_on_lines)), Some((r_intersection, r_on_lines))) = (
        second_edge_left.intersection(&first_edge_left),
        second_edge_right.intersection(&first_edge_right),
    ) {
        let first_segment_start_edge = Line::new(first_edge_left.start, first_edge_right.start);
        let second_segment_end_edge = Line::new(second_edge_left.end, second_edge_right.end);

        let self_intersection_l = first_segment_start_edge
            .intersection(&second_edge_left)
            .filter(|(_, on_both)| *on_both)
            .or_else(|| {
                second_segment_end_edge
                    .intersection(&first_edge_left)
                    .filter(|(_, on_both)| *on_both)
            });

        let self_intersection_r = first_segment_start_edge
            .intersection(&second_edge_right)
            .filter(|(_, on_both)| *on_both)
            .or_else(|| {
                second_segment_end_edge
                    .intersection(&first_edge_right)
                    .filter(|(_, on_both)| *on_both)
            });

        let (self_intersection_l, self_intersection_r) =
            (self_intersection_l.is_some(), self_intersection_r.is_some());

        let is_self_intersecting = self_intersection_l || self_intersection_r;

        // The side that will have the miter/bevel on it, relative to first line
        let outer_side = if l_on_lines || self_intersection_l {
            Side::Right
        } else if r_on_lines || self_intersection_r {
            Side::Left
        } else {
            // Default to some randomly chosen side when lines are colinear
            Side::Right
        };

        let (outside_intersection, inside_intersection, ext_outside, fixed_outside) =
            match outer_side {
                Side::Right => (
                    r_intersection,
                    l_intersection,
                    second_edge_right,
                    first_edge_right,
                ),
                Side::Left => (
                    l_intersection,
                    r_intersection,
                    second_edge_left,
                    first_edge_left,
                ),
            };

        // Distance from midpoint to miter end point
        let miter_length_squared = Line::new(mid, outside_intersection).length_squared();

        // Normal line: non-overlapping line end caps
        if !is_self_intersecting {
            // Intersection is within limit at which it will be chopped off into a bevel, so return
            // a miter.
            if miter_length_squared <= miter_limit {
                let corners = EdgeCorners {
                    left: l_intersection,
                    right: r_intersection,
                };

                Joint {
                    kind: JointKind::Miter,
                    first_edge_end: corners,
                    second_edge_start: corners,
                }
            }
            // Miter is too long, chop it into bevel-style corner
            else {
                let filler_triangle =
                    Triangle::new(fixed_outside.end, ext_outside.start, inside_intersection);

                match outer_side {
                    // Line turning right
                    Side::Left => Joint {
                        kind: JointKind::Bevel { filler_triangle },
                        first_edge_end: EdgeCorners {
                            left: first_edge_left.end,
                            right: inside_intersection,
                        },
                        second_edge_start: EdgeCorners {
                            left: second_edge_left.start,
                            right: inside_intersection,
                        },
                    },
                    // Line turning left
                    Side::Right => Joint {
                        kind: JointKind::Bevel { filler_triangle },
                        first_edge_end: EdgeCorners {
                            left: inside_intersection,
                            right: first_edge_right.end,
                        },
                        second_edge_start: EdgeCorners {
                            left: inside_intersection,
                            right: second_edge_right.start,
                        },
                    },
                }
            }
        }
        // Line segments overlap (degenerate)
        else {
            Joint {
                kind: JointKind::Degenerate {
                    filler_triangle: Triangle::new(fixed_outside.end, ext_outside.start, mid),
                },
                first_edge_end: EdgeCorners {
                    left: first_edge_left.end,
                    right: first_edge_right.end,
                },
                second_edge_start: EdgeCorners {
                    left: second_edge_left.start,
                    right: second_edge_right.start,
                },
            }
        }
    }
    // Lines are colinear (probably). Draw a single thick line from start of first line to end of
    // second line.
    else {
        Joint {
            kind: JointKind::Colinear,
            first_edge_end: EdgeCorners {
                left: first_edge_left.start,
                right: first_edge_right.start,
            },
            second_edge_start: EdgeCorners {
                left: second_edge_left.end,
                right: second_edge_right.end,
            },
        }
    }
}

fn render_line(
    start_corner: Joint,
    end_corner: Joint,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let Joint {
        second_edge_start:
            EdgeCorners {
                left: left_start,
                right: right_start,
            },
        ..
    } = start_corner;
    let Joint {
        first_edge_end:
            EdgeCorners {
                left: left_end,
                right: right_end,
            },
        ..
    } = end_corner;

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        // .fill_color(Rgb888::GREEN)
        .build();

    Triangle::new(left_start, left_end, right_start)
        .into_styled(style)
        .draw(display)?;

    Triangle::new(right_start, left_end, right_end)
        .into_styled(style)
        .draw(display)?;

    Ok(())
}

fn draw_filler_triangle(
    corner: Joint,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::YELLOW)
        .stroke_width(1)
        // .fill_color(Rgb888::CYAN)
        .build();

    match corner.kind {
        JointKind::Bevel {
            filler_triangle, ..
        }
        | JointKind::Degenerate {
            filler_triangle, ..
        } => filler_triangle.into_styled(style).draw(display),
        _ => Ok(()),
    }
}

fn draw(
    triangle: Triangle,
    width: u32,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let corner_1 = corner(triangle.p3, triangle.p1, triangle.p2, width);
    let corner_2 = corner(triangle.p1, triangle.p2, triangle.p3, width);
    let corner_3 = corner(triangle.p2, triangle.p3, triangle.p1, width);

    // P1 -> P2
    render_line(corner_1, corner_2, display).unwrap();
    // P2 -> P3
    render_line(corner_2, corner_3, display).unwrap();
    // P3 -> P1
    render_line(corner_3, corner_1, display).unwrap();

    draw_filler_triangle(corner_1, display)?;
    draw_filler_triangle(corner_2, display)?;
    draw_filler_triangle(corner_3, display)?;

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

    draw(trongle, width, display)?;

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

    trongle(end_point, width, &mut display)?;

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

            trongle(end_point, width, &mut display)?;
        }
    }

    Ok(())
}
