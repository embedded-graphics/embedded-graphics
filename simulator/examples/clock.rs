//! Demonstrate usage of primitives like `fill.rs` but use macros instead for shorter code
//!
//! # Development findings
//!
//! Some of these should probably be turned into issues and discussed before this example is
//! released.
//!
//! 1. The primitives macros should accept both `()` tuples and variables/expressions for their
//! coordinate positions. In this code, I often find myself replacing a macro with a call to
//! `[Primitive]::new()` so I can pass in a `Point`
//! 2. I often want a relative offset from a `Point`. Maybe we implement `Transform` for `Point`? Or
//! a new trait/set of methods to do relative moves.
//! 3. The font docs could use screenshots of what the text looks like
//! 4. Some sort of "layout" thing, like being able to say "position this thing centered in the
//! display" - see digital clock positioning for example
//! 5. The font docs should give examples of `write!()`ing into a fixed length buffer using
//! `arrayvec`. This example uses `format!()` because the simulator is `std`, but this won't work in
//! `no-std` environments

use chrono::{Local, Timelike};
use core::f32::consts::{FRAC_PI_2, PI};
use embedded_graphics::egcircle;
use embedded_graphics::fonts::Font12x16;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line, Rectangle};
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

/// Draw the seconds hand given a seconds value (0 - 59)
fn draw_seconds_hand(seconds: u32) -> impl Iterator<Item = Pixel<BinaryColor>> {
    // Convert seconds into a position around the circle in radians
    let seconds_radians = ((seconds as f32 / 60.0) * 2.0 * PI) + START;

    let end = Point::new(
        CENTER + (seconds_radians.cos() * (CENTER as f32)) as i32,
        CENTER + (seconds_radians.sin() * (CENTER as f32)) as i32,
    );

    // Basic line hand
    let hand = Line::new((CENTER, CENTER).into(), end).stroke(Some(BinaryColor::On));

    // Offset from end of hand
    let decoration_offset = Point::new(
        (seconds_radians.cos() * (20.0)) as i32,
        (seconds_radians.sin() * (20.0)) as i32,
    );

    // Add a fancy circle near the end of the hand
    let decoration = Circle::new(end - decoration_offset, 5)
        .fill(Some(BinaryColor::Off))
        .stroke(Some(BinaryColor::On))
        .stroke_width(1);

    hand.into_iter().chain(decoration)
}

/// Draw the hour hand (0-11)
fn draw_hour_hand(hour: u32) -> Line<BinaryColor> {
    // Convert hour into a position around the circle in radians
    let hour_radians = ((hour as f32 / 12.0) * 2.0 * PI) + START;

    let hand_len = CENTER as f32 - 60.0;

    let end = Point::new(
        CENTER + (hour_radians.cos() * hand_len) as i32,
        CENTER + (hour_radians.sin() * hand_len) as i32,
    );

    // Basic line hand
    Line::new((CENTER, CENTER).into(), end).stroke(Some(BinaryColor::On))
}

/// Draw the minute hand (0-59)
fn draw_minute_hand(minute: u32) -> Line<BinaryColor> {
    // Convert minute into a position around the circle in radians
    let minute_radians = ((minute as f32 / 60.0) * 2.0 * PI) + START;

    let hand_len = CENTER as f32 - 30.0;

    let end = Point::new(
        CENTER + (minute_radians.cos() * hand_len) as i32,
        CENTER + (minute_radians.sin() * hand_len) as i32,
    );

    // Basic line hand
    Line::new((CENTER, CENTER).into(), end).stroke(Some(BinaryColor::On))
}

/// Draw digital clock just above center with black text on a white background
///
/// NOTE: The formatted time str must be passed in as references to temporary values in a
/// function can't be returned.
fn draw_digital_clock<'a>(time_str: &'a str) -> impl Iterator<Item = Pixel<BinaryColor>> + 'a {
    let text = Font12x16::<BinaryColor>::render_str(&time_str)
        .stroke(Some(BinaryColor::Off))
        .translate((CENTER - 48, CENTER - 48).into());

    // Add some padding on the background around the time digits, here it's 2px in each direction
    let offset = Size::new(2, 2);

    let background = Rectangle::new(text.top_left() - offset, text.bottom_right() + offset)
        .fill(Some(BinaryColor::On));

    // Draw the white background first, then the black text. Order matters here
    background.into_iter().chain(text)
}

fn main() {
    let mut display = DisplayBuilder::new()
        .title("Clock")
        .size(DISP_SIZE as usize, DISP_SIZE as usize)
        .scale(2)
        .build_binary();

    loop {
        display.clear();

        display.draw(draw_face());

        let time = Local::now();

        display.draw(draw_hour_hand(time.hour()));
        display.draw(draw_minute_hand(time.minute()));
        display.draw(draw_seconds_hand(time.second()));

        // NOTE: In no-std environments, consider using
        // [arrayvec](https://stackoverflow.com/a/39491059/383609) and a fixed size buffer
        let clock_text = format!(
            "{:02}:{:02}:{:02}",
            time.hour(),
            time.minute(),
            time.second()
        );

        // Draw digital clock just above center
        display.draw(draw_digital_clock(&clock_text));

        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }
}
