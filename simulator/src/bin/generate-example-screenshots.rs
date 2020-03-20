//! Generate example screenshots for the embedded-graphics documentation
//!
//! Run from the workspace root with `cargo run --bin generate-example-screenshots`.
//!
//! Screenshots are output to `target/drawing-ops`.

use eg::pixelcolor::RgbColor;
use embedded_graphics as eg;
use embedded_graphics::DrawTarget;
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};
use std::collections::HashMap;

fn main() -> Result<(), core::convert::Infallible> {
    let output_base = "./target/drawing-ops";

    std::fs::create_dir_all(output_base).expect("Output directory could not be created");

    let mut display: SimulatorDisplay<_> = SimulatorDisplay::new(eg::geometry::Size::new(64, 64));

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
            use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

            Pixel(Point::new(32, 32), Rgb888::RED).draw(display)
        }),
    );

    examples.insert(
        "rectangle",
        Box::new(|display: &mut SimulatorDisplay<_>| {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Rectangle, style::PrimitiveStyleBuilder,
            };

            Rectangle::new(Point::new(16, 24), Point::new(48, 40))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_width(2)
                        .stroke_color(Rgb888::RED)
                        .fill_color(Rgb888::CYAN)
                        .build(),
                )
                .draw(display)
        }),
    );

    // Render all examples to images
    for (name, code) in examples {
        display.clear(eg::pixelcolor::Rgb888::BLACK)?;

        let path = format!("{}/{}.png", output_base, name);

        code(&mut display)?;

        display
            .to_image_buffer(&output_settings)
            .save(path)
            .unwrap();
    }

    Ok(())
}
