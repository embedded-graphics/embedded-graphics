//! Draw a 16BPP BMP image onto a monochrome display
//!
//! This example uses `impl From<u16> for SimPixelColor` from `src/lib` to convert the image into
//! a black and white pixel iterator. The simulator doesn't currently support drawing with colour.
//!
//! Note that this requires the `tga` feature to be turned on for `embedded-graphics`. Turn it on
//! with the following in `Cargo.toml`:
//!
//! [dependencies]
//! embedded-graphics = { version = "*", features = [ "tga" ] }

use embedded_graphics::image::ImageTga;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;
use embedded_graphics_simulator::SimulatorDisplay;
use std::thread;
use std::time::Duration;

fn main() {
    let image: ImageTga<Rgb888> = ImageTga::new(include_bytes!("./rust-pride.tga")).unwrap();

    let mut display = SimulatorDisplay::new(Size::new(304, 128));
    let mut window = display.build_window().title("TGA image").scale(2).build();

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
