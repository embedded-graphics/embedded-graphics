//! DOCS LOL

use crate::SimulatorDisplay;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

fn pixel(disp: &mut SimulatorDisplay<Rgb888>, x: i32, y: i32) {
    disp.draw_pixel(Pixel(Point::new(x, y), Rgb888::RED)).ok();
}

/***********************************************************************
 *                                                                     *
 *                            X BASED LINES                            *
 *                                                                     *
 ***********************************************************************/

fn x_perpendicular(
    disp: &mut SimulatorDisplay<Rgb888>,
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    xstep: i32,
    ystep: i32,
    einit: i32,
    w_left: i32,
    w_right: i32,
    winit: i32,
) {
    let mut p = 0;
    let mut q = 0;

    let threshold = dx - 2 * dy;
    let e_diag = -2 * dx;
    let e_square = 2 * dy;

    let mut y = y0;
    let mut x = x0;
    let mut error = einit;
    let mut tk = dx + dy - winit;

    while tk <= w_left {
        pixel(disp, x, y);
        if error >= threshold {
            x = x + xstep;
            error = error + e_diag;
            tk = tk + 2 * dy;
        }
        error = error + e_square;
        y = y + ystep;
        tk = tk + 2 * dx;
        q += 1;
    }

    y = y0;
    x = x0;
    error = -einit;
    tk = dx + dy + winit;

    while tk <= w_right {
        if p > 0 {
            pixel(disp, x, y);
        }
        if error > threshold {
            x = x - xstep;
            error = error + e_diag;
            tk = tk + 2 * dy;
        }
        error = error + e_square;
        y = y - ystep;
        tk = tk + 2 * dx;
        p += 1;
    }

    if q == 0 && p < 2 {
        pixel(disp, x0, y0); // we need this for very thin lines
    }
}

fn x_varthick_line(
    disp: &mut SimulatorDisplay<Rgb888>,
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    xstep: i32,
    ystep: i32,
    thickness: f32,
    pxstep: i32,
    pystep: i32,
) {
    let mut p_error = 0;
    let mut error = 0;
    let mut y = y0;
    let mut x = x0;
    let threshold = dx - 2 * dy;
    let e_diag = -2 * dx;
    let e_square = 2 * dy;
    let length = dx + 1;
    let d = ((dx * dx + dy * dy) as f32).sqrt();

    let mut p = 0;

    while p < length {
        let w_left = (thickness * 2.0 * d) as i32;
        let w_right = (thickness * 2.0 * d) as i32;
        x_perpendicular(
            disp, x, y, dx, dy, pxstep, pystep, p_error, w_left, w_right, error,
        );
        if error >= threshold {
            y = y + ystep;
            error = error + e_diag;
            if p_error >= threshold {
                x_perpendicular(
                    disp,
                    x,
                    y,
                    dx,
                    dy,
                    pxstep,
                    pystep,
                    p_error + e_diag + e_square,
                    w_left,
                    w_right,
                    error,
                );
                p_error = p_error + e_diag;
            }
            p_error = p_error + e_square;
        }
        error = error + e_square;
        x = x + xstep;

        p += 1;
    }
}

/***********************************************************************
 *                                                                     *
 *                            Y BASED LINES                            *
 *                                                                     *
 ***********************************************************************/

fn y_perpendicular(
    disp: &mut SimulatorDisplay<Rgb888>,
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    xstep: i32,
    ystep: i32,
    einit: i32,
    w_left: i32,
    w_right: i32,
    winit: i32,
) {
    let mut p = 0;
    let mut q = 0;
    let threshold = dy - 2 * dx;
    let e_diag = -2 * dy;
    let e_square = 2 * dx;

    let mut y = y0;
    let mut x = x0;
    let mut error = -einit;
    let mut tk = dx + dy + winit;

    while tk <= w_left {
        pixel(disp, x, y);
        if error > threshold {
            y = y + ystep;
            error = error + e_diag;
            tk = tk + 2 * dx;
        }
        error = error + e_square;
        x = x + xstep;
        tk = tk + 2 * dy;
        q += 1;
    }

    y = y0;
    x = x0;
    error = einit;
    tk = dx + dy - winit;

    while tk <= w_right {
        if p > 0 {
            pixel(disp, x, y);
        }
        if error >= threshold {
            y = y - ystep;
            error = error + e_diag;
            tk = tk + 2 * dx;
        }
        error = error + e_square;
        x = x - xstep;
        tk = tk + 2 * dy;
        p += 1;
    }

    if q == 0 && p < 2 {
        pixel(disp, x0, y0); // we need this for very thin lines
    }
}

fn y_varthick_line(
    disp: &mut SimulatorDisplay<Rgb888>,
    x0: i32,
    y0: i32,
    dx: i32,
    dy: i32,
    xstep: i32,
    ystep: i32,
    thickness: f32,
    pxstep: i32,
    pystep: i32,
) {
    let mut p_error = 0;
    let mut error = 0;
    let mut y = y0;
    let mut x = x0;
    let threshold = dy - 2 * dx;
    let e_diag = -2 * dy;
    let e_square = 2 * dx;
    let length = dy + 1;
    let d = ((dx * dx + dy * dy) as f32).sqrt();

    let mut p = 0;

    while p < length {
        let w_left = (thickness * 2.0 * d) as i32;
        let w_right = (thickness * 2.0 * d) as i32;
        y_perpendicular(
            disp, x, y, dx, dy, pxstep, pystep, p_error, w_left, w_right, error,
        );
        if error >= threshold {
            x = x + xstep;
            error = error + e_diag;
            if p_error >= threshold {
                y_perpendicular(
                    disp,
                    x,
                    y,
                    dx,
                    dy,
                    pxstep,
                    pystep,
                    p_error + e_diag + e_square,
                    w_left,
                    w_right,
                    error,
                );
                p_error = p_error + e_diag;
            }
            p_error = p_error + e_square;
        }
        error = error + e_square;
        y = y + ystep;

        p += 1;
    }
}

/***********************************************************************
 *                                                                     *
 *                                ENTRY                                *
 *                                                                     *
 ***********************************************************************/

/// DOCS LOL
pub fn draw_varthick_line(
    disp: &mut SimulatorDisplay<Rgb888>,
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
    thickness: f32,
) {
    let mut dx = x1 - x0;
    let mut dy = y1 - y0;
    let mut xstep = 1;
    let mut ystep = 1;

    if dx < 0 {
        dx = -dx;
        xstep = -1;
    }
    if dy < 0 {
        dy = -dy;
        ystep = -1;
    }

    if dx == 0 {
        xstep = 0
    };
    if dy == 0 {
        ystep = 0
    };

    let (pxstep, pystep) = match xstep + ystep * 4 {
        -5 => (-1, 1),
        -1 => (-1, 0),
        3 => (1, 1),
        -4 => (0, -1),
        0 => (0, 0),
        4 => (0, 1),
        -3 => (-1, -1),
        1 => (-1, 0),
        5 => (1, -1),
        v => unreachable!(v),
    };

    if dx > dy {
        x_varthick_line(
            disp, x0, y0, dx, dy, xstep, ystep, thickness, pxstep, pystep,
        );
    } else {
        y_varthick_line(
            disp, x0, y0, dx, dy, xstep, ystep, thickness, pxstep, pystep,
        );
    }
}
