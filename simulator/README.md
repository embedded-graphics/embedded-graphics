# Embedded graphics simulator

[![Build Status](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master.svg?style=shield)](https://circleci.com/gh/jamwaffles/embedded-graphics/tree/master)
[![Crates.io](https://img.shields.io/crates/v/embedded-graphics-simulator.svg)](https://crates.io/crates/embedded-graphics-simulator)
[![Docs.rs](https://docs.rs/embedded-graphics-simulator/badge.svg)](https://docs.rs/embedded-graphics-simulator)

## [Documentation](https://docs.rs/embedded-graphics-simulator)

![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)

The simulator can be used to test and debug
[embedded-graphics](https://crates.io/crates/embedded-graphics) code, or produce snazzy examples
for people to try drivers out without needing physical hardware to run on.

## Setup

The simulator uses SDL and its development libraries which must be installed to build and run
it.

### Linux (`apt`)

```bash
sudo apt install libsdl2-dev
```

### macOS (`brew`)

```bash
brew install sdl2
```

### Windows

The Windows install process is a bit more involved, but it _does_ work. See [the SDL2
wiki](https://wiki.libsdl.org/Installation#WinRT.2FWindows_8.2FWinPhone) for instructions.

## Examples

### Simulate a 128x64 SSD1306 OLED

```rust
use embedded_graphics::fonts::{Font6x8, Text};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::primitives::{Circle, Line};
use embedded_graphics::style::{PrimitiveStyle, TextStyle};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, SimulatorEvent, Window,
};
use std::thread;
use std::time::Duration;

let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(128, 64));
let output_settings = OutputSettingsBuilder::new()
    .theme(BinaryColorTheme::OledBlue)
    .build();
let mut window = Window::new("Example", &output_settings);

Text::new("Hello World!", Point::zero())
    .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
    .draw(&mut display);

Circle::new(Point::new(65, 1), 63)
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .draw(&mut display);

Line::new(Point::new(32, 32), Point::new(1, 32))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .translate(Point::new(64, 0))
    .draw(&mut display);
Line::new(Point::new(32, 32), Point::new(40, 40))
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .translate(Point::new(64, 0))
    .draw(&mut display);

'running: loop {
    window.update(&display);

    for event in window.events() {
        match event {
            SimulatorEvent::MouseButtonUp { point, .. } => {
                println!("Click event at ({}, {})", point.x, point.y);
            }
            SimulatorEvent::Quit => break 'running,
            _ => {}
        }

        thread::sleep(Duration::from_millis(200));
    }
}
```

## Creating screenshots

Screenshots of programs, that use `Window` to display a simulated display, can be created by
setting the `EG_SIMULATOR_DUMP` environment variable:

```bash
EG_SIMULATOR_DUMP=screenshot.png cargo run
```

By setting the variable the display passed to the first `Window::update` call gets exported as a
PNG file to the specified path. After the file is exported the process is terminated.

## Exporting images

If a program doesn't require to display a window and only needs to export one or more images, a
`SimulatorDisplay` can also be converted to an `image` crate `ImageBuffer` by using the
`to_image_buffer` method. The resulting buffer can then be used to save the display content to
any format supported by `image`.

## Usage without SDL2

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

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
