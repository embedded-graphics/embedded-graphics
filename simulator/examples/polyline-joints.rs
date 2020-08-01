use embedded_graphics::{
    fonts::*,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::line_joint::{EdgeCorners, LineJoint},
    primitives::triangle::MathematicalPoints,
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

struct TriangleIterator<'a> {
    points: &'a [Point],
    start_idx: usize,
    t1: Option<Triangle>,
    t2: Option<Triangle>,
    filler: Option<Triangle>,
    width: u32,
    alignment: StrokeAlignment,
    end_joint: LineJoint,
}

impl<'a> TriangleIterator<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let start_idx = 0;

            // If there are enough points to compute first joint, do so. Otherwise the line is two
            // points long and should just be a straight segment.
            let start_joint =
                LineJoint::start(points[start_idx], points[start_idx + 1], width, alignment);
            let end_joint = if points.len() >= 3 {
                LineJoint::from_points(
                    points[start_idx],
                    points[start_idx + 1],
                    points[start_idx + 2],
                    width,
                    alignment,
                )
            } else {
                LineJoint::end(points[start_idx], points[start_idx + 1], width, alignment)
            };

            // Initialise with line between p0 and p1
            let (t1, t2) = Self::edge_triangles(start_joint, end_joint);

            Self {
                points,
                t1: Some(t1),
                t2: Some(t2),
                start_idx,
                filler: end_joint.filler(),
                width,
                alignment,
                end_joint,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            points: &[],
            t1: None,
            t2: None,
            filler: None,
            start_idx: 0,
            width: 0,
            alignment: StrokeAlignment::Center,
            end_joint: LineJoint::empty(),
        }
    }

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
}

impl<'a> Iterator for TriangleIterator<'a> {
    type Item = Triangle;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(triangle) = self
            .t1
            .take()
            .or_else(|| self.t2.take())
            .or_else(|| self.filler.take())
        {
            Some(triangle)
        }
        // We've gone through the list of triangles in this edge/joint. Reset state for next edge
        // and joint.
        else {
            self.start_idx += 1;

            let start_joint = self.end_joint;

            self.end_joint = if let Some(third_point) = self.points.get(self.start_idx + 2) {
                LineJoint::from_points(
                    *self.points.get(self.start_idx)?,
                    *self.points.get(self.start_idx + 1)?,
                    *third_point,
                    self.width,
                    self.alignment,
                )
            } else {
                LineJoint::end(
                    *self.points.get(self.start_idx)?,
                    *self.points.get(self.start_idx + 1)?,
                    self.width,
                    self.alignment,
                )
            };

            let (t1, t2) = Self::edge_triangles(start_joint, self.end_joint);

            self.t1 = Some(t1);
            self.t2 = Some(t2);
            self.filler = self.end_joint.filler();

            self.next()
        }
    }
}

struct ThickPoints<'a> {
    triangle_iter: TriangleIterator<'a>,
    points_iter: MathematicalPoints,
}

impl<'a> ThickPoints<'a> {
    pub fn new(points: &'a [Point], width: u32, alignment: StrokeAlignment) -> Self {
        if points.len() < 2 {
            Self::empty()
        } else {
            let mut triangle_iter = TriangleIterator::new(points, width, alignment);

            let points_iter = triangle_iter
                .next()
                .map(|t| t.mathematical_points())
                .unwrap_or_else(MathematicalPoints::empty);

            Self {
                triangle_iter,
                points_iter,
            }
        }
    }

    pub fn empty() -> Self {
        Self {
            triangle_iter: TriangleIterator::empty(),
            points_iter: MathematicalPoints::empty(),
        }
    }
}

impl<'a> Iterator for ThickPoints<'a> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.points_iter.next() {
            Some(point)
        } else {
            self.points_iter = self.triangle_iter.next().map(|t| t.mathematical_points())?;

            self.next()
        }
    }
}

fn draw(
    points: &[Point],
    width: u32,
    alignment: StrokeAlignment,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK).unwrap();

    Text::new(&format!("Points {}", points.len()), Point::zero())
        .into_styled(
            TextStyleBuilder::new(Font6x8)
                .background_color(Rgb888::YELLOW)
                .text_color(Rgb888::BLUE)
                .build(),
        )
        .draw(display)?;

    ThickPoints::new(points, width, alignment)
        .map(|p| Pixel(p, Rgb888::RED))
        .draw(display)?;

    let skeleton_style = PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1);

    Polyline::new(points)
        .into_styled(skeleton_style)
        .draw(display)
}

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    // let (w, h) = (320i32, 256i32);

    let w = 320i32;
    // 16:9 aspect ratio for Twitter
    let h = 180i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Polyline joints debugger", &output_settings);

    let mut end_point = Point::new(82, 110);

    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    let mut mouse_down = false;

    let points = [
        Point::new(PADDING, h / 2),
        Point::new(100, h / 2),
        Point::new(120, h / 2 - 20),
        Point::new(140, h / 2),
        Point::new(160, h / 2),
        Point::new(180, h / 2 + 10),
        Point::new(200, PADDING),
        Point::new(220, h / 2 + 20),
        Point::new(240, h / 2),
        Point::new(w - PADDING, h / 2),
    ];

    let mut num_points = points.len();

    draw(&points[0..num_points], width, alignment, &mut display)?;

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
                    Keycode::Left => num_points = num_points.saturating_sub(1),
                    Keycode::Right => num_points = (num_points + 1).min(points.len()),
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

            draw(&points[0..num_points], width, alignment, &mut display)?;
        }
    }

    Ok(())
}
