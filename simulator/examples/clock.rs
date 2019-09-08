//! # Example: Clock
//!
//! ![Screenshot of clock example]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAgAAAAIAAgMAAACJFjxpAAAACVBMVEUAAAD///8vND9Nm1xmAAAFfUlEQVR42uzbTZLaMBRF4TO5+7uTu8msMhXooKoGEzBSR0/2GfDzkOSvXOUh/PrPcXZ2dnZ2dnb2QbllXmktQFovIRYD5BoABtBzw2qA28UbAEBbhNUASQIt09IDwmoAJQFo+fnviwEeHe9na1YDKDFbgFYTsBjg8cHehC4H2DjVm2sXAyiBlwEoWQyg5O0NSwGU7NiyEEDJrk3LAJTs3LYIQMnujWsAkv10rwBI4ANBfUAidifF1QGKMXszSXGAYj4CkNQGxIB28wHiyoCEjwGK6wIULpl9GQClLiDuASCuCki4JnbUtimuCVDolFITENOruCJAoVtKRUBMv+J6gJiexdUACl1TqgFi+hbXAih0TqkFiOldXAmg0D2lEiCmf3EdgMKAlDqAmBHFVQAKQ1KqAGLGFFcBMCgVAciMKjUAYVhyBYBGAlIBEDOueH6AwsCUAgAzsgKAMDR5doBGAzI7IGZs8eyAMDZNDpAxY8vcgBgxtnhuABCGpqkBMoAZWmYGxF8MJYwqnhnAJWEwg9LEAIUGEYNSJgaYa2OfxokBoQFeUexkyrMCFN8B1B+gZFqA5e8A3BsgMy0gRv4OUG+AIZ4VAF8C0a7svoAAmhSgtFtusLkk9wQIQJkUYGiYIK6pF6A1KSDeWOb+AM8J2Dpc3QGaEqBsrnN3QKYEmLt0e8vbPZVNCYi3b7C7Azwj4MnzIpJcEHCzAPBoEv4JUBkA4pIPAJCf/1Elt6Npnx9Owr8BpA7ga34AQNhIALTj2+WAx5MXAHIdADk8AEMDQNLiflIVILOZBgBIJQBpgG8K2qQ64Pl6DwB4NgDPktvRf4Lb97vJiwDVAqDDA0j+vkAubU4C1APIPw0g5QBvdQLqAcKPA+QTcAJqAS4Z8N1sq1IAmVcSoF4AUhBAAB8agEHHBsiQFQHh1QS4D0AuCcCgYwNk8KEBCHRsAAYfGyCj1QAy7yRwBwApCyDo4ACMDw4Q8rEBBB0cgPHBAbIODkB4IcBv9u7YRnIoBoJoOZ1fO51/Koe9xSag0afIEcuRReABDUimwoWsTwHybIDslwOQ3g7AfjtAgcQvBqAE5BcDiIG8GSAAvRlgALyALwD85FJAfmoEeHyCBYwFPP4qfh7w+Of4aQAB+dUAEvNuALCABSzgGwAyTxQWsIAFLGABC1hAGwDhgeQFLGABC1hAI4BMfWEBC1hAIwChPHkBC1hAK4BMdWEBC2gFIBQnL2AB3QCmtnQDqBzAApoBCKXJC1hAO4BMZWEB7QCEwuQFdASYutIRoFIAHQGhLLUEUAlwT4CpKj0BKgTQExAAcTQBqCmAVOwQQO4KMJjTGdIVIBOOJxO6AhIKUtoCCCXJbQEyFYW+gFCQGgOoAbgzwJwvnQEqAdAYQDie3BtgTpfeABUAaA0gHE7uDjBnS3eAwtEUmgM4DXB7gMzJQn9AOJgGAIg5VzwAcHIDhQEATgI8AiBzqjACQDiUPAVgzpQpAIUjKQwBnNogHgM4s4HCGMCZDeJBgBMbKAwCnNggHgW4fwOFUYD7N4iHAZTbFxgGuHuDeBzgXkHMPIDCbSkMBNy5QTwSoNy4wEgACb+JS+nvGQ8FkA9/nmb+FzMVoNwBUBgLIAEQFxOA4sEAYsBczUDMZIDiDwEJowEoFpcTCcMBJJ8AFI8HkHw0IPMBHwgU8w0AJdcPvwKAkqtnXwJAybWjrwH8a+8OcQCGYRgAhuR/If7/V7aqYKCxtUlDVkybOgfKy8v0FSMAq9MXrAC8sOi4GYB+nVR01g4QAN4Bcu/3A/TFpaBugMimu9SMHaCrL31uB4jAHQZInM/UD7AJOAEJst4PEIGdB5CAWu8HWIYjersfYOXrcj/AZDKZTCa/5AK/3Qdyk7dphgAAAABJRU5ErkJggg==)
//!
//! This example shows some more advanced usage of Embedded Graphics. It draws a round clock face
//! with hour, minute and second hands. A digital clock is drawn in the middle of the clock. The
//! whole thing is updated with your computer's local time every 50ms.

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

const DISP_SIZE: i32 = 256;
const CENTER: Point = Point::new(DISP_SIZE / 2, DISP_SIZE / 2);
const SIZE: i32 = 120;

/// Start at the top of the circle
const START: f32 = -FRAC_PI_2;

/// Convert a polar coordinate (angle/distance) into an (X, Y) coordinate centered around `CENTER`
fn polar(angle: f32, radius: f32) -> Point {
    CENTER + Point::new((angle.cos() * radius) as i32, (angle.sin() * radius) as i32)
}

/// Draw a circle and 12 tics as a simple clock face
fn draw_face() -> impl Iterator<Item = Pixel<BinaryColor>> {
    let tic_len = 10.0;

    // Use the circle macro to create the outer face
    let face = egcircle!(
        CENTER,
        SIZE as u32,
        stroke = Some(BinaryColor::On),
        stroke_width = 2
    );

    // Create 12 `Line`s starting from the outer edge and drawing inwards by `tic_len` pixels
    let tics = (0..12).into_iter().map(move |index| {
        // Start angle around the circle, in radians
        let angle = START + (PI * 2.0 / 12.0) * index as f32;

        // Start point on circumference
        let start = polar(angle, SIZE as f32);

        // End point; start point offset by `tic_len` pixels towards the circle center
        let end = polar(angle, SIZE as f32 - tic_len);

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

    let end = polar(seconds_radians, SIZE as f32);

    // Basic line hand
    let hand = Line::new(CENTER, end).stroke(Some(BinaryColor::On));

    // Offset from end of hand
    let decoration_offset = polar(seconds_radians, SIZE as f32 - 20.0);

    // Add a fancy circle near the end of the hand
    let decoration = Circle::new(decoration_offset, 5)
        .fill(Some(BinaryColor::Off))
        .stroke(Some(BinaryColor::On));

    hand.into_iter().chain(decoration)
}

/// Draw the hour hand (0-11)
fn draw_hour_hand(hour: u32) -> Line<BinaryColor> {
    // Convert hour into a position around the circle in radians
    let hour_radians = ((hour as f32 / 12.0) * 2.0 * PI) + START;

    let hand_len = SIZE as f32 - 60.0;

    let end = polar(hour_radians, hand_len);

    // Basic line hand
    Line::new(CENTER, end).stroke(Some(BinaryColor::On))
}

/// Draw the minute hand (0-59)
fn draw_minute_hand(minute: u32) -> Line<BinaryColor> {
    // Convert minute into a position around the circle in radians
    let minute_radians = ((minute as f32 / 60.0) * 2.0 * PI) + START;

    let hand_len = SIZE as f32 - 30.0;

    let end = polar(minute_radians, hand_len);

    // Basic line hand
    Line::new(CENTER, end).stroke(Some(BinaryColor::On))
}

/// Draw digital clock just above center with black text on a white background
///
/// NOTE: The formatted time str must be passed in as references to temporary values in a
/// function can't be returned.
fn draw_digital_clock<'a>(time_str: &'a str) -> impl Iterator<Item = Pixel<BinaryColor>> + 'a {
    let text = Font12x16::render_str(&time_str)
        .stroke(Some(BinaryColor::Off))
        .translate(CENTER - Size::new(48, 48));

    // Add a background around the time digits. Note that there is no bottom-right padding as this
    // is added by the font renderer itself
    let background = Rectangle::new(text.top_left() - Size::new(3, 3), text.bottom_right())
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

        // Draw a small circle over the hands in the center of the clock face. This has to happen
        // after the hands are drawn so they're covered up
        display.draw(
            Circle::new(CENTER, 4)
                .fill(Some(BinaryColor::On))
                .stroke(Some(BinaryColor::On)),
        );

        let end = display.run_once();

        if end {
            break;
        }

        thread::sleep(Duration::from_millis(50));
    }
}
