//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code

use chrono::{Local, Timelike};
use core::f32::consts::{FRAC_PI_2, PI};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::Line;
use embedded_graphics::{egcircle, egline};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static DISP_SIZE: i32 = 256;
static CENTER: i32 = 127;

/// Start at the top of the circle
static START: f32 = -FRAC_PI_2;

/// Draw a circle and 12 tics as a simple clock face
fn draw_face() -> impl Iterator<Item = Pixel<BinaryColor>> {
    let tic_len = 10.0;

    // Use the circle macro to create the outer face
    let face = egcircle!(
        (CENTER, CENTER),
        CENTER as u32,
        stroke = Some(BinaryColor::On),
        stroke_width = 2
    );

    // Create 12 `Line`s starting from the outer edge and drawing inwards by `tic_len` pixels
    let tics = (0..12).into_iter().map(move |index| {
        // Start angle around the circle, in radians
        let angle = START + (PI * 2.0 / 12.0) * index as f32;

        // Start point on circumference
        let start = Point::new(
            CENTER + (angle.cos() * (CENTER as f32)) as i32,
            CENTER + (angle.sin() * (CENTER as f32)) as i32,
        );

        // End point; start point offset by `tic_len` pixels towards the circle center
        let end = start
            - Point::new(
                (angle.cos() * tic_len) as i32,
                (angle.sin() * tic_len) as i32,
            );

        Line::new(start, end)
            .stroke(Some(BinaryColor::On))
            .into_iter()
    });

    // Create a single iterator of pixels, first iterating over the circle, then over the 12 lines
    // generated
    face.into_iter().chain(tics.flatten())
}

fn main() {
    // Start at top of circle
    let mut seconds_radians: f32 = START;

    let mut display = DisplayBuilder::new()
        .title("Clock")
        .size(DISP_SIZE as usize, DISP_SIZE as usize)
        .scale(2)
        .build_rgb();

    loop {
        display.clear();

        display.draw(draw_face());

        display.draw(egline!(
            (CENTER, CENTER),
            (
                CENTER + (seconds_radians.cos() * (CENTER as f32)) as i32,
                CENTER + (seconds_radians.sin() * (CENTER as f32)) as i32
            ),
            stroke = Some(BinaryColor::On)
        ));

        let time = Local::now();

        seconds_radians = ((time.second() as f32 / 60.0) * 2.0 * PI) + START;

        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }
}
