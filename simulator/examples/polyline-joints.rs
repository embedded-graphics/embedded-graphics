use embedded_graphics::{
    fonts::*,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::line::{Intersection, Side},
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

    // filled_tri(t1, Rgb888::RED).draw(display)?;
    // filled_tri(t2, Rgb888::RED).draw(display)?;

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(1)
        // .fill_color(Rgb888::GREEN)
        .build();

    t1.into_styled(style).draw(display)?;

    t2.into_styled(style).draw(display)?;

    Ok(())
}

fn draw(
    points: &[Point],
    width: u32,
    alignment: StrokeAlignment,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK).unwrap();

    let mut interior_joints = points.windows(3).map(|slice| match slice {
        [start, mid, end] => LineJoint::from_points(*start, *mid, *end, width, alignment),
        _ => todo!(),
    });

    let mut prev_joint = LineJoint::start(points[0], points[1], width, alignment);
    let mut curr_joint = interior_joints.next().unwrap();

    loop {
        draw_thick_edge(prev_joint, curr_joint, display)?;

        prev_joint = curr_joint;

        if let Some(curr) = interior_joints.next() {
            curr_joint = curr;
        } else {
            break;
        }
    }

    let penultimate = points[points.len() - 2];
    let last = points.last().unwrap();

    let final_joint = LineJoint::end(penultimate, *last, width, alignment);

    draw_thick_edge(curr_joint, final_joint, display)?;

    let skeleton_style = PrimitiveStyle::with_stroke(Rgb888::YELLOW, 1);

    Polyline::new(points)
        .into_styled(skeleton_style)
        .draw(display)?;

    Ok(())
}

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    let (w, h) = (320i32, 256i32);

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

    // let points = [
    //     Point::new(PADDING, h / 2),
    //     Point::new(100, h / 2),
    //     Point::new(120, h / 2 - 20),
    //     Point::new(140, h / 2),
    //     Point::new(160, h / 2),
    //     Point::new(180, h / 2 + 10),
    //     Point::new(200, PADDING),
    //     Point::new(220, h / 2 + 20),
    //     Point::new(240, h / 2),
    //     Point::new(w - PADDING, h / 2),
    // ];

    let p1 = Point::new(20, h / 2);
    let p2 = Point::new(w / 2, h / 3);

    draw(&[p1, p2, end_point], width, alignment, &mut display)?;

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

            draw(&[p1, p2, end_point], width, alignment, &mut display)?;
        }
    }

    Ok(())
}
