# Embedded graphics simulator

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/embedded-graphics-simulator.svg)](https://crates.io/crates/embedded-graphics-simulator)
[![Docs.rs](https://docs.rs/embedded-graphics-simulator/badge.svg)](https://docs.rs/embedded-graphics-simulator)

![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)

The simulator can be used to test and debug [embedded-graphics](https://crates.io/crates/embedded-graphics) code, or produce snazzy examples for people to try drivers out without needing physical hardware to run on.

# Examples

## Simulate a 128x64 SSD1306 OLED

```rust,no_run
use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment, TextStyle},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};

fn main() -> Result<(), std::convert::Infallible> {
    // Create a new simulator display with 128x64 pixels.
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));

    // Create styles used by the drawing operations.
    let thin_stroke = PrimitiveStyle::with_stroke(BinaryColor::On, 1);
    let thick_stroke = PrimitiveStyleBuilder::new()
        .stroke_color(BinaryColor::On)
        .stroke_width(3)
        .stroke_alignment(StrokeAlignment::Inside)
        .build();
    let fill = PrimitiveStyle::with_fill(BinaryColor::On);
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);

    let yoffset = 14;

    // Draw a 3px wide outline around the display.
    Rectangle::new(Point::zero(), display.size())
        .into_styled(thick_stroke)
        .draw(&mut display)?;

    // Draw a triangle.
    Triangle::new(
        Point::new(18, 17 + yoffset),
        Point::new(18 + 16, 17 + yoffset),
        Point::new(18 + 8, yoffset),
    )
    .into_styled(thin_stroke)
    .draw(&mut display)?;

    // Draw a filled square
    Rectangle::new(Point::new(55, yoffset), Size::new(18, 18))
        .into_styled(fill)
        .draw(&mut display)?;

    // Draw a circle with a 3px wide stroke.
    Circle::new(Point::new(92, yoffset), 18)
        .into_styled(thick_stroke)
        .draw(&mut display)?;

    // Draw centered text.
    let text = "embedded-graphics";
    let width = text.len() as i32 * 6;
    Text::new(text, Point::new(64 - width / 2, 43))
        .into_styled(text_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        .theme(BinaryColorTheme::OledBlue)
        .build();
    Window::new("Hello World", &output_settings).show_static(&display);

    Ok(())
}
```

# Creating screenshots

Screenshots of programs, that use `Window` to display a simulated display, can be created by
setting the `EG_SIMULATOR_DUMP` environment variable:

```bash
EG_SIMULATOR_DUMP=screenshot.png cargo run
```

By setting the variable the display passed to the first `Window::update` call gets exported as a
PNG file to the specified path. After the file is exported the process is terminated.

# Exporting images

If a program doesn't require to display a window and only needs to export one or more images, a
`SimulatorDisplay` can also be converted to an `image` crate `ImageBuffer` by using the
`to_image_buffer` method. The resulting buffer can then be used to save the display content to
any format supported by `image`.

# Usage without SDL2

When the simulator is used in headless/CI environments that don't require showing a window, SDL2
support can be disabled. This removes the requirement of SDL2 being installed on the target machine,
but still allows the simulator to be used to generate images.

The `with-sdl` feature is enabled by default and can be disabled by adding `default-features = false` to the dependency:

```toml
[dependencies.embedded-graphics-simulator]
version = "0.2.0"
default-features = false
```

See the [Choosing
Features](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#choosing-features)
Cargo manifest documentation for more details.
