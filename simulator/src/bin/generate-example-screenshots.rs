//! Generate example screenshots for the embedded-graphics documentation
//!
//! Run from the workspace root with `cargo run --bin generate-example-screenshots`.
//!
//! Screenshots are output to `target/drawing-ops`.

use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use std::collections::HashMap;

fn main() -> Result<(), core::convert::Infallible> {
    let output_base = "./target/drawing-ops";

    std::fs::create_dir_all(output_base).expect("Output directory could not be created");

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(50, 50));

    let output_settings = OutputSettingsBuilder::new()
        .scale(2)
        .pixel_spacing(1)
        .build();

    let mut examples: HashMap<
        &'static str,
        Box<dyn Fn(&mut SimulatorDisplay<_>) -> Result<(), core::convert::Infallible>>,
    > = HashMap::new();

    // Add examples
    examples.insert(
        "pixel",
        Box::new(|display: &mut SimulatorDisplay<_>| {
            Pixel(Point::new(25, 25), Rgb888::RED).draw(display)
        }),
    );

    // Render all examples to images
    for (name, code) in examples {
        display.clear(Rgb888::BLACK)?;

        let path = format!("{}/{}.png", output_base, name);

        code(&mut display)?;

        display
            .to_image_buffer(&output_settings)
            .save(path)
            .unwrap();
    }

    Ok(())
}
