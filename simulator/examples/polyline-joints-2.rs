use embedded_graphics::{
    fonts::*,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::line_joint::{EdgeCorners, LineJoint},
    primitives::polyline::ScanlineIntersections,
    primitives::polyline_outline_iter::PolylineOutlineIterator,
    // primitives::thick_segment::Segments,
    primitives::thick_segment::ThickSegment,
    primitives::triangle::MathematicalPoints,
    primitives::Line,
    primitives::*,
    style::*,
};
use embedded_graphics_simulator::{
    OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use sdl2::keyboard::Keycode;

fn crosshair(
    point: Point,
    color: Rgb888,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    let radius = Size::new(4, 4);

    Line::new(point - radius.x_axis(), point + radius.x_axis())
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)?;

    Line::new(point - radius.y_axis(), point + radius.y_axis())
        .into_styled(PrimitiveStyle::with_stroke(color, 1))
        .draw(display)
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

fn draw(
    points: &[Point],
    width: u32,
    alignment: StrokeAlignment,
    mouse_pos: Point,
    display: &mut SimulatorDisplay<Rgb888>,
) -> Result<(), core::convert::Infallible> {
    display.clear(Rgb888::BLACK)?;

    let mut points = points.to_vec();
    if let StrokeAlignment::Outside = alignment {
        points.reverse();
    }
    let points = &points;

    let scanline = Line::new(
        mouse_pos.y_axis(),
        mouse_pos.y_axis() + display.size().x_axis(),
    );
    let scanline_y = scanline.start.y;

    let pl = Polyline::new(points).into_styled(
        PrimitiveStyleBuilder::new()
            .stroke_width(width)
            // .stroke_alignment(alignment)
            .stroke_color(Rgb888::RED)
            .build(),
    );

    pl.bounding_box()
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 1))
        .draw(display)?;

    pl.draw(display)?;

    // let joints = tmp
    //     .windows(3)
    //     .enumerate()
    //     .map(|(idx, items)| {
    //         if let [start, mid, end] = items {
    //             if idx == 0 {
    //                 LineJoint::start(*start, *mid, width, StrokeAlignment::Center)
    //             } else {
    //                 LineJoint::from_points(*start, *mid, *end, width, StrokeAlignment::Center)
    //             }
    //         } else {
    //             unreachable!()
    //         }
    //     })
    //     .collect::<Vec<LineJoint>>();

    // let mut joints = joints.windows(2).skip(1).take(1);

    // while let Some([start_joint, end_joint]) = joints.next() {
    //     // let start_joint = LineJoint::start(*start, *mid, width, StrokeAlignment::Center);
    //     // let end_joint = LineJoint::from_points(*start, *mid, *end, width, StrokeAlignment::Center);
    //     dbg!(start_joint, end_joint);
    //     Segments::new(*start_joint, *end_joint)
    //         .enumerate()
    //         .try_for_each(|(idx, segment)| {
    //             Text::new(&format!("{}", idx), segment.start - Point::new(0, 8))
    //                 .into_styled(
    //                     TextStyleBuilder::new(Font6x8)
    //                         .text_color(Rgb888::MAGENTA)
    //                         .build(),
    //                 )
    //                 .draw(display)?;

    //             segment
    //                 .into_styled(
    //                     PrimitiveStyleBuilder::new()
    //                         .stroke_width(1)
    //                         // .stroke_alignment(alignment)
    //                         .stroke_color(Rgb888::RED)
    //                         .build(),
    //                 )
    //                 .draw(display)
    //         })?;
    // }

    // Scanline
    scanline
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::BLUE, 1))
        .draw(display)?;

    let lines = PolylineOutlineIterator::new(points, width, StrokeAlignment::Center);

    // // Draw polyline skeleton
    // lines.enumerate().try_for_each(|(idx, line)| {
    //     // Text::new(&format!("{}", idx), line.start)
    //     //     // Text::new(&format!("{}", line.sign_y()), line.start)
    //     //     .into_styled(
    //     //         TextStyleBuilder::new(Font6x8)
    //     //             .text_color(Rgb888::WHITE)
    //     //             .build(),
    //     //     )
    //     //     .draw(display)?;

    //     line.into_styled(
    //         PrimitiveStyleBuilder::new()
    //             .stroke_width(1)
    //             // .stroke_alignment(alignment)
    //             .stroke_color(Rgb888::MAGENTA)
    //             .build(),
    //     )
    //     .draw(display)
    // })?;

    let intersections =
        ScanlineIntersections::new(points, width, StrokeAlignment::Center, scanline.start.y);

    intersections.enumerate().try_for_each(|(idx, line)| {
        // Pixel(
        //     point,
        //     match state {
        //         State::Outside => Rgb888::YELLOW,
        //         State::Inside => Rgb888::BLACK,
        //     },
        // )
        // .draw(display)

        let marker =
            Line::new(line.start, line.end - Point::new(0, 10)).translate(if idx % 2 == 0 {
                Point::new(0, -15)
            } else {
                Point::zero()
            });

        empty_crosshair(marker.start, Rgb888::GREEN, display)?;
        empty_crosshair(marker.end, Rgb888::RED, display)?;

        marker
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(Rgb888::MAGENTA)
                    .build(),
            )
            .draw(display)?;

        // Text::new(&format!("{}", idx), marker.start - Point::new(0, 8))
        //     .into_styled(
        //         TextStyleBuilder::new(Font6x8)
        //             .text_color(Rgb888::MAGENTA)
        //             .build(),
        //     )
        //     .draw(display)?;

        line.into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(1)
                // .stroke_alignment(alignment)
                // .stroke_color(match state {
                //     State::Outside => Rgb888::YELLOW,
                //     State::Inside => Rgb888::RED,
                // })
                .stroke_color(Rgb888::YELLOW)
                .build(),
        )
        .draw(display)
    })?;

    Text::new(
        &format!("M {}, {}", mouse_pos.x, mouse_pos.y),
        Point::zero(),
    )
    .into_styled(
        TextStyleBuilder::new(Font6x8)
            .text_color(Rgb888::WHITE)
            .build(),
    )
    .draw(display)?;

    crosshair(mouse_pos, Rgb888::WHITE, display)?;

    Ok(())
}

const PADDING: i32 = 16;

fn main() -> Result<(), core::convert::Infallible> {
    // let (w, h) = (320i32, 256i32);

    let w = 320i32;
    // 16:9 aspect ratio for Twitter
    let h = 180i32;

    let mut display: SimulatorDisplay<Rgb888> =
        SimulatorDisplay::new(Size::new(w as u32 + 100, h as u32));
    let output_settings = OutputSettingsBuilder::new()
        .scale(4)
        // .pixel_spacing(1)
        .build();
    let mut window = Window::new("Polyline joints debugger", &output_settings);

    let mut end_point = Point::new(363, 39);

    let mut width = 15u32;
    let mut alignment = StrokeAlignment::Center;

    let mut mouse_down = false;

    let mut points = [
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
        end_point,
        // Point::new(w + 100, h / 2),
    ];

    let mut num_points = points.len();

    draw(
        &points[0..num_points],
        width,
        alignment,
        end_point,
        &mut display,
    )?;

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
                            StrokeAlignment::Outside => StrokeAlignment::Center,
                            // StrokeAlignment::Outside => StrokeAlignment::Inside,
                            StrokeAlignment::Inside => StrokeAlignment::Center,
                        }
                    }
                    _ => (),
                },
                SimulatorEvent::MouseButtonUp { .. } => mouse_down = false,
                SimulatorEvent::MouseMove { point, .. } => {
                    if mouse_down {
                        *points.get_mut(10).unwrap() = end_point;
                    }
                    end_point = point;
                }
                _ => {}
            }

            // *points.get_mut(10).unwrap() = end_point;

            draw(
                &points[0..num_points],
                width,
                alignment,
                end_point,
                &mut display,
            )?;
        }
    }

    Ok(())
}
