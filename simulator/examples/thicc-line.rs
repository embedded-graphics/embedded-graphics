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
    let mut error = p_error_initial;
    let mut p = start;

    let threshold = delta.x - 2 * delta.y;

    let width_threshold_sq = 4 * width.pow(2) * (delta.x.pow(2) + delta.y.pow(2));

    let e_diag = -2 * delta.x;
    let e_square = 2 * delta.y;

    let mut width_accum = delta.x + delta.y - error_initial;

    while width_accum.pow(2) <= width_threshold_sq {
        display.set_pixel(p.x as usize, p.y as usize, Rgb888::RED);

        if error > threshold {
            p -= Point::new(direction.x, 0);
            error += e_diag;
            width_accum += 2 * delta.y;
        }

        error += e_square;
        p += Point::new(0, direction.y);
        width_accum += 2 * delta.x;
    }

    let mut width_accum = delta.x + delta.y + error_initial;
    let mut error = -p_error_initial;
    let mut p = start;

    while width_accum.pow(2) <= width_threshold_sq {
        display.set_pixel(p.x as usize, p.y as usize, Rgb888::GREEN);

        if error > threshold {
            p.x += 1;
            error += e_diag;
            width_accum += 2 * delta.y;
        }

        error += e_square;
        p.y -= 1;
        width_accum += 2 * delta.x;
    }
}

fn draw_line(display: &mut RgbDisplay, p0: Point, p1: Point, width: i32) {
    let delta = p1 - p0;

    let direction = match (delta.x < 0, delta.y < 0) {
        (false, false) => Point::new(1, 1),
        (false, true) => Point::new(1, -1),
        (true, false) => Point::new(-1, 1),
        (true, true) => Point::new(-1, -1),
    };

    let mut error = 0;
    // Perpendicular error
    let mut p_error = 0;

    let mut p = p0;

    let threshold = delta.x - 2 * delta.y;

    let e_diag = -2 * delta.x;
    let e_square = 2 * delta.y;

    for _ in 0..=delta.x.max(delta.y).abs() {
        // while p != p1 {
        println!("{:?}", p);
        draw_perp(display, p, delta, direction, p_error, width, error);

        if error > threshold {
            // p.y += 1;
            p += Point::new(0, direction.y);
            error += e_diag;

            if p_error > threshold {
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
        p += Point::new(direction.x, 0);
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

        angle += 0.1;

        thread::sleep(Duration::from_millis(50));
    }
}
