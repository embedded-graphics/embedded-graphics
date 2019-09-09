use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use embedded_graphics_simulator::RgbDisplay;
use std::thread;
use std::time::Duration;

fn draw_perp_left(display: &mut RgbDisplay, x0: i32, y0: i32, dx: i32, dy: i32, error: i32) {
    let mut error = error;
    let mut x = x0;

    let threshold = dx - 2 * dy;

    let e_diag = -2 * dx;
    let e_square = 2 * dy;

    for y in y0..(y0 + 5) {
        display.set_pixel(x as usize, y as usize, Rgb888::BLUE);

        if error > threshold {
            x -= 1;
            error += e_diag;
        }

        error += e_square;
    }
}

fn draw_perp_right(display: &mut RgbDisplay, x0: i32, y0: i32, dx: i32, dy: i32, error: i32) {
    let mut error = -error;
    let mut x = x0;

    let threshold = dx - 2 * dy;

    let e_diag = -2 * dx;
    let e_square = 2 * dy;

    for y in 0..5 {
        display.set_pixel(x as usize, (y0 - y) as usize, Rgb888::GREEN);

        if error > threshold {
            x += 1;
            error += e_diag;
        }

        error += e_square;
    }
}

fn draw_line(display: &mut RgbDisplay, x0: i32, y0: i32, dx: i32, dy: i32) {
    let mut error = 0;
    // Perpendicular error
    let mut p_error = 0;
    let mut y = y0;

    let threshold = dx - 2 * dy;

    let e_diag = -2 * dx;
    let e_square = 2 * dy;

    for x in x0..(x0 + dx) {
        draw_perp_left(display, x, y, dx, dy, p_error);
        draw_perp_right(display, x, y, dx, dy, p_error);

        display.set_pixel(x as usize, y as usize, Rgb888::RED);

        if error > threshold {
            y += 1;
            error = error + e_diag;

            if p_error > threshold {
                draw_perp_left(display, x, y, dx, dy, p_error + e_diag + e_square);
                draw_perp_right(display, x, y, dx, dy, p_error + e_diag + e_square);

                p_error = e_square;
            }

            p_error += e_square;
        }

        error += e_square;
    }
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(320, 256)
        .scale(3)
        .build_rgb();

    // display.set_pixel(10, 10, BinaryColor::On);

    // thick_octant1(&mut display, 20, 100, 100, 20, 5);

    draw_line(&mut display, 10, 50, 50, 40);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
