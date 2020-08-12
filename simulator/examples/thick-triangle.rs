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

    // TriangleIterator::new(triangle, width, alignment, true).try_for_each(|(t, ty)| {
    //     let color = match ty {
    //         TriangleType::Border => Rgb888::RED,
    //         TriangleType::Fill => Rgb888::CYAN,
    //     };

    //     t.mathematical_points()
    //         .map(move |p| Pixel(p, color))
    //         // t.into_styled(PrimitiveStyle::with_stroke(
    //         //     color,
    //         //     1,
    //         // ))
    //         .draw(display)
    // })?;

    triangle
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(width)
                .stroke_alignment(alignment)
                .stroke_color(Rgb888::RED)
                .fill_color(Rgb888::CYAN)
                .build(),
        )
        .draw(display)?;

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
