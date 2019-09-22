use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics::{egline, fonts::Font6x8, primitives::Line, text_6x8};
use embedded_graphics_simulator::DisplayBuilder;
use embedded_graphics_simulator::RgbDisplay;
use integer_sqrt::IntegerSquareRoot;
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
    if width == 1 {
        display.draw(egline!(p0, p1, stroke_color = Some(Rgb888::WHITE)));

        return;
    }

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

    // Draw center line using existing e-g `Line`. Uncomment to debug.
    // display.draw(egline!(
    //     p0,
    //     p0 + Point::new(20, 0),
    //     stroke_color = Some(Rgb888::WHITE)
    // ));
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(256, 256)
        .scale(4)
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

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        // display.clear();

        let x = 127 + (angle.cos() * 120.0) as i32;
        let y = 127 + (angle.sin() * 120.0) as i32;

        let width = 4;

        draw_line(&mut display, Point::new(127, 127), Point::new(x, y), width);

        // display.draw(
        //     Line::new(Point::new(127, 127), Point::new(x, y))
        //         .stroke_color(Some(Rgb888::WHITE))
        //         .into_iter(),
        // );

        if angle < (2.0 * core::f32::consts::PI - 0.1) {
            angle += 0.1;
        }

        thread::sleep(Duration::from_millis(10));
    }
}
