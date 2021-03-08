//! # Embedded graphics examples

use generate_drawing_examples::*;

/// ## Draw a single pixel
///
/// This example draws a single green pixel.
///
/// For cases where many pixels are drawn it is preferable to implement
/// a custom iterator instead of calling `Pixel::draw` for each pixel, because
/// some display drivers implement accelerated drawing of iterators.
fn draw_pixel(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{pixelcolor::Rgb888, prelude::*};

    Pixel(Point::new(32, 32), Rgb888::GREEN).draw(&mut display)?;

    Ok(display)
}

/// ## Draw a line
///
/// This example draws a red line with 8px stroke.
fn draw_line(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{Line, PrimitiveStyle},
    };

    Line::new(Point::new(16, 24), Point::new(51, 34))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 8))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw a rectangle
///
/// This example draws a rectangle with a 2px thick red stroke and cyan fill color.
fn draw_rectangle(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{PrimitiveStyleBuilder, Rectangle},
    };

    Rectangle::new(Point::new(16, 24), Size::new(32, 16))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(2)
                .stroke_color(Rgb888::RED)
                .fill_color(Rgb888::CYAN)
                .build(),
        )
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw a circle
///
/// This example draws a circle with no stroke and a solid blue fill.
fn draw_circle(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{Circle, PrimitiveStyle},
    };

    Circle::new(Point::new(16, 16), 40)
        .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw an ellipse
///
/// This example draws an ellipse with a 2px green stroke.
fn draw_ellipse(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{Ellipse, PrimitiveStyle},
    };

    Ellipse::new(Point::new(8, 16), Size::new(48, 32))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw an arc
///
/// This example draws an arc with a 2px green stroke.
fn draw_arc(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{Arc, PrimitiveStyle},
    };

    Arc::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw a sector
///
/// This example draws a sector with no stroke and a solid blue fill.
fn draw_sector(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{PrimitiveStyle, Sector},
    };

    Sector::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
        .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw a triangle
///
/// This example draws a triangle with a solid 1px magenta stroke and no fill.
fn draw_triangle(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{PrimitiveStyle, Triangle},
    };

    Triangle::new(Point::new(32, 16), Point::new(16, 48), Point::new(48, 48))
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw a polyline
///
/// This example draws a polyline with 1px cyan stroke.
fn draw_polyline(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{Polyline, PrimitiveStyle},
    };

    let points: [Point; 5] = [
        Point::new(8, 8),
        Point::new(48, 16),
        Point::new(32, 48),
        Point::new(16, 32),
        Point::new(32, 32),
    ];

    Polyline::new(&points)
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::CYAN, 1))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Draw a rectangle with rounded corners
///
/// This example draws a rectangle with rounded corners, red stroke and green fill.
fn draw_rounded_rectangle(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        pixelcolor::Rgb888,
        prelude::*,
        primitives::{PrimitiveStyleBuilder, Rectangle, RoundedRectangle},
    };

    let style = PrimitiveStyleBuilder::new()
        .stroke_color(Rgb888::RED)
        .stroke_width(3)
        .fill_color(Rgb888::GREEN)
        .build();

    RoundedRectangle::with_equal_corners(
        Rectangle::new(Point::new(8, 16), Size::new(48, 32)),
        Size::new(10, 10),
    )
    .into_styled(style)
    .draw(&mut display)?;

    Ok(display)
}

/// ## Draw some text
///
/// This example draws the text \"Hello,\\nRust!\" with the `Font6x10` font in green.
fn draw_text(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{
        mono_font::{ascii::Font6x10, MonoTextStyle},
        pixelcolor::Rgb888,
        prelude::*,
        text::Text,
    };
    Text::new("Hello,\nRust!", Point::new(2, 28))
        .into_styled(MonoTextStyle::new(Font6x10, Rgb888::GREEN))
        .draw(&mut display)?;

    Ok(display)
}

/// ## Display a TGA image
///
/// This example uses [tinytga](https://crates.io/crates/tinytga) to draw an image to the display.
fn draw_tga_image(mut display: Display) -> Result<Display, std::convert::Infallible> {
    use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*};
    use tinytga::Tga;

    // Load the TGA image
    let tga: Tga<Rgb888> = Tga::from_slice(include_bytes!("../assets/rust-pride.tga")).unwrap();

    let image = Image::new(&tga, Point::zero());

    // Display the image
    image.draw(&mut display)?;

    Ok(display)
}

fn main() {
    example!(draw_pixel);
    example!(draw_line);
    example!(draw_rectangle);
    example!(draw_circle);
    example!(draw_ellipse);
    example!(draw_arc);
    example!(draw_sector);
    example!(draw_triangle);
    example!(draw_polyline);
    example!(draw_rounded_rectangle);
    example!(draw_text);
    example!(draw_tga_image);

    generate_markdown(include_str!("main.rs"));
}
