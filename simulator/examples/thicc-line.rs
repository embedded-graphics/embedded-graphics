use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::{egline, fonts::Font6x8, primitives::Line, text_6x8};
use embedded_graphics_simulator::DisplayBuilder;
use embedded_graphics_simulator::RgbDisplay;
use embedded_graphics_simulator::SimulatorEvent;
use integer_sqrt::IntegerSquareRoot;
use sdl2::keyboard::Keycode;
use std::thread;
use std::time::Duration;

fn draw_perp(
    display: &mut RgbDisplay,
    start: Point,
    delta: Point,
    direction: Point,
    p_error_initial: i32,
    width: i32,
    error_initial: i32,
) {
    // let width_threshold_sq =
    //     2 * (width / 2) * ((((delta.x.pow(2) + delta.y.pow(2)) as f32).sqrt()) as i32);

    // let width_threshold_sq_2 = 4 * (width / 2).pow(2) * (delta.x.pow(2) + delta.y.pow(2));
    let width_threshold_sq = width * (delta.x.pow(2) + delta.y.pow(2)).integer_sqrt();

    // assert_eq!(width_threshold_sq.pow(2), width_threshold_sq_2);

    // let width_threshold_sq = width_threshold_sq_2;

    let larger_component = delta.x.max(delta.y);
    let other_component = delta.x.min(delta.y);

    let e_diag = larger_component * -2;
    let e_square = other_component * 2;

    let threshold = larger_component - 2 * other_component;

    {
        let mut p = start;

        let (mut error, mut width_accum) = if delta.x > delta.y {
            (p_error_initial, delta.x + delta.y - error_initial)
        } else {
            (-p_error_initial, delta.x + delta.y + error_initial)
        };

        // Add another pixel if line width is even
        let width_threshold_sq = if width % 2 == 0 {
            width_threshold_sq + 2 * larger_component
        } else {
            width_threshold_sq
        };

        // Right hand side of line
        while width_accum <= width_threshold_sq {
            display.set_pixel(p.x as usize, p.y as usize, Rgb888::CYAN);

            if error > threshold {
                if delta.x > delta.y {
                    p -= Point::new(direction.x, 0);
                } else {
                    p += Point::new(0, direction.y);
                };

                error += e_diag;

                width_accum += 2 * other_component;
            }

            error += e_square;

            if delta.x > delta.y {
                p += Point::new(0, direction.y);
            } else {
                p -= Point::new(direction.x, 0);
            };

            width_accum += 2 * larger_component;
        }
    }

    {
        let mut p = start;

        let (mut error, mut width_accum) = if delta.x > delta.y {
            (-p_error_initial, delta.x + delta.y + error_initial)
        } else {
            (p_error_initial, delta.x + delta.y - error_initial)
        };

        // Left hand side of line
        while width_accum <= width_threshold_sq {
            display.set_pixel(p.x as usize, p.y as usize, Rgb888::YELLOW);

            if error > threshold {
                if delta.x > delta.y {
                    p += Point::new(direction.x, 0);
                } else {
                    p -= Point::new(0, direction.y);
                };

                error += e_diag;

                width_accum += 2 * other_component;
            }

            error += e_square;

            if delta.x > delta.y {
                p -= Point::new(0, direction.y);
            } else {
                p += Point::new(direction.x, 0);
            };

            width_accum += 2 * larger_component;
        }
    }
}

fn draw_line(display: &mut RgbDisplay, p0: Point, p1: Point, width: i32) {
    match width {
        0 => return,
        // 1 => display.draw(egline!(p0, p1, stroke_color = Some(Rgb888::WHITE))),
        width => {
            let delta = (p1 - p0).abs();

            let direction = match (p1.x >= p0.x, p1.y >= p0.y) {
                (true, true) => Point::new(1, 1),
                (true, false) => Point::new(1, -1),
                (false, true) => Point::new(-1, 1),
                (false, false) => Point::new(-1, -1),
            };

            let mut error = 0;
            // Perpendicular error
            let mut p_error = 0;

            let mut p = p0;

            let (threshold, e_diag, e_square) = if delta.x > delta.y {
                (delta.x - 2 * delta.y, -2 * delta.x, 2 * delta.y)
            } else {
                (delta.y - 2 * delta.x, -2 * delta.y, 2 * delta.x)
            };

            for _ in 0..=delta.x.max(delta.y).abs() {
                draw_perp(display, p, delta, direction, p_error, width, error);

                if error > threshold {
                    if delta.x > delta.y {
                        p += Point::new(0, direction.y);
                    } else {
                        p += Point::new(direction.x, 0);
                    };

                    error += e_diag;

                    if p_error > threshold {
                        draw_perp(
                            display,
                            p,
                            delta,
                            direction,
                            p_error + e_diag + e_square,
                            width,
                            error,
                        );

                        p_error += e_diag;
                    }

                    p_error += e_square;
                }

                error += e_square;

                if delta.x > delta.y {
                    p += Point::new(direction.x, 0);
                } else {
                    p += Point::new(0, direction.y);
                };
            }
        }
    }
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(120, 120)
        .scale(12)
        .pixel_spacing(1)
        .build_rgb();

    let mut angle: f32 = 0.0;

    // Uncomment to draw out straight test lines
    // for i in 0..12 {
    //     draw_line(
    //         &mut display,
    //         Point::new(10, 5 + (i * 15)),
    //         Point::new(100, 5 + (i * 15)),
    //         i,
    //     );

    //     let t = format!("W: {}", i);

    //     let text: Font6x8<Rgb888> = text_6x8!(&t).translate(Point::new(110, 5 + i * 15));

    //     display.draw(&text);
    // }

    // draw_line(&mut display, Point::new(10, 20), Point::new(20, 10), 1);
    // display.draw(
    //     Line::new(Point::new(10, 20), Point::new(100, 10))
    //         .stroke_color(Some(Rgb888::WHITE))
    //         .fill_color(Some(Rgb888::RED)),
    // );

    // display.draw(
    //     Line::new(Point::new(127, 127), Point::new(200, 150))
    //         .stroke_color(Some(Rgb888::WHITE))
    //         .fill_color(Some(Rgb888::RED))
    //         .into_iter(),
    // );

    let mut position = Point::new(60, 40);

    let offs = Point::new(100, 0);

    // display.draw(
    //     Line::new(Point::new(10, 10), Point::new(40, -6))
    //         // .stroke_color(Some(Rgb888::YELLOW))
    //         // .fill_color(Some(Rgb888::RED))
    //         .style(Style {
    //             stroke_color: Some(Rgb888::YELLOW),
    //             fill_color: Some(Rgb888::RED),
    //             test_color: Some(Rgb888::CYAN),
    //             ..Style::default()
    //         })
    //         .into_iter(),
    // );

    let len = 50.0;
    let center = Point::new(60, 60);
    let mut show_extra_perp = true;
    let mut width = 10i8;

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        let x = center.x + (angle.cos() * len) as i32;
        let y = center.y + (angle.sin() * len) as i32;

        // draw_line(&mut display, Point::new(127, 127), Point::new(x, y), width);

        // display.clear();
        // display.draw(
        //     Line::new(center, Point::new(x, y))
        //         .show_extra_perp()
        //         .style(Style {
        //             stroke_color: Some(Rgb888::WHITE),
        //             fill_color: Some(Rgb888::WHITE),
        //             test_color: Some(Rgb888::WHITE),
        //             stroke_width: 1,
        //             ..Style::default()
        //         })
        //         .into_iter(),
        // );

        // if angle < (2.0 * core::f32::consts::PI - 0.1) {
        //     angle += 0.1;
        // }
        // angle += 0.1;
        // thread::sleep(Duration::from_millis(100));

        display.clear();
        display.draw(
            Line::new(Point::new(30, 30), position)
                .show_extra_perp(show_extra_perp)
                // .stroke_color(Some(Rgb888::YELLOW))
                // .fill_color(Some(Rgb888::RED))
                .style(Style {
                    stroke_color: Some(Rgb888::YELLOW),
                    fill_color: Some(Rgb888::RED),
                    test_color: Some(Rgb888::CYAN),
                    stroke_width: width as u8,
                    ..Style::default()
                })
                .into_iter(),
        );
        // display.draw(
        //     Line::new(Point::new(30, 30) + offs, position + offs)
        //         // .stroke_color(Some(Rgb888::YELLOW))
        //         // .fill_color(Some(Rgb888::RED))
        //         .style(Style {
        //             stroke_color: Some(Rgb888::YELLOW),
        //             fill_color: Some(Rgb888::RED),
        //             test_color: Some(Rgb888::CYAN),
        //             stroke_width: 10,
        //             ..Style::default()
        //         })
        //         .into_iter(),
        // );

        display.draw(text_6x8!(
            &format!("W: {}", width),
            stroke_color = Some(Rgb888::RED),
        ));

        for event in display.get_input_events() {
            match event {
                SimulatorEvent::KeyDown { keycode, .. } => {
                    let delta = match keycode {
                        Keycode::Left => Point::new(-1, 0),
                        Keycode::Right => Point::new(1, 0),
                        Keycode::Up => Point::new(0, -1),
                        Keycode::Down => Point::new(0, 1),
                        _ => Point::zero(),
                    };
                    position += delta;

                    match keycode {
                        Keycode::Space => {
                            show_extra_perp = !show_extra_perp;
                        }
                        Keycode::O => width += 1,
                        Keycode::L => width = (width - 1).max(0),
                        _ => (),
                    }
                }

                _ => {}
            }
        }
    }
}
