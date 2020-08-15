use embedded_graphics::{
    fonts::*,
    pixelcolor::{Gray8, Rgb888},
    prelude::*,
    primitives::line_joint::{EdgeCorners, LineJoint},
    primitives::triangle::MathematicalPoints,
    primitives::*,
    style::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, OverdrawDisplay, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;
use std::cmp::Ordering;
use triangle::sort_clockwise;

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

fn empty_crosshair(
    point: Point,
    color: Rgb888,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let radius = Size::new_equal(4);
    let inner_radius = Size::new_equal(2);

    Line::new(point - radius.x_axis(), point - inner_radius.x_axis())
        .points()
        .chain(Line::new(point + radius.x_axis(), point + inner_radius.x_axis()).points())
        .chain(Line::new(point - radius.y_axis(), point - inner_radius.y_axis()).points())
        .chain(Line::new(point + radius.y_axis(), point + inner_radius.y_axis()).points())
        .map(|p| Pixel(p, color))
        .draw(display)
}

fn point_label(
    point: Point,
    idx: u32,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    Text::new(&format!("P{}", idx), point)
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)
}

fn sort_two_yx_cmp(p1: &Point, p2: &Point) -> Ordering {
    // If p1.y is less than p2.y, return it first. Otherwise, if they have the same Y coordinate,
    // the first point becomes the one with the lesser X coordinate.
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

// Flag will be true if pair was swapped
fn sort_two_yx(p1: Point, p2: Point) -> (Point, Point, bool) {
    // If p1.y is less than p2.y, return it first. Otherwise, if they have the same Y coordinate,
    // the first point becomes the one with the lesser X coordinate.
    if p1.y < p2.y || (p1.y == p2.y && p1.x < p2.x) {
        (p1, p2, false)
    } else {
        (p2, p1, true)
    }
}

fn trapezium(
    mouse_pos: Point,
    points: [Point; 4],
    scanline: Line,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    // let mut points = points;
    // points.sort_by(|a, b| sort_two_yx_cmp(a, b));

    // let center = points
    //     .iter()
    //     .fold(Point::zero(), |accum, point| accum + *point)
    //     / 4;

    // empty_crosshair(center, Rgb888::CYAN, display)?;

    // let mut points = points;
    // points.sort_by(|a, b| sort_clockwise(a, b, center));

    let [p0, p1, p2, p3] = points;

    point_label(p0, 0 as u32, display)?;
    point_label(p1, 1 as u32, display)?;
    point_label(p2, 2 as u32, display)?;
    point_label(p3, 3 as u32, display)?;

    let lines = [
        Line::new(p0, p1),
        Line::new(p1, p2),
        Line::new(p2, p3),
        Line::new(p3, p0),
    ];

    let intersections = lines
        .iter()
        .filter_map(|l| l.segment_intersection_point(&scanline));

    let (min, max): (Option<Point>, Option<Point>) =
        intersections.fold((None, None), |acc, intersection_point| {
            (
                acc.0
                    .map(|min| min.component_min(intersection_point))
                    .or_else(|| Some(intersection_point)),
                acc.1
                    .map(|max| max.component_max(intersection_point))
                    .or_else(|| Some(intersection_point)),
            )
        });

    let style = PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1);

    if let (Some(min), Some(max)) = (min, max) {
        let fill_line = Line::new(min, max);

        fill_line.into_styled(style).draw(display)?;
    }

    Ok(())
}

fn trapezium_bounding_box(points: [Point; 4]) -> Rectangle {
    let min = points[0]
        .component_min(points[1])
        .component_min(points[2])
        .component_min(points[3]);
    let max = points[0]
        .component_max(points[1])
        .component_max(points[2])
        .component_max(points[3]);

    Rectangle::with_corners(min, max)
}

struct TrapeziumIterator {
    points: [Point; 4],
    pos: Point,
    right_limit: i32,
    y_limit: i32,
    scanline: Line,
}

impl TrapeziumIterator {
    fn new(points: [Point; 4]) -> Self {
        let bb = trapezium_bounding_box(points);

        let scanline = Line::new(bb.top_left, bb.top_left + bb.size.x_axis());

        if let Some((left, right)) = Self::intersections(&scanline, &points) {
            Self {
                points,
                pos: left,
                right_limit: right.x,
                scanline,
                y_limit: bb.bottom_right().unwrap().y,
            }
        } else {
            Self::empty()
        }
    }

    fn empty() -> Self {
        Self {
            points: [Point::zero(); 4],
            pos: Point::zero(),
            right_limit: 0,
            scanline: Line::new(Point::zero(), Point::zero()),
            y_limit: 0,
        }
    }

    fn intersections(scanline: &Line, points: &[Point; 4]) -> Option<(Point, Point)> {
        let [p0, p1, p2, p3] = *points;

        let lines = [
            Line::new(p0, p1),
            Line::new(p1, p2),
            Line::new(p2, p3),
            Line::new(p3, p0),
        ];

        let intersections = lines
            .iter()
            .filter_map(|l| l.segment_intersection_point(&scanline));

        let (min, max): (Option<Point>, Option<Point>) =
            intersections.fold((None, None), |acc, intersection_point| {
                (
                    acc.0
                        .map(|min| min.component_min(intersection_point))
                        .or_else(|| Some(intersection_point)),
                    acc.1
                        .map(|max| max.component_max(intersection_point))
                        .or_else(|| Some(intersection_point)),
                )
            });

        if let (Some(min), Some(max)) = (min, max) {
            Some((min, max))
        } else {
            None
        }
    }
}

impl Iterator for TrapeziumIterator {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        let point = self.pos;

        self.pos.x += 1;

        // Reached end of scanline. Step down one.
        if self.pos.x > self.right_limit {
            // Step down a line
            self.scanline.translate_mut(Point::new(0, 1));

            // If scanline is off bottom of bounding box, we're finished
            if self.scanline.start.y > self.y_limit {
                return None;
            }

            let (min, max) = Self::intersections(&self.scanline, &self.points)?;

            self.pos = min;
            self.right_limit = max.x;

            self.next()
        } else {
            Some(point)
        }
    }
}

fn draw(
    mouse_pos: Point,
    corner_pos: Point,
    // display: &mut OverdrawDisplay,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK)?;

    let scanline = Line::new(
        mouse_pos.y_axis(),
        mouse_pos.y_axis() + display.size().x_axis(),
    );

    // Scanline
    scanline
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(display)?;

    let points1 = [
        Point::new(40, 20),
        Point::new(80, 10),
        corner_pos,
        Point::new(30, 60),
        // Point::new(10, 40),
    ];

    TrapeziumIterator::new(points1)
        .map(|p| Pixel(p, Rgb888::YELLOW))
        .draw(display)?;

    // trapezium(mouse_pos, points1, scanline, display)?;

    // let points2 = [
    //     Point::new(40, 5) + Point::new(100, 0),
    //     Point::new(80, 10) + Point::new(100, 0),
    //     corner_pos + Point::new(100, 0),
    //     Point::new(30, 60) + Point::new(100, 0),
    //     // Point::new(10, 40),
    // ];

    // trapezium(mouse_pos, points2, scanline, display)?;

    Ok(())
}

fn main() -> Result<(), core::convert::Infallible> {
    let w = 150i32;
    let h = 100i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32 + 100, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Polyline segment debugger", &output_settings);

    // let mut overdraw_display = OverdrawDisplay::new(display.size());

    let mut corner_pos = Point::zero();
    let mut mouse_pos = Point::zero();

    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    let mut mouse_down = false;

    draw(mouse_pos, corner_pos, &mut display)?;

    // overdraw_display.draw_to_display(&mut display)?;

    'running: loop {
        window.update(&display);

        for event in window.events() {
            match event {
                SimulatorEvent::Quit => break 'running,
                SimulatorEvent::MouseButtonDown { point, .. } => {
                    mouse_down = true;

                    corner_pos = point;
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
                        corner_pos = point;
                    }
                    mouse_pos = point;
                }
                _ => {}
            }

            draw(mouse_pos, corner_pos, &mut display)?;

            // overdraw_display.draw_to_display(&mut display)?;
        }
    }

    Ok(())
}
