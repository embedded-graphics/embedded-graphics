//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<Rgb565> for SimPixelColor` from `src/lib` to convert the image into
//! a colour pixel iterator. The simulator uses the `ColorOled` theme to display the image in colour
//!
//! Note that this requires the `bmp` feature to be turned on for `embedded-graphics`. Turn it on
//! with the following in `Cargo.toml`:
//!
//! [dependencies]
//! embedded-graphics = { version = "*", features = [ "bmp" ] }

use embedded_graphics::image::ImageBmp;
use embedded_graphics::pixelcolor::Rgb565;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::SimulatorDisplay;
use std::thread;
use std::time::Duration;

fn main() {
    let image: ImageBmp<Rgb565> = ImageBmp::new(include_bytes!("./rust-pride.bmp")).unwrap();

    let mut display = SimulatorDisplay::new(Size::new(304, 128));
    let mut window = display.build_window().title("BMP image").scale(2).build();

    image.draw(&mut display);

    loop {
        window.update(&display);

        let end = window.handle_events();
        if end {
            break;
        }

        thread::sleep(Duration::from_millis(200));
    }
}
