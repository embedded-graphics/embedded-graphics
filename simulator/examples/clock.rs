//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::{egcircle, egline, egrectangle, egtriangle};
use embedded_graphics_simulator::DisplayBuilder;
use std::thread;
use std::time::Duration;

static DISP_SIZE: i32 = 256;
static CENTER: i32 = 127;

fn main() {
    // Start at top of circle
    let mut angle: f32 = -core::f32::consts::FRAC_2_PI;

    let mut display = DisplayBuilder::new()
        .title("Clock")
        .size(DISP_SIZE as usize, DISP_SIZE as usize)
        .scale(2)
        .build_rgb();

    display.draw(egcircle!(
        (CENTER, CENTER),
        CENTER as u32,
        stroke = Some(BinaryColor::On)
    ));

    loop {
        display.draw(egline!(
            (CENTER, CENTER),
            (
                CENTER + (angle.cos() * (CENTER as f32)) as i32,
                CENTER + (angle.sin() * (CENTER as f32)) as i32
            ),
            stroke = Some(BinaryColor::On)
        ));

        angle += 0.1;

        println!(
            "Angle sin {}, cos {}",
            (angle.sin() * 10.0) as i32,
            (angle.cos() * 10.0) as i32
        );

        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }
}
