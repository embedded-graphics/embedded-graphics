use embedded_graphics::egline;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::DisplayBuilder;
use embedded_graphics_simulator::RgbDisplay;
use std::thread;
use std::time::Duration;

// TODO: Wtf does `winit` mean?
fn draw_perp(
    display: &mut RgbDisplay,
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    error_initial: i32,
    width: i32,
    winit: i32,
) {
    let mut error = error_initial;
    let mut x = x0;
    let mut y = y0;

    let threshold = dx - 2 * dy;

    let width_threshold = 4 * (width * width) * (dx * dx + dy * dy);

    let e_diag = -2 * dx;
    let e_square = 2 * dy;

    // TODO: WTF does `tk` mean?
    let mut tk = dx + dy - winit;

    while (tk * tk) <= width_threshold {
        display.set_pixel(x as usize, y as usize, Rgb888::RED);

        if error > threshold {
            x -= 1;
            error += e_diag;
            tk += 2 * dy;
        }

        error += e_square;
        y += 1;
        tk += 2 * dx;
    }

    // TODO: WTF does `tk` mean?
    let mut tk = dx + dy + winit;
    let mut error = -error_initial;
    let mut x = x0;
    let mut y = y0;

    while (tk * tk) <= width_threshold {
        display.set_pixel(x as usize, y as usize, Rgb888::GREEN);

        if error > threshold {
            x += 1;
            error += e_diag;
            tk += 2 * dy;
        }

        error += e_square;
        y -= 1;
        tk += 2 * dx;
    }
}

fn draw_line(display: &mut RgbDisplay, x0: i32, y0: i32, x1: i32, y1: i32, width: i32) {
    let dx = x1 - x0;
    let dy = y1 - y0;

    let mut error = 0;
    // Perpendicular error
    let mut p_error = 0;
    let mut y = y0;

    let threshold = dx - 2 * dy;

    let e_diag = -2 * dx;
    let e_square = 2 * dy;

    for x in x0..=(x0 + dx) {
        draw_perp(display, x, y, dx, dy, p_error, width, error);

        if error > threshold {
            y += 1;
            error += e_diag;

            if p_error > threshold {
                draw_perp(
                    display,
                    x,
                    y,
                    dx,
                    dy,
                    p_error + e_diag + e_square,
                    width,
                    error,
                );

                p_error += e_diag;
            }

            p_error += e_square;
        }

        error += e_square;
    }

    // Draw center line using existing e-g `Line`
    display.draw(egline!(
        (x0, y0),
        (x1, y1),
        stroke_color = Some(Rgb888::WHITE)
    ));
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(256, 256)
        .scale(8)
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

        let x = dbg!(127 + (angle.cos() * 120.0) as i32);
        let y = dbg!(127 + (angle.sin() * 120.0) as i32);

        draw_line(&mut display, 127, 127, x, y, 10);

        angle += 0.1;

        thread::sleep(Duration::from_millis(50));
    }
}
