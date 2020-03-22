//! Generate example screenshots for the embedded-graphics documentation
//!
//! Run from the workspace root with:
//!
//! ```bash
//! cargo run --bin generate-example-screenshots | rustfmt +nightly --config-path rustfmt.nightly.toml
//! ```
//!
//! Screenshots are output to `target/drawing-ops`.

use embedded_graphics::{
    pixelcolor::Rgb888, prelude::*, primitives::Rectangle, style::PrimitiveStyleBuilder,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

const OUTPUT_BASE: &str = "./target/drawing-ops";

macro_rules! op {
    ($display:ident, $name:expr, $code:block) => {
        $display.clear(Rgb888::BLACK).unwrap();

        (|| $code)().unwrap();

        let output_settings = OutputSettingsBuilder::new()
            .scale(2)
            .pixel_spacing(1)
            .build();

        let path = format!("{}/{}.png", OUTPUT_BASE, $name);
        $display
            .to_image_buffer(&output_settings)
            .save(path)
            .unwrap();

        let docs = format!(
            "```rust\n{}\n```",
            stringify!($code).trim_matches(|c| c == '{' || c == '}')
        );

        println!("/// # {} example", $name);
        for line in docs.lines() {
            println!("/// {}", line);
        }
    };
}

fn main() {
    let output_base = "./target/drawing-ops";
    std::fs::create_dir_all(output_base).expect("Output directory could not be created");

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(64, 64));

    op!(display, "pixel", {
        Pixel(Point::new(32, 32), Rgb888::RED).draw(&mut display)
    });

    op!(display, "rectangle", {
        Rectangle::new(Point::new(16, 24), Point::new(48, 40))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(2)
                    .stroke_color(Rgb888::RED)
                    .fill_color(Rgb888::CYAN)
                    .build(),
            )
            .draw(&mut display)
    });

    // Add dummy mod to allow running rustfmt
    println!("mod dummy {}", "{}");
}
