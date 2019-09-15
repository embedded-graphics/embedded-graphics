use embedded_graphics::egline;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use embedded_graphics_simulator::RgbDisplay;
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
    let mut p = start;

    let width_threshold_sq = 4 * width.pow(2) * (delta.x.pow(2) + delta.y.pow(2));

    let (mut error, e_diag, e_square, mut width_accum, threshold) = if delta.x > delta.y {
        (
            p_error_initial,
            delta.x * -2,
            delta.y * 2,
            delta.x + delta.y - error_initial,
            delta.x - 2 * delta.y,
        )
    } else {
        (
            -p_error_initial,
            delta.y * -2,
            delta.x * 2,
            delta.x + delta.y + error_initial,
            delta.y - 2 * delta.x,
        )
    };

    // Left hand side of line
    while width_accum.pow(2) <= width_threshold_sq {
        display.set_pixel(p.x as usize, p.y as usize, Rgb888::RED);

        if error > threshold {
            if delta.x > delta.y {
                p += Point::new(-direction.x, 0);
            } else {
                p += Point::new(0, direction.y);
            };

            error += e_diag;

            if delta.x > delta.y {
                width_accum += 2 * delta.y;
            } else {
                width_accum += 2 * delta.x;
            }
        }

        error += e_square;

        if delta.x > delta.y {
            p += Point::new(0, direction.y);
        } else {
            p += Point::new(-direction.x, 0);
        };

        if delta.x > delta.y {
            width_accum += 2 * delta.x;
        } else {
            width_accum += 2 * delta.y;
        }
    }

    let mut p = start;

    let (mut error, mut width_accum) = if delta.x > delta.y {
        (-p_error_initial, delta.x + delta.y + error_initial)
    } else {
        (p_error_initial, delta.x + delta.y - error_initial)
    };

    // Right hand side of line
    while width_accum.pow(2) <= width_threshold_sq {
        display.set_pixel(p.x as usize, p.y as usize, Rgb888::YELLOW);

        if error > threshold {
            if delta.x > delta.y {
                p += Point::new(direction.x, 0);
            } else {
                p += Point::new(0, -direction.y);
            };

            error += e_diag;

            if delta.x > delta.y {
                width_accum += 2 * delta.y;
            } else {
                width_accum += 2 * delta.x;
            }
        }

        error += e_square;

        if delta.x > delta.y {
            p += Point::new(0, -direction.y);
        } else {
            p += Point::new(direction.x, 0);
        };

        if delta.x > delta.y {
            width_accum += 2 * delta.x;
        } else {
            width_accum += 2 * delta.y;
        }
    }
}

fn draw_line(display: &mut RgbDisplay, p0: Point, p1: Point, width: i32) {
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
            // p.y += 1;
            if delta.x > delta.y {
                p += Point::new(0, direction.y);
            } else {
                p += Point::new(direction.x, 0);
            };

            error += e_diag;

            if p_error >= threshold {
                p_error += e_diag;

                draw_perp(
                    display,
                    p,
                    delta,
                    direction,
                    p_error + e_square,
                    width,
                    error,
                );
            }

            p_error += e_square;
        }

        error += e_square;
        // p.x += 1;
        if delta.x > delta.y {
            p += Point::new(direction.x, 0);
        } else {
            p += Point::new(0, direction.y);
        };
    }

    // Draw center line using existing e-g `Line`
    display.draw(egline!(p0, p1, stroke_color = Some(Rgb888::WHITE)));
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(256, 256)
        .scale(3)
        .build_rgb();

    // draw_line(&mut display, 20, 20, 100, 50, 1);

    // draw_line(&mut display, 10, 100, 50, 100, 1);

    let mut angle: f32 = 0.0;

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        display.clear();

        let x = 127 + (angle.cos() * 120.0) as i32;
        let y = 127 + (angle.sin() * 120.0) as i32;

        draw_line(&mut display, Point::new(127, 127), Point::new(x, y), 10);
        // draw_line(&mut display, Point::new(127, 127), Point::new(150, 10), 10);

        angle += 0.1;

        thread::sleep(Duration::from_millis(50));
    }
}
