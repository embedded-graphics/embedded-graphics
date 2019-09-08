use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::BinaryDisplay;
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

fn perp_octant1(
    display: &mut BinaryDisplay,
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    einit: i32,
    width: i32,
    winit: i32,
) {
    let threshold = dx - 2 * dy;
    let E_diag = -2 * dx;
    let E_square = 2 * dy;
    let wthr = 2.0 * width as f32 * ((dx * dx + dy * dy) as f32).sqrt();

    let mut x = x0;
    let mut y = y0;
    let mut error = einit;
    let mut tk = dx + dy - winit;

    while tk as f32 <= wthr {
        display.set_pixel(x as usize, y as usize, BinaryColor::On);

        if error > threshold {
            x -= 1;
            error += E_diag;
            tk += 2 * dy;
        }
        error += E_square;
        y += 1;
        tk += 2 * dx;
    }

    let mut x = x0;
    let mut y = y0;
    let mut error = -einit;
    let mut tk = dx + dy + winit;

    while tk as f32 <= wthr {
        display.set_pixel(x as usize, y as usize, BinaryColor::On);
        if error > threshold {
            x += 1;
            error += E_diag;
            tk += 2 * dy;
        }
        error += E_square;
        y -= 1;
        tk += 2 * dx;
    }
}

fn thick_octant1(display: &mut BinaryDisplay, x0: i32, y0: i32, x1: i32, y1: i32, width: i32) {
    let dx = x1 - x0;
    let dy = y1 - y0;
    let mut p_error = 0;
    let mut error = 0;
    let mut y = y0;
    let mut x = x0;
    let threshold = dx - 2 * dy;
    let E_diag = -2 * dx;
    let E_square = 2 * dy;
    let length = dx + 1;

    for p in 1..length {
        perp_octant1(display, x, y, dx, dy, p_error, width, error);
        if error > threshold {
            y += 1;
            error += E_diag;
            if p_error > threshold {
                perp_octant1(
                    display,
                    x,
                    y,
                    dx,
                    dy,
                    p_error + E_diag + E_square,
                    width,
                    error,
                );
                p_error += E_diag;
            }
            p_error += E_square;
        }
        error += E_square;
        x += 1;
    }
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Delete me and update 'strokes' demo")
        .size(320, 256)
        .scale(3)
        .build_binary();

    // display.set_pixel(10, 10, BinaryColor::On);

    thick_octant1(&mut display, 20, 100, 100, 20, 5);

    loop {
        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
