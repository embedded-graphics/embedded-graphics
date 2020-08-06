//! Generate example screenshots for the `doc/drawing-examples.md` file.
//!
//! To properly generate the correct files, a script that uses this binary (among other things) can
//! be run from the workspace root:
//!
//! ```bash
//! ./generate_drawing_examples.sh
//! ```
//!
//! The `generate_drawing_examples.sh` script will process the output of this binary into a Markdown
//! file.
//!
//! Screenshots are output to `doc/assets`.

use embedded_graphics::{pixelcolor, prelude::*};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay};

const IMAGE_OUTPUT_BASE: &str = "./doc/assets";

macro_rules! op {
    ($display:ident, $title:expr, $description:expr, $code:block) => {
        $display.clear(pixelcolor::Rgb888::BLACK).unwrap();

        (|| {
            $code;

            Ok::<(), core::convert::Infallible>(())
        })()
        .unwrap();

        let cleansed_title = $title.replace(" ", "_").to_lowercase();

        let output_settings = OutputSettingsBuilder::new().scale(2).build();

        let file_path = format!("{}/{}.png", IMAGE_OUTPUT_BASE, cleansed_title);
        $display
            .to_image_buffer(&output_settings)
            .save(&file_path)
            .unwrap();

        let doc_assets_path = format!("./assets/{}.png", cleansed_title);

        // Newlines in the code block aren't preserved by the stringify macro.
        // Use {} in the code block to insert newlines in the generated output.
        let doc_lines: Vec<_> = stringify!($code)
            .trim_matches(|c| c == '{' || c == '}')
            .trim()
            .lines()
            .map(|l| l.trim())
            .map(|l| if l == "{ }" { "" } else { l })
            .collect();

        println!(
            r#"//!
//! ## {}
//!
//! {}
//!
//! <img align="left" alt="{} example screenshot" src="{}" />
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! {}
//! # Ok::<(), core::convert::Infallible>(())
//! ```"#,
            $title,
            $description.lines().collect::<Vec<_>>().join("\n//! "),
            $title,
            doc_assets_path,
            doc_lines.join("\n//! ")
        );
    };
}

fn main() {
    std::fs::create_dir_all(IMAGE_OUTPUT_BASE).expect("Output directory could not be created");

    let mut display: SimulatorDisplay<pixelcolor::Rgb888> =
        SimulatorDisplay::new(Size::new(64, 64));

    // Note: Alternative header syntax here as lines beginning with `//! #` are stripped as hidden
    // code lines.
    println!("//! Embedded graphics examples\n//! ===");

    op!(
        display,
        "Draw a single pixel",
        r#"This example draws a single green pixel.

For cases where many pixels are drawn it is preferable to implement
a custom iterator instead of calling `Pixel::draw` for each pixel, because
some display drivers implement accelerated drawing of iterators."#,
        {
            use embedded_graphics::{pixelcolor::Rgb888, prelude::*};
            {}
            Pixel(Point::new(32, 32), Rgb888::GREEN).draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a line",
        "This example draws a red line with 8px stroke.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Line, style::PrimitiveStyle,
            };
            {}
            Line::new(Point::new(16, 24), Point::new(51, 34))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 8))
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
            {}
            Rectangle::new(Point::new(16, 24), Size::new(32, 16))
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
                pixelcolor::Rgb888, prelude::*, primitives::Circle, style::PrimitiveStyle,
            };
            {}
            Circle::new(Point::new(16, 16), 40)
                .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw an ellipse",
        "This example draws an ellipse with a 2px green stroke.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Ellipse, style::PrimitiveStyle,
            };
            {}
            Ellipse::new(Point::new(8, 16), Size::new(48, 32))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw an arc",
        "This example draws an arc with a 2px green stroke.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Arc, style::PrimitiveStyle,
            };
            {}
            Arc::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a sector",
        "This example draws a sector with no stroke and a solid blue fill.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Sector, style::PrimitiveStyle,
            };
            {}
            Sector::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
                .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a triangle",
        "This example draws a triangle with a solid 1px magenta stroke and no fill.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Triangle, style::PrimitiveStyle,
            };
            {}
            Triangle::new(Point::new(32, 16), Point::new(16, 48), Point::new(48, 48))
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a polyline",
        "This example draws a polyline with 1px cyan stroke.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888, prelude::*, primitives::Polyline, style::PrimitiveStyle,
            };
            {}
            let points: [Point; 5] = [
                Point::new(8, 8),
                Point::new(48, 16),
                Point::new(32, 48),
                Point::new(16, 32),
                Point::new(32, 32),
            ];
            {}
            Polyline::new(&points)
                .into_styled(PrimitiveStyle::with_stroke(Rgb888::CYAN, 1))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Draw a rectangle with rounded corners",
        "This example draws a rectangle with rounded corners, red stroke and green fill.",
        {
            use embedded_graphics::{
                pixelcolor::Rgb888,
                prelude::*,
                primitives::{Rectangle, RoundedRectangle},
                style::PrimitiveStyleBuilder,
            };
            {}
            let style = PrimitiveStyleBuilder::new()
                .stroke_color(Rgb888::RED)
                .stroke_width(3)
                .fill_color(Rgb888::GREEN)
                .build();
            {}
            RoundedRectangle::with_equal_corners(
                Rectangle::new(Point::new(8, 16), Size::new(48, 32)),
                Size::new(10, 10),
            )
            .into_styled(style)
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
                style::TextStyle,
            };
            {}
            Text::new("Hello,\nRust!", Point::new(2, 28))
                .into_styled(TextStyle::new(Font6x8, Rgb888::GREEN))
                .draw(&mut display)?;
        }
    );

    op!(
        display,
        "Display a TGA image",
        "This example uses [tinytga](https://crates.io/crates/tinytga) to draw an image to the display.

The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga`
object with embedded-graphics.",
        {
            use embedded_graphics::{
                image::Image,
                pixelcolor::Rgb888,
                prelude::*,
            };
            use tinytga::Tga;
            {}
            // Load the TGA image
            let tga: Tga<Rgb888> = Tga::from_slice(
                include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/../simulator/examples/assets/rust-pride.tga"))
            ).unwrap();
            {}
            let image = Image::new(&tga, Point::zero());
            {}
            // Display the image
            image.draw(&mut display)?;
        }
    );

    // Add dummy mod to allow running rustfmt
    println!("pub mod dummy {{}}");
}
