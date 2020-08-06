//! # Example: Sub images
//!
//! Loads a TGA file using the [tinytga](https://crates.io/crates/tinybmp) crate and splits it
//! into multiple sub images.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.

use embedded_graphics::{
    fonts::{Font6x8, Text},
    image::Image,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::Rectangle,
    style::TextStyle,
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use tinytga::Tga;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(250, 400));

    // Load TGA file with the tiles.
    let tiles: Tga<Rgb888> = Tga::from_slice(include_bytes!("../../assets/tiles.tga")).unwrap();

    // Create sub images for the individual tiles.
    // Note that the tiles don't have to be the same size.
    let tile_a = tiles.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(64, 64)));
    let tile_b = tiles.sub_image(&Rectangle::new(Point::new(64, 0), Size::new(64, 64)));
    let tile_c = tiles.sub_image(&Rectangle::new(Point::new(0, 64), Size::new(128, 64)));

    // Draw the entire image.
    Image::new(&tiles, Point::new(100, 10)).draw(&mut display)?;

    // Draw each tile.
    Image::new(&tile_a, Point::new(100, 170)).draw(&mut display)?;
    Image::new(&tile_b, Point::new(100, 240)).draw(&mut display)?;
    Image::new(&tile_c, Point::new(100, 310)).draw(&mut display)?;

    // Draw labels.
    let text_style = TextStyle::new(Font6x8, Rgb888::WHITE);
    Text::new("TGA image", Point::new(10, 70))
        .into_styled(text_style)
        .draw(&mut display)?;
    Text::new("Tile A", Point::new(10, 200))
        .into_styled(text_style)
        .draw(&mut display)?;
    Text::new("Tile B", Point::new(10, 270))
        .into_styled(text_style)
        .draw(&mut display)?;
    Text::new("Tile C", Point::new(10, 340))
        .into_styled(text_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Sub images", &output_settings).show_static(&display);

    Ok(())
}
