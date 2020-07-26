use embedded_graphics::{
    fonts::*,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::line::{Intersection, Side},
    primitives::*,
    style::*,
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

fn corner(start: Point, mid: Point, end: Point, width: u32, alignment: StrokeAlignment) -> Joint {
    let first_line = Line::new(start, mid);
    let second_line = Line::new(mid, end);

    // Miter length limit is dobule the line width (but squared to avoid sqrt() costs)
    let miter_limit = (width * 2).pow(2);

    // Left and right edges of thick first segment
    let (first_edge_left, first_edge_right) = first_line.extents(width as i32, alignment);
    // Left and right edges of thick second segment
    let (second_edge_left, second_edge_right) = second_line.extents(width as i32, alignment);

    if let (
        Intersection::Point {
            point: l_intersection,
            side: outer_side,
            ..
        },
        Intersection::Point {
            point: r_intersection,
            ..
        },
    ) = (
        second_edge_left.line_intersection(&first_edge_left),
        second_edge_right.line_intersection(&first_edge_right),
    ) {
        let first_segment_start_edge = Line::new(first_edge_left.start, first_edge_right.start);
        let second_segment_end_edge = Line::new(second_edge_left.end, second_edge_right.end);

        let self_intersection_l = first_segment_start_edge.segment_intersection(&second_edge_left)
            || second_segment_end_edge.segment_intersection(&first_edge_left);

        let self_intersection_r = first_segment_start_edge.segment_intersection(&second_edge_right)
            || second_segment_end_edge.segment_intersection(&first_edge_right);

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
        if !self_intersection_l && !self_intersection_r {
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
                let kind = JointKind::Bevel {
                    filler_triangle: Triangle::new(
                        fixed_outside.end,
                        ext_outside.start,
                        inside_intersection,
                    ),
                };

                match outer_side {
                    // Line turning right
                    Side::Left => Joint {
                        kind,
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
                        kind,
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
    // Lines are colinear
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

    // Highlight left (outside) edge
    Line::new(left_start, left_end)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
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
        // .fill_color(Rgb888::GREEN)
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
    alignment: StrokeAlignment,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let corner_1 = corner(triangle.p3, triangle.p1, triangle.p2, width, alignment);
    let corner_2 = corner(triangle.p1, triangle.p2, triangle.p3, width, alignment);
    let corner_3 = corner(triangle.p2, triangle.p3, triangle.p1, width, alignment);

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
    alignment: StrokeAlignment,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK).unwrap();

    let p1 = Point::new(100, 100);
    let p2 = Point::new(50, 130);
    let p3 = moving_point;
    // let p3 = Point::new(92, 20);

    let trongle = Triangle::new(p1, p2, p3).sorted_clockwise();

    draw(trongle, width, alignment, display)?;

    Text::new("P1", trongle.p1)
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)?;

    Text::new("P2", trongle.p2)
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)?;

    Text::new("P3", trongle.p3)
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)?;

    Text::new(&format!("{:?}", alignment), Point::zero())
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(190, 190));
    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Line joints debugger", &output_settings);

    let mut end_point = Point::new(20, 20);
    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    let mut mouse_down = false;

    trongle(end_point, width, alignment, &mut display)?;

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
                    Keycode::Space => {
                        alignment = match alignment {
                            StrokeAlignment::Center => StrokeAlignment::Outside,
                            StrokeAlignment::Outside => StrokeAlignment::Inside,
                            StrokeAlignment::Inside => StrokeAlignment::Center,
                        }
                    }
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

            trongle(end_point, width, alignment, &mut display)?;
        }
    }

    Ok(())
}
