//! Generate example screenshots for the embedded-graphics documentation
//!
//! Run from the workspace root with:
//!
//! ```bash
//! cargo run --bin generate-example-screenshots | rustfmt +nightly --config-path rustfmt.nightly.toml
//! ```
//!
//! Screenshots are output to `target/drawing-ops`.

use embedded_graphics::{pixelcolor, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

const OUTPUT_BASE: &str = "./target/drawing-ops";

macro_rules! op {
    ($display:ident, $title:expr, $description:expr, $code:block) => {
        $display.clear(pixelcolor::Rgb888::BLACK).unwrap();

        (|| {
            $code;

            Ok::<(), core::convert::Infallible>(())
        })().unwrap();

        let output_settings = OutputSettingsBuilder::new().scale(2).build();

        let path = format!("{}/{}.png", OUTPUT_BASE, $title);
        $display
            .to_image_buffer(&output_settings)
            .save(&path)
            .unwrap();

        let screenshot = base64::encode(std::fs::read(&path).expect("Couldn't open screenshot"));

        let docs = stringify!($code)
                .trim_matches(|c| c == '{' || c == '}')
                .trim();

        // Note: empty lines must remain between HTML elements and inner Markdown for the Markdown
        // to be parsed correctly.
        println!(
            r#"//! ## {}
            //!
            //! {}
            //!
            //! <div style="display: flex">
            //! <img style="width: 128px; height: 128px; display: inline-block; margin-right: 8px;" alt="{} example screenshot" src="data:image/png;base64,{}" />
            //! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! {}
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!"#,
            $title,
            $description,
            $title,
            screenshot,
            docs.lines().collect::<Vec<_>>().join("\n//! ")
        );
    };
}

fn main() {
    let output_base = "./target/drawing-ops";
    std::fs::create_dir_all(output_base).expect("Output directory could not be created");

    let mut display: SimulatorDisplay<pixelcolor::Rgb888> =
        SimulatorDisplay::new(Size::new(64, 64));

    op!(
        display,
        "Draw a single pixel",
        "This example draws a single green pixel.",
        {
            use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

            Pixel(Point::new(32, 32), Rgb888::GREEN).draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a line",
        "This example draws a red line with 1px stroke.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Line, style::PrimitiveStyleBuilder,
            };

            Line::new(Point::new(16, 24), Point::new(48, 40))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_width(1)
                        .stroke_color(Rgb888::RED)
                        .build(),
                )
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a rectangle",
        "This example draws a rectangle with a 2px thick red stroke and cyan fill color.",
        {
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
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a circle",
        "This example draws a circle with no stroke and a solid blue fill.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Circle, style::PrimitiveStyleBuilder,
            };

            Circle::new(Point::new(22, 22), 20)
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .fill_color(Rgb888::BLUE)
                        .build(),
                )
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a triangle",
        "This example draws a triangle with a solid 1px magenta stroke and no fill.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Triangle, style::PrimitiveStyleBuilder,
            };

            Triangle::new(Point::new(32, 16), Point::new(16, 48), Point::new(48, 48))
                .into_styled(
                    PrimitiveStyleBuilder::new()
                        .stroke_width(1)
                        .stroke_color(Rgb888::MAGENTA)
                        .build(),
                )
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw some text",
        "This example draws the text \"Hello,\\nRust!\" with the [`Font6x8`] font in green.",
        {
            use embedded_graphics::{
                fonts::{Font6x8, Text},
                pixelcolor::Rgb888,
                prelude::*,
                style::TextStyleBuilder,
            };

            // Create a new text style
            let style = TextStyleBuilder::new(Font6x8)
                .text_color(Rgb888::GREEN)
                .build();

            Text::new("Hello,\nRust!", Point::new(2, 28))
                .into_styled(style)
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Display a TGA image",
        "This example uses [tinytga](https://crates.io/crates/tinytga) to draw an image to the display.",
        {
            use embedded_graphics::{
                image::Image,
                pixelcolor::Rgb888,
                prelude::*,
            };
            use tinytga::Tga;

            // Load the TGA image
            let tga = Tga::from_slice(
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../simulator/examples/assets/rust-pride.tga"))
            ).unwrap();

            let image: Image<Tga, Rgb888> = Image::new(&tga, Point::zero());

            // Display the image
            image.draw(&mut display)?;
        }
    );

    // Add dummy mod to allow running rustfmt
    println!("pub mod dummy {{}}");
}
