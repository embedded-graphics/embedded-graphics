//! This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
//! following built in items:
//!
//! * [Raw data images](./image/struct.ImageRaw.html)
//! * [Primitives](./primitives/index.html)
//!     * [Lines](./primitives/line/struct.Line.html)
//!     * [Rectangles (and squares)](./primitives/rectangle/struct.Rectangle.html)
//!     * [Circles](./primitives/circle/struct.Circle.html)
//!     * [Ellipses](./primitives/ellipse/struct.Ellipse.html)
//!     * [Arcs](./primitives/arc/struct.Arc.html)
//!     * [Sectors](./primitives/sector/struct.Sector.html)
//!     * [Triangles](./primitives/triangle/struct.Triangle.html)
//!     * [Polylines](./primitives/polyline/struct.Polyline.html)
//!     * [Rounded rectangles](./primitives/rounded_rectangle/struct.RoundedRectangle.html)
//! * [Text with multiple fonts](./fonts/index.html#types)
//!
//! Additional functionality provided by external crates:
//!
//! * [BMP images - `tinybmp`](https://crates.io/crates/tinybmp)
//! * [TGA images - `tinytga`](https://crates.io/crates/tinytga)
//! * [ProFont monospace font - `profont`](https://crates.io/crates/profont)
//! * [Picofont Pico8 font - `embedded-picofont`](https://crates.io/crates/embedded_picofont)
//! * [IBM437 font - `ibm437`](https://crates.io/crates/ibm437)
//!
//! If you know of a crate that is not in this list, please [open an
//! issue](https://github.com/jamwaffles/embedded-graphics/issues/new).
//!
//! Note that some of these crates may not support the latest version of embedded-graphics.
//!
//! You can also add your own objects by implementing [`Drawable`] on them. Additionally, all
//! iterators over pixels (`Iterator<Item = Pixel<C>>`) have a default [`Drawable`] implementation
//! already created.
//!
//! A core goal of embedded-graphics is to draw graphics without using any buffers; the crate is
//! `no_std` compatible and works without a dynamic memory allocator, and without pre-allocating
//! large chunks of memory. To achieve this, it takes an `Iterator` based approach, where pixel
//! values and positions are calculated on the fly, with the minimum of saved state. This allows the
//! consuming application to use far less RAM at little to no performance penalty.
//!
//! # Supported displays
//!
//! These are just some of the displays the community has added embedded_graphics support to. This
//! list is taken from the [dependent crates
//! list](https://crates.io/crates/embedded-graphics/reverse_dependencies) on crates.io so might be
//! missing some unpublished entries. Please [open an
//! issue](https://github.com/jamwaffles/embedded-graphics/issues/new) if there's a display driver
//! that should be added to this list.
//!
//! Note that some drivers may not support the latest version of embedded-graphics.
//!
//! * [embedded-graphics-web-simulator](https://crates.io/crates/embedded-graphics-web-simulator): Simulated display in your browser via Webassembly
//! * [epd-waveshare](https://crates.io/crates/epd-waveshare) Driver for various ePaper displays (EPD) from Waveshare
//! * [hub75](https://crates.io/crates/hub75): A rust driver for hub75 rgb matrix displays
//! * [ili9341](https://crates.io/crates/ili9341): A platform agnostic driver to interface with the ILI9341 (and ILI9340C) TFT LCD display
//! * [ls010b7dh01](https://crates.io/crates/ls010b7dh01): A platform agnostic driver for the LS010B7DH01 memory LCD display
//! * [sh1106](https://crates.io/crates/sh1106): I2C driver for the SH1106 OLED display
//! * [ssd1306](https://crates.io/crates/ssd1306): I2C and SPI (4 wire) driver for the SSD1306 OLED display
//! * [ssd1322](https://crates.io/crates/ssd1322): Pure Rust driver for the SSD1322 OLED display chip
//! * [ssd1331](https://crates.io/crates/ssd1331): SPI (4 wire) driver for the SSD1331 OLED display
//! * [ssd1351](https://crates.io/crates/ssd1351): SSD1351 driver
//! * [ssd1675](https://crates.io/crates/ssd1675): Rust driver for the Solomon Systech SSD1675 e-Paper display (EPD) controller
//! * [st7735-lcd](https://crates.io/crates/st7735-lcd): Rust library for displays using the ST7735 driver
//! * [st7789](https://crates.io/crates/st7789): A Rust driver library for ST7789 displays
//! * [st7920](https://crates.io/crates/st7920): ST7920 LCD driver in Rust
//!
//! # Simulator
//!
//! Embedded graphics comes with a [simulator]!
//!
//! ![It can display all sorts of embedded-graphics test code.](https://raw.githubusercontent.com/jamwaffles/embedded-graphics/master/assets/simulator-demo.png)
//!
//! Take a look at the [simulator examples] to see what
//! embedded_graphics can do, and how it might look on a display. You can run the examples like
//! this:
//!
//! ```bash
//! git clone https://github.com/jamwaffles/embedded-graphics.git
//! cd embedded-graphics
//!
//! cargo run -p embedded-graphics-simulator --example hello
//! ```
//!
//! [simulator]: https://github.com/jamwaffles/embedded-graphics/tree/c4f74c12dae9f0a0193fa48192f905a002bf8c9d/simulator
//! [simulator examples]: https://github.com/jamwaffles/embedded-graphics/tree/c4f74c12dae9f0a0193fa48192f905a002bf8c9d/simulator/examples
//!
//! # Crate features
//!
//! Add these to your `Cargo.toml` to turn on extra bits of functionality.
//!
//! * `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
//! support to enable conversions from `nalgebra::Vector2` to [`Point`] and [`Size`].
//!
//! * `fixed_point` - use fixed point arithmetic instead of floating point for all trigonometric
//! calculation.
//!
//! # Implementing `embedded_graphics` in a driver
//!
//! To add support for embedded_graphics to a display driver, [`DrawTarget`] should be implemented.
//! This allows all embedded_graphics objects to be rendered by the display. See the [`DrawTarget`]
//! documentation for implementation details.
//!
//! # Examples
//!
//! ## Draw a circle and some text
//!
//! This example uses the [`Circle`] primitive and the [`Font6x8`] font to draw a filled circle and  some text over it on the screen.
//!
//! ```rust
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//! #   mock_display::MockDisplay,
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     primitives::Circle,
//!     style::{PrimitiveStyle, TextStyle},
//! };
//!
//! # let mut display = MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! # display.set_allow_out_of_bounds_drawing(true);
//!
//! let c = Circle::new(Point::new(12, 12), 17).into_styled(PrimitiveStyle::with_fill(Rgb565::RED));
//! let t = Text::new("Hello Rust!", Point::new(20, 16))
//!     .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN));
//!
//! // The `display` variable contains a `DrawTarget` implementation provided by the display driver
//! // crate. See the driver crate documentation for more information about how it is constructed.
//! c.draw(&mut display)?;
//! t.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//! ## Chaining
//!
//! Items can be chained to build more complex graphics objects.
//!
//! ```rust
//! use embedded_graphics::{
//!     fonts::{Font6x8, Text},
//!     mock_display::MockDisplay,
//!     pixelcolor::Rgb565,
//!     prelude::*,
//!     primitives::{Circle, Rectangle},
//!     style::{PrimitiveStyle, TextStyle},
//! };
//!
//! fn build_thing(text: &'static str) -> impl Iterator<Item = Pixel<Rgb565>> {
//!     Rectangle::new(Point::new(0, 0), Size::new(40, 40))
//!         .into_styled(PrimitiveStyle::with_stroke(Rgb565::CYAN, 1))
//!         .into_iter()
//!         .chain(
//!             &Circle::new(Point::new(12, 12), 17)
//!                 .into_styled(PrimitiveStyle::with_fill(Rgb565::RED)),
//!         )
//!         .chain(
//!             &Text::new(text, Point::new(20, 16))
//!                 .into_styled(TextStyle::new(Font6x8, Rgb565::GREEN)),
//!         )
//! }
//!
//! # let mut display = MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! # display.set_allow_out_of_bounds_drawing(true);
//! build_thing("Hello Rust!").draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! ## Draw a single pixel
//!
//! This example draws a single green pixel.
//!
//! For cases where many pixels are drawn it is preferable to implement
//! a custom iterator instead of calling `Pixel::draw` for each pixel, because
//! some display drivers implement accelerated drawing of iterators.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a single pixel example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAABNElEQVR4nO3RsQkAMQwEwf/+i7YduQEHi2AGlOvY/yMlQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEJsfYJ0bvGPs45cAvBAgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWIbyaYCgYRUdd0AAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//! };
//!
//! Pixel(Point::new(32, 32), Rgb888::GREEN).draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a line
//!
//! This example draws a red line with 8px stroke.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a line example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAABxklEQVR4nO3RQW7bQBAAweT/j04C+OhQkM2d7ZVcBfC6nJn+/YuUADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQGxTgD//vm0/eymbbiLAlU03+QhwZdMQR9q0uwBXNu3+OMBnm8Y6wKZNBbgyvulXT//Y+LjbjW8kwGPjG60N8Nn4AsPG5xfgsfH5pwNcGV9skfE5BXhscM7q9FcGV71hcCoBnjE41WkBrgye4AmDfxfgGVv/LslnO/8lwH/s/Nelnxxm4s0vE+A4PyfJ/RdGCHCo08LcP9/9F7YS4Dhtkvvnu/9CTIBDTYdZdbhV7xxHgOOsTbLqcKveeQECHOp7YVYdbtU7L0yA4zxOsvZka197EwIc6iPM2pOtfe3NCfCGBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMg9hfefCSBHuP7aAAAAABJRU5ErkJggg==" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::Line,
//!     style::PrimitiveStyle,
//! };
//!
//! Line::new(Point::new(16, 24), Point::new(51, 34))
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 8))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a rectangle
//!
//! This example draws a rectangle with a 2px thick red stroke and cyan fill color.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a rectangle example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAABkUlEQVR4nO3RMWoDURAEUev+h5adycomcFPIvAcL+5MOph5fpASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyA2DDA8+f7L3Zn2i0LcLJbFuBktyzAyW75LcDj+fv1GZ6P13Fef39ttyzAyW5ZgJPdsgAnu2UBTnbLApzslgU42S0LcLJbFuBktyzAyW5ZgJPdsgAnu2UBTnbLApzslgU42S0LcLJbFuBktyzAyW5ZgJPdsgAnu2UBTnbLApzslgU42S0LcLJbFuBktyzAyW5ZgJPdsgAnu+W3AJ9ud6bdsgAnu2UBTnbLApzsljkRICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWICxASICRATICZATICYADEBYgLEBIgJEBMgJkBMgJgAMQFiAsQEiAkQEyAmQEyAmAAxAWLfWYd4geVYJIUAAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::Rectangle,
//!     style::PrimitiveStyleBuilder,
//! };
//!
//! Rectangle::new(Point::new(16, 24), Size::new(32, 16))
//!     .into_styled(
//!         PrimitiveStyleBuilder::new()
//!             .stroke_width(2)
//!             .stroke_color(Rgb888::RED)
//!             .fill_color(Rgb888::CYAN)
//!             .build(),
//!     )
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a circle
//!
//! This example draws a circle with no stroke and a solid blue fill.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a circle example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAACV0lEQVR4nO3RUU7FMAwFUdj/ooF+UVUhkidpblrmSPzZKc/z+aEoA4QZIMwAYQYIM0CYAcIMEGaAMAOEGSDMAGEGCDNAmAHCDBBmgDADhBkgzABhBggzQJgBwgwQZoAwA4QZIMwAYQYIM0CYAcIMEPaCAF8/fxV7/eS9/hvEAItUD12VOUXmq4gBAu4++l/WnWXdlxADxKROf7biOCu+gRggYIej/+WuQ931LmKAmJ1Pfzb/XPNfRAwQ85TTn8082sy3EAPEPPH0Z3NON+cVxACHOa8gBjjMeQUxwGHOK0VPP/3Z6AFH9xED/BrdL3rT6c/4GfkmYoArvokY4IpvIga44ptFbz39GTkm2UEM0EZ2EAO0kR3EAG1kBzFAG9lBDNBGdhADtJEdxABtZKfoP5z+rHbS2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2jRigJ7aNGKAnto0YoCe2vSA/5CBHJPsIAZoIzuIAdrIDmKANrKDGKCN7CAGaCM7iAHayA5igDayM+CtGfgZ+SZigCu+iRjgim8iBrjimwPelGH0gKP7iAF+je4PeHqGOaeb8wpigMOcVxADHOa8ghjgMOeVAU/MMPNoM99CDLCFp2SYf675LyIG2MLOGe461F3vIgbYyA4xVhxnxTcQA2whlWHdWdZ9CTHARu6OkTlF5quIATZVDbPXT97rv0EMoAEGCDNAmAHCDBBmgDADhBkgzABhBggzQJgBwgwQZoAwA4QZIOwbbHdQgYxk5xMAAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::Circle,
//!     style::PrimitiveStyle,
//! };
//!
//! Circle::new(Point::new(16, 16), 40)
//!     .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw an ellipse
//!
//! This example draws an ellipse with a 2px green stroke.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw an ellipse example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAADDklEQVR4nO3R26rlMAwD0Jn//+gZQaGU3X0OiWVbaaoFfXPji/7+MSkHIOYAxByAmAMQcwBiDkDMAYg5ADEHIOYAxByAmAMQcwBiDkDMAYg5ADEHIOYAxByAmAMQcwBiDkDMAYg5ADEHIOYAxByAmAMQcwBiawfwD1+GhbdceDRwAE2yDj1rge0XGAEcQKvxc2dN199xWHtD6D9Hf8dhXQ1/P0HXFJ8WmKqlCSyw6hcLTFXZ5Kf1Knuy2mcuexjal0nQPnPNw/c1avrUatmi4EloGb1cyxZ5T97HPeR10CjeK+kZKB5UpnivjGfuI2a8uqKCTekHoGCsRRVsyj1QMNADpG5N/AqpozxG6tbEr3AdhXvpeZJ2J36FpCEeKWn36K9J7R+PvkPoJ6Abb4K+w/xP15aH+Tf2QV9jshzolluhrzFZDteW83/vibjJZDkQzbZF3GSyHIhm2yJuMlkORLNtETeZKSfavELoPsOFEGrwIqH7DBdCqMGLhO4zXAihBi8Sus9wIYQavEjoPsOFEGrwIqH7DBdCqMGLhO4zXAihBi8Sus9wIYQavEjoPsOFEGrwIqH7DBdCqMGLhO4zXAihBi8Sus9w4SnUZnPETSbLgWi2LeImk+VANNsWcZPJciCabYu4yWQ5XJsd5t/YB32NyXKgW26FvsZk+enaOPrGDug7hH4CuvEm6DuEfjrR7R8saXfiV0ga4pGSdid+hesQB+69Z0jdmvgVUkd5jNStiV9PqQMtrWBT+gEoGGtRBZvSD5zuwx3yOmgU75X0DBQPKlO8V9IzH+5D1/Sp1bJFwZPQMnq5li0KnjzdFzhU9mS1z1z2MLQvk6B95rKHP/y02KFrik8LTNXSBBZY9YsFpmpp8uH3ta+ypuvvOKy9IfSfo7/jsPaGX40fKNcC2y8wAjiARWUFs/CWC48GDsCqOQAxByDmAMQcgJgDEHMAYg5AzAGIOQAxByDmAMQcgJgDEHMAYg5AzAGIOQAxByDmAMQcgJgDEHMAYg5AzAGIOQAxByDmAMQcgJgDEPsPGleAgfKjdjYAAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::Ellipse,
//!     style::PrimitiveStyle,
//! };
//!
//! Ellipse::new(Point::new(8, 16), Size::new(48, 32))
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw an arc
//!
//! This example draws an arc with a 2px green stroke.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw an arc example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAABTUlEQVR42u3cQQ6DMAxFQRvl/lcOOzaIDYqEHWYu0Ba9/gapbUbEDH7rcAkEgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAqC/se0rW/1d57QAbCij++8CZoEraAGwAFXf8dns8SwAFuDNOzKLPI9mi2ABLEDTBciiz6vZElgAC+D/AbY+q1gALIAlsABYAEtgAbAAlsACYAEsgQVAAF9vbwoAAXA7G0wBIABnAQEgAGcBASAAZwEBIAAEgADcDQgAAbgbEAACQAAIAAEgAASAABAAAkAACAABIAAEwKPhEhQzLQAWgEtaAASAAHAGcPq3AFgAp38LgAAQAIs+afxXsAVAAAgAASAABIAAEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAkAACIDvnG3zIPeb6boNAAAAAElFTkSuQmCC" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! use embedded_graphics::{
//!     pixelcolor::Rgb888, prelude::*, primitives::Arc, style::PrimitiveStyle,
//! };
//!
//! Arc::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::GREEN, 2))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a sector
//!
//! This example draws a sector with no stroke and a solid blue fill.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a sector example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAABbElEQVR42u3dgYkCQRBE0W4x/5TbCISFXcHpei8D9V85I+h1VU0R6+UpEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAATvbe/xDvfu2hLQB7dR3/zaD5g6fQAmABUv7idy2CBXAN5PmFGgHgDBB+FjjjbGABnAFIPhsIwBnAJ4HJZwILYAG2/EqYJbAACAABkHkGcBawAAjg7OUaASCAB95L2ytrARCAs4AAiA3AWcACIABLYAEQAALg6xtk2P8N9H0CC4AAEACZAfhcwAIgAARARfxO4NX79VgALECwzEWwABaA5EWwAK6BXF+EFgD7sh5Pwx3zw5fGAuAW4LZgAbAAFsECIIBTF6EFgM8BuHQ2aAuAW4DbggXAGQALgAAQAAJAAAgAASAABIAAEAACQAAIAAEgAASAABAAAkAACAABIAAEgAAQAAJAAAgAASAABCAABIAAEAACQAAIAAEgAATAfh9WESP9yc4uqgAAAABJRU5ErkJggg==" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! use embedded_graphics::{
//!     pixelcolor::Rgb888, prelude::*, primitives::Sector, style::PrimitiveStyle,
//! };
//!
//! Sector::new(Point::new(12, 12), 40, -30.0.deg(), 150.0.deg())
//!     .into_styled(PrimitiveStyle::with_fill(Rgb888::BLUE))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a triangle
//!
//! This example draws a triangle with a solid 1px magenta stroke and no fill.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a triangle example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAC0klEQVR4nO3R0YpbMQyE4fb9H7p1CQwHygn22ppfhvn2ZgmJNdL8/hWoFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFAC7voA/4+/fGrcucmtuSQGwFABLAZjn6Z//3+W+xPI8+vP/u9yXWJ5Hf/5/l/sSy/Poz//vcl/i4e3cb593dlNWeTv02+ed3ZRV3g799nlnN2WVt0O/fd7ZTVmHmRPPfKePO1LKzHFnvtPHHSll5rgz3+njjpQyc9yZ7/RxR8ph9ayr36d0zyerB139PqV7Plk96Or3Kd3zyepBV79P6Z5v2Dnlzm89+iaTnSPu/NajbzLZOeLObz36JpOdI+781qNvsuHU+U69U6FjJjl1uFPvVOiYSU4d7tQ7FTpmklOHO/VOhY6ZhoqTVby5r1caqThWxZv7eqWRimNVvLmvVxqpOFbFm/t6pRmqz1T9/qouOaT6QNXvr+qSQ6oPVP3+qi45pPpA1e+v6pJjcJ7GOes7PoE4j+Kc9R2fQJxHcc76jk8gzqM4Z33HJxioc1Bzn8jZQh2CmvtEzhbqENTcJ3K2UIeg5j6Rs4cOJ2AzMFOFXf6DzcBMFXb5DzYDM1XY5T/YDMzUgV37f1Qe9zyhFn5D5XHPE2rhN1Qe9zyhFn5D5XHPG6hVZ/iz+SaJf8l5/my+SeJfcp4/m2+S+Jec58/mmzT41/sZZ07HDHEutsOZ0zFDnIvtcOZ0zBDnYjucOR0zBudKp3gy174unmXO8mSufV08y5zlyVz7uniWOcuTufb14bPG7epqqHpXUsB3Ve/GpBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAEsBsBQASwGwFABLAbAUAPsLOXL8gVjK1mAAAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::Triangle,
//!     style::PrimitiveStyle,
//! };
//!
//! Triangle::new(Point::new(32, 16), Point::new(16, 48), Point::new(48, 48))
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::MAGENTA, 1))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a polyline
//!
//! This example draws a polyline with 1px cyan stroke.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a polyline example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAACb0lEQVR4nO3RgU7jQAwA0eP/P5qjrE7NEYiyydpj03kSCNF01/G8/RHKADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAmw/w/v7x6z9v84fon/ndGWCpFbvbJ9kyz6EV2zHADfHbMc+h+Pc3wCH6/V8+D/2GBvj4qesF8tR+BwOU9ivy9JjyewYorUmeKnOsZ4DSyuTJu6kWA5S2zxOWJOrc3gxQyIhhAIwBMMGrH2JP780AMAPADIBJWf2QcUc/BoAZAGYATOLqh7ybejAAzAAwA2DSVz9k31eXAWAGgBkAA61+YG6txQAwA8AMgEFXP5B38wwAMwDMAJgCqx/4CRgGgBkA9kIBxqtGO7/KMc/554PFzzFeONr5hY55zj8fLGuOOq9dZ5JPWXPUee06k3zKnYN9efb2H+ROw66Avf0HxDTUIqh7DxHTUIug7j3ETZO5jsy7JnEzZS4l865J9Ew5q8m55RJ6ppzV5NxySY2Z4hYUd/IiNSaLW1PcyYtUmixiWRFnLlVpsohlRZy5VL3JVq1s1TnB6s23anGrzglWdb7767t/Qoqq891f3/0TUtSe79oSr30LUnvKa6u89i1IhylnFzr7PKrDlLMLnX0e1WPKhzNrPfNMMX1mPbPcM88U02nWh+MVH39aUqdZH45XfPxpSZ1mfdovev+fJvpN/LBf9/4/TfSb+Gm79O3frfSb+Gm79O3frfSb+Ku2qx+6zv1kAN1hAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmAJgBYAaAGQBmANhfnlfkgaL6FZsAAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::Polyline,
//!     style::PrimitiveStyle,
//! };
//!
//! let points: [Point; 5] = [
//!     Point::new(8, 8),
//!     Point::new(48, 16),
//!     Point::new(32, 48),
//!     Point::new(16, 32),
//!     Point::new(32, 32),
//! ];
//!
//! Polyline::new(&points)
//!     .into_styled(PrimitiveStyle::with_stroke(Rgb888::CYAN, 1))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw a rectangle with rounded corners
//!
//! This example draws a rectangle with rounded corners, red stroke and green fill.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw a rectangle with rounded corners example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAB8UlEQVR4nO3RwW7DIBQF0eb/PzqtBAtHDmDzIJPCHMkb27C48/gRygAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzACwiQGef88q5s0072YDXDLs5pXmvmLUcKPuMUCnAffsNv1RfL74DQYICd1Qn/5R//yvPKs7VT82RM4aIKt+bOg8W9p2pdFLSjEKrxv6ThngjcLrhtunSgvvMP3RqAx3/zdA9kUBdpv+6Jzh9KLh7v8GeGEAmAFgHw1w3nnn6Y8iGa7/aYAiA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMADMAzAAwA8AMAPtQgOS8+c4ZItMnd/83wAsDwAwA+4oAyW4ZztMnhddFd/83QIYFSEpr75Bh1PRJ3ykDvFF43dB3KquvvVKM0uhJ9WND5KwBsurHhsjZbKGdb4vPF7/BACHxG7LdMowabtQ9Bug06p43Vkoyb6Z5Nxvgknk36xIDwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwAwAMwDMADADwH4B4Bq0gX3sDWQAAAAASUVORK5CYII=" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     primitives::{
//!         Rectangle,
//!         RoundedRectangle,
//!     },
//!     style::PrimitiveStyleBuilder,
//! };
//!
//! let style = PrimitiveStyleBuilder::new()
//!     .stroke_color(Rgb888::RED)
//!     .stroke_width(3)
//!     .fill_color(Rgb888::GREEN)
//!     .build();
//!
//! RoundedRectangle::with_equal_corners(
//!     Rectangle::new(Point::new(8, 16), Size::new(48, 32)),
//!     Size::new(10, 10),
//! )
//! .into_styled(style)
//! .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Draw some text
//!
//! This example draws the text "Hello,\nRust!" with the [`Font6x8`] font in green.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Draw some text example screenshot" src="data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAADBklEQVR4nO3R247bUAxD0fb/P7ptwBeqGhky5sIcYy+ggEEz0yj8/QtRDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBBWB/jz79//2cuUX9OnZPrspvNo9Wj9HDV7mfJr+pRMn910Hq0erZ+jZi9TvrH57KbzUPXo6YeY8o3NZzedh6pHTz9Ez5U4f+vUnN7KdUdv3dQ8UD1Fp9bspedKnL91ak5v5bqjt25qHqie0k916qqjZ3c3d1Pnbn6geoQOm6irjp7d3dxNnbv5geoR02Ge63mijlO/527q3M0PVI+YDvNczxN1nPo9d1Pnbn6gesR0mOf+vLHpT519rkQ8f3v1y+qMmr147s8bm/7U2edKxPO3V7+szqjZS8+VOH8rP9kRNae3b6l+2emAnitx/lZ+siNqTm/f0lFf9sKBP70c+JU/9JABdEZXW19J/+Pm7++bR6kH6ciutr6S/sfN3983j1IPmo6c8s/b/+V98yj1oOnIKf+8/V/eN49SD5qO9NyfXc+VdOr0t8on6l93DlQPmo703J9dz5V06vS3yifqX3cOVA/SkZ231PFEPPfna9/RPEo9SEd23lLHE/Hcn699R/Mo9SA/0p/dPlfi/K2o0/Nu3zxKPciP9Ge3z5U4fyvq9LzbN49SD+pH7hPxvFPTO5vk0eqh/fh9Ip53anpnkzxaPXQ6vudKRLkSf+70tvP+dWd6e6x60HRkz5WIciX+3Olt5/3rzvT2WI876DQMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBDGAGEMEMYAYQwQxgBhDBD2FyjfqoEr2rxJAAAAAElFTkSuQmCC" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     fonts::{
//!         Font6x8,
//!         Text,
//!     },
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     style::TextStyle,
//! };
//!
//! Text::new("Hello,\nRust!", Point::new(2, 28))
//!     .into_styled(TextStyle::new(Font6x8, Rgb888::GREEN))
//!     .draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! ## Display a TGA image
//!
//! This example uses [tinytga](https://crates.io/crates/tinytga) to draw an image to the display.
//!
//! <div style="display: flex">
//! <img style="width: 128px; height: 128px; margin-right: 8px;" alt="Display a TGA image example screenshot" src="data:image/png;base64,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" />
//! <div style="flex-grow: 1;">
//!
//! ```rust
//! # let mut display = embedded_graphics::mock_display::MockDisplay::default();
//! # display.set_allow_overdraw(true);
//! use embedded_graphics::{
//!     image::Image,
//!     pixelcolor::Rgb888,
//!     prelude::*,
//! };
//! use tinytga::Tga;
//!
//! let tga = Tga::from_slice(include_bytes!(concat!(
//!     env!("CARGO_MANIFEST_DIR"),
//!     "/../simulator/examples/assets/rust-pride.tga"
//! )))
//! .unwrap();
//!
//! let image: Image<Tga, Rgb888> = Image::new(&tga, Point::zero());
//!
//! image.draw(&mut display)?;
//! # Ok::<(), core::convert::Infallible>(())
//! ```
//!
//! </div>
//! </div>
//!
//! [`Circle`]: ./primitives/circle/struct.Circle.html
//! [`Point`]: ./geometry/struct.Point.html
//! [`Size`]: ./geometry/struct.Size.html
//! [`Font6x8`]: ./fonts/struct.Font6x8.html
//! [`DrawTarget`]: ./draw_target/trait.DrawTarget.html
//! [`Drawable`]: ./drawable/trait.Drawable.html

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/jamwaffles/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true"
)]
#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

#[cfg(feature = "nalgebra_support")]
extern crate nalgebra;

mod draw_target;
mod drawable;
pub mod fonts;
pub mod geometry;
pub mod image;
pub mod mock_display;
pub mod pixelcolor;
pub mod prelude;
pub mod primitives;
pub mod style;
pub mod transform;

pub use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
};
