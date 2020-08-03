use core::slice::Windows;
use embedded_graphics::{
    fonts::*,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::line_joint::{EdgeCorners, JointKind, LineJoint},
    primitives::*,
    style::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn filled_tri(triangle: Triangle, color: Rgb888) -> impl Iterator<Item = Pixel<Rgb888>> {
    triangle.mathematical_points().map(move |p| Pixel(p, color))
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

fn draw_thick_edge(
    start_corner: LineJoint,
    end_corner: LineJoint,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let LineJoint {
        second_edge_start:
            EdgeCorners {
                left: left_start,
                right: right_start,
            },
        ..
    } = start_corner;
    let LineJoint {
        first_edge_end:
            EdgeCorners {
                left: left_end,
                right: right_end,
            },
        ..
    } = end_corner;

    let t1 = Triangle::new(left_start, left_end, right_start);
    let t2 = Triangle::new(right_start, left_end, right_end);

    filled_tri(t1, Rgb888::RED).draw(display)?;
    filled_tri(t2, Rgb888::RED).draw(display)?;

    // Highlight left (outside) edge
    Line::new(left_start, left_end)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
        .draw(display)?;

    Ok(())
}

fn draw_filler_triangle(
    corner: LineJoint,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    match corner.kind {
        JointKind::Bevel {
            filler_triangle, ..
        }
        | JointKind::Degenerate {
            filler_triangle, ..
        } => filled_tri(filler_triangle, Rgb888::YELLOW).draw(display),
        _ => Ok(()),
    }
}

fn draw_degenerate_edge(
    start_corner: LineJoint,
    end_corner: LineJoint,
    center: Point,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let LineJoint {
        second_edge_start:
            EdgeCorners {
                left: left_start,
                // right: right_start,
                ..
            },
        ..
    } = start_corner;
    let LineJoint {
        first_edge_end:
            EdgeCorners {
                left: left_end,
                // right: right_end,
                ..
            },
        ..
    } = end_corner;

    let tri = Triangle::new(left_start, left_end, center);

    if tri.area_doubled() > 0 {
        filled_tri(tri, Rgb888::CYAN).draw(display)?;
    }

    // Highlight left (outside) edge
    Line::new(left_start, left_end)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
        .draw(display)?;

    Ok(())
}

fn draw_degenerate_bevel(
    corner: LineJoint,
    center: Point,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    match corner.kind {
        JointKind::Bevel {
            filler_triangle, ..
        }
        | JointKind::Degenerate {
            filler_triangle, ..
        } => {
            let t = Triangle::new(filler_triangle.p1, filler_triangle.p2, center);

            filled_tri(t, Rgb888::GREEN).draw(display)
        }
        _ => Ok(()),
    }
}

/// Calculate squared distance from midpoint of an outside (left) edge to the center of the triangle
fn calc_dist(center: Point, start: LineJoint, end: LineJoint) -> u32 {
    let start = start.second_edge_start.left;
    let end = end.first_edge_end.left;

    Line::new(start, end).distance_to_point_squared(center)
}

struct TriangleIterator {
    is_collapsed: bool,

    /// Whether the triangle center is filled or not.
    has_fill: bool,
    joints: [LineJoint; 3],
    at_joint: usize,
    t1: Option<Triangle>,
    t2: Option<Triangle>,
    filler: Option<Triangle>,
    inner_fill: Option<Triangle>,
    center: Point,
}

impl TriangleIterator {
    fn new(
        triangle: Triangle,
        stroke_width: u32,
        stroke_alignment: StrokeAlignment,
        has_fill: bool,
    ) -> Self {
        let triangle = triangle.sorted_clockwise();
        let Triangle { p1, p2, p3 } = triangle;

        // Joint centered around P1
        let joint1 = LineJoint::from_points(p3, p1, p2, stroke_width, stroke_alignment);
        // Joint centered around P2
        let joint2 = LineJoint::from_points(p1, p2, p3, stroke_width, stroke_alignment);
        // Joint centered around P3
        let joint3 = LineJoint::from_points(p2, p3, p1, stroke_width, stroke_alignment);

        let centroid = triangle.centroid();

        let dist1 = calc_dist(centroid, joint1, joint2);
        let dist2 = calc_dist(centroid, joint2, joint3);
        let dist3 = calc_dist(centroid, joint3, joint1);

        // Flag denoting whether the inside of the triangle is completely filled by the edge strokes
        // or not.
        let is_collapsed = dist1 < stroke_width.pow(2)
            || dist2 < stroke_width.pow(2)
            || dist3 < stroke_width.pow(2);

        let joints = [joint1, joint2, joint3];

        let inner_fill = if has_fill && !is_collapsed {
            Some(Triangle::new(
                joint1.first_edge_end.right,
                joint2.first_edge_end.right,
                joint3.first_edge_end.right,
            ))
        } else {
            None
        };

        let mut self_ = Self {
            is_collapsed,
            at_joint: 0,
            joints,
            t1: None,
            t2: None,
            filler: None,
            center: centroid,
            has_fill,
            inner_fill,
        };

        self_.update_triangles();

        self_
    }

    fn update_triangles(&mut self) -> Option<()> {
        let s = self.at_joint % 3;
        let e = (self.at_joint + 1) % 3;

        let start_joint = self.joints.get(s)?;
        let end_joint = self.joints.get(e)?;

        if !self.is_collapsed {
            let (t1, t2) = Self::edge_triangles(*start_joint, *end_joint);

            self.t1 = Some(t1);
            self.t2 = Some(t2);
            self.filler = start_joint.filler();
        } else {
            let filler = start_joint.filler().map(|filler| {
                // Rework filler triangle so it touches the triangle center
                Triangle::new(filler.p1, filler.p2, self.center)
            });

            self.t1 = Self::collapsed_edge_triangle(*start_joint, *end_joint, self.center);
            self.t2 = None;
            self.filler = filler;
        }

        Some(())
    }

    // TODO: Dedupe with polyline method of same name
    fn edge_triangles(start_joint: LineJoint, end_joint: LineJoint) -> (Triangle, Triangle) {
        let LineJoint {
            second_edge_start:
                EdgeCorners {
                    left: left_start,
                    right: right_start,
                },
            ..
        } = start_joint;
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    right: right_end,
                },
            ..
        } = end_joint;

        let t1 = Triangle::new(left_start, left_end, right_start);
        let t2 = Triangle::new(right_start, left_end, right_end);

        (t1, t2)
    }

    fn collapsed_edge_triangle(
        start_joint: LineJoint,
        end_joint: LineJoint,
        center: Point,
    ) -> Option<Triangle> {
        let LineJoint {
            second_edge_start:
                EdgeCorners {
                    left: left_start,
                    // right: right_start,
                    ..
                },
            ..
        } = start_joint;
        let LineJoint {
            first_edge_end:
                EdgeCorners {
                    left: left_end,
                    // right: right_end,
                    ..
                },
            ..
        } = end_joint;

        let tri = Triangle::new(left_start, left_end, center);

        if tri.area_doubled() > 0 {
            Some(tri)
        } else {
            None
        }
    }
}

enum TriangleType {
    Border,
    Fill,
}

impl Iterator for TriangleIterator {
    type Item = (Triangle, TriangleType);

    fn next(&mut self) -> Option<Self::Item> {
        self.inner_fill
            .take()
            .map(|t| (t, TriangleType::Fill))
            .or_else(|| self.t1.take().map(|t| (t, TriangleType::Border)))
            .or_else(|| self.t2.take().map(|t| (t, TriangleType::Border)))
            .or_else(|| self.filler.take().map(|t| (t, TriangleType::Border)))
            .or_else(|| {
                self.at_joint += 1;

                // Triangles for all 3 joints and edges have been returned. We're done.
                if self.at_joint == 3 {
                    return None;
                }

                self.update_triangles();

                self.next()
            })

        // if let Some(triangle) = self
        //     .t1
        //     .take()
        //     .or_else(|| self.t2.take())
        //     .or_else(|| self.filler.take())
        // {
        //     Some(triangle)
        // }
        // // We've gone through the list of triangles in this edge/joint. Reset state for next edge
        // // and joint.
        // else {
        //     self.at_joint += 1;

        //     // Triangles for all 3 joints and edges have been returned. We're done.
        //     if self.at_joint == 3 {
        //         return None;
        //     }

        //     self.update_triangles();

        //     self.next()
        // }
    }
}

fn draw(
    triangle: Triangle,
    width: u32,
    alignment: StrokeAlignment,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let corner_1 = LineJoint::from_points(triangle.p3, triangle.p1, triangle.p2, width, alignment);
    let corner_2 = LineJoint::from_points(triangle.p1, triangle.p2, triangle.p3, width, alignment);
    let corner_3 = LineJoint::from_points(triangle.p2, triangle.p3, triangle.p1, width, alignment);

    Text::new(
        &format!("{} {} {}", corner_1.kind, corner_2.kind, corner_3.kind),
        Point::zero(),
    )
    .into_styled(
        TextStyleBuilder::new(Font6x8)
            .background_color(Rgb888::YELLOW)
            .text_color(Rgb888::BLUE)
            .build(),
    )
    .draw(display)?;

    TriangleIterator::new(triangle, width, alignment, true).try_for_each(|(t, ty)| {
        let color = match ty {
            TriangleType::Border => Rgb888::RED,
            TriangleType::Fill => Rgb888::CYAN,
        };

        t.mathematical_points()
            .map(move |p| Pixel(p, color))
            // t.into_styled(PrimitiveStyle::with_stroke(
            //     color,
            //     1,
            // ))
            .draw(display)
    })?;

    // let centroid = triangle.centroid();

    // let dist1 = calc_dist(centroid, corner_1, corner_2);
    // let dist2 = calc_dist(centroid, corner_2, corner_3);
    // let dist3 = calc_dist(centroid, corner_3, corner_1);

    // let is_filled = dist1 < width.pow(2) || dist2 < width.pow(2) || dist3 < width.pow(2);

    // // Triangle is completely filled. Draw triangle fan around center
    // if is_filled {
    //     // P1 -> P2
    //     draw_degenerate_edge(corner_1, corner_2, centroid, display).unwrap();
    //     // P2 -> P3
    //     draw_degenerate_edge(corner_2, corner_3, centroid, display).unwrap();
    //     // P3 -> P1
    //     draw_degenerate_edge(corner_3, corner_1, centroid, display).unwrap();

    //     draw_degenerate_bevel(corner_1, centroid, display)?;
    //     draw_degenerate_bevel(corner_2, centroid, display)?;
    //     draw_degenerate_bevel(corner_3, centroid, display)?;
    // }
    // // Triangle has a hole in the center. Draw borders and joints as normal.
    // else {
    //     // P1 -> P2
    //     draw_thick_edge(corner_1, corner_2, display)?;
    //     // P2 -> P3
    //     draw_thick_edge(corner_2, corner_3, display)?;
    //     // P3 -> P1
    //     draw_thick_edge(corner_3, corner_1, display)?;

    //     draw_filler_triangle(corner_1, display)?;
    //     draw_filler_triangle(corner_2, display)?;
    //     draw_filler_triangle(corner_3, display)?;

    //     // Fill inside
    //     filled_tri(
    //         Triangle::new(
    //             corner_1.first_edge_end.right,
    //             corner_2.first_edge_end.right,
    //             corner_3.first_edge_end.right,
    //         ),
    //         Rgb888::YELLOW,
    //     )
    //     .draw(display)?;
    // }

    // empty_crosshair(centroid, Rgb888::MAGENTA, display);

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

    Text::new(&format!("W {}", width), Point::new(30, 8))
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)?;

    Text::new(&format!("{:?}", alignment), Point::new(0, 8))
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
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Line joints debugger", &output_settings);

    // let mut end_point = Point::new(20, 20);
    let mut end_point = Point::new(82, 110);
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
