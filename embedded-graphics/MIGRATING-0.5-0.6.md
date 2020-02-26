# Migrating from embedded-graphics 0.5.x to 0.6.0

## Pixel colours

TODO for both driver authors and library consumers

## For driver authors

Adding embedded-graphics support to a display driver has changed somewhat. The `Drawing` trait has been replaced by the `DrawTarget` trait which provides a slightly different interface.

An associated error type is also added to `DrawTarget` to allow for better error handling than a `panic!()`.

Implementations for display drivers should now at minimum look something like the following:

```rust
use embedded_graphics::{
    drawable,
    geometry::Size,
    pixelcolor::{
        raw::{RawData, RawU1},
        BinaryColor,
    },
    DrawTarget,
};

#[cfg(feature = "graphics")]
impl DrawTarget<BinaryColor> for DisplayDriver
{
    type Error = core::convert::Infallible;

    fn draw_pixel(&mut self, pixel: drawable::Pixel<BinaryColor>) -> Result<(), Self::Error> {
        let drawable::Pixel(pos, color) = pixel;

        // Guard against negative values. All positive i32 values from `pos` can be represented in
        // the `u32`s that `set_pixel()` accepts...
        if pos.x < 0 || pos.y < 0 {
            return Ok(());
        }

        // ... which makes the `as` coercions here safe.
        self.set_pixel(pos.x as u32, pos.y as u32, RawU1::from(color).into_inner());

        Ok(())
    }

    fn size(&self) -> Size {
        let (w, h) = self.get_dimensions();

        Size::new(w as u32, h as u32)
    }
}
```

This is a reduced example taken from the [ssd1306](https://crates.io/crates/ssd1306) driver. It uses `BinaryColor` as the SSD1306 can only be on or off.

Some notes on the above:

- The `Drawing` trait is renamed to `DrawTarget`.
- The `draw()` method is replaced by `draw_pixel()`. This method should handle setting of individual pixels on the display. How it does this is at the descretion of the display driver (pixel buffer, immediate mode, etc).
- A new `size()` method is now a required item. This should return the width and height of the display as a `Size`.
- The implementation of `DrawTarget` must now provide an associated `Error` type. In the above example `core::convert::Infallible` is used, however a better error type should be used to communicate hardware failures, etc to the user.
- Any pixels that are offscreen (negative coordinates or greater than display dimensions) should result in a noop. In the example above, `self.set_pixel()` (defined by the SSD1306 driver) will not attempt to set any pixels beyond the positive screen limits.
- `draw_pixel()` now returns `Result<(), Self::Error>` to account for driver error handling.

## For font authors

- The `embedded_graphics::fonts::font_builder` module along with its exported `FontBuilderConf` and `FontBuilder` is removed. Now only `Font` needs to be implemented.
- The `Font` trait (used to be `FontBuilderConf`) implementation has changed slightly, replacing `CHART_WIDTH` and `CHAR_HEIGHT` with a single `CHARACTER_SIZE` constant, using the `Size` struct:

  ```diff
  - use embedded_graphics::fonts::font_builder::{FontBuilder, FontBuilderConf};
  + use embedded_graphics::fonts::Font;

  + use embedded_graphics::geometry::Size;

    #[derive(Debug, Copy, Clone)]
    pub enum MyFont {}

    impl Font for MyFont {
        const FONT_IMAGE: &'static [u8] = include_bytes!("../data/my_font.raw");
  -     const CHAR_WIDTH: u32 = 12;
  -     const CHAR_HEIGHT: u32 = 22;
  +     const CHARACTER_SIZE: Size = Size::new(12, 22);
        const FONT_IMAGE_WIDTH: u32 = 480;
    }
  ```

## For library consumers

### General drawing operations

Drawing operations are "reversed" in 0.6.0. Instead of calling `display.draw(thing)`, call `thing.draw(&mut display)`:

```diff
- let rect = Rectangle::new(Coord::new(50, 20), Coord::new(60, 35))
-     .stroke(Some(5u8))
-     .stroke_width(3)
-     .fill(Some(10u8));
-
- display.draw(rect);

+ let style = PrimitiveStyleBuilder::new()
+     .stroke_color(Rgb565::RED)
+     .stroke_width(3)
+     .fill_color(Rgb565::GREEN)
+     .build();
+
+ Rectangle::new(Point::new(50, 20), Point::new(60, 35))
+     .into_styled(style)
+     .draw(&mut display)?;
```

Drawing operations are now fallible, with `.draw()` calls returning a `Result`. This allows for error handling if an error occurs during a drawing operation.

### Coordinates and positioning

The `Coord` and `UnsignedCoord` have been renamed to `Point` and `Size` respectively. Both items are no longer tuple structs, but structs with the named fields `x` and `y`. `Point`s can store negative coordinates, whereas `Size`s must be positive up to `u32::MAX_VALUE` as with `Coord` and `UnsignedCoord` before.

The `icoord` and `ucoord` macros have been removed. Instead, use `Point::new(x, y)` and `Size::new(x, y)` respectively.

`Point::zero()` and `Size::zero()` were also added to get zero-filled coordinates.

```diff
- let c = Coord::new(-10, 20);
+ let c = Point::new(-10, 20);

- println!("X: {}, Y: {}", c.0, c.1);
+ println!("X: {}, Y: {}", c.x, c.y);

- let p = UnsignedCoord::new(5, 15);
+ let p = Size::new(5, 15);

- println!("X: {}, Y: {}", p.0, p.1);
+ println!("X: {}, Y: {}", p.x, p.y);
```

### Text

Text is now drawn with the `Text` struct. A `TextStyle` must be provided which selects which font to draw the text with.

```diff
- use embedded_graphics::{
-     prelude::*,
-     fonts::Font6x8,
- };
-
- display.draw(
-     Font6x8::render_str("Hello Rust!")
-         .translate(Coord::new(20, 30))
-         .fill(Some(1u8))
-         .stroke(Some(0u8)),
- );

+ use embedded_graphics::{
+     fonts::{Font6x8, Text},
+     pixelcolor::BinaryColor,
+     prelude::*,
+     style::TextStyleBuilder,
+ };
+
+ // Create a new text style
+ let style = TextStyleBuilder::new(Font6x8)
+     .text_color(BinaryColor::On)
+     .background_color(BinaryColor::Off)
+     .build();
+
+ // Create a text at position (20, 30) and draw it using style defined above
+ Text::new("Hello Rust!", Point::new(20, 30))
+     .into_styled(style)
+     .draw(&mut display)?;
```

Macro usage has also changed:

```diff
- use embedded_graphics::{prelude::*, text_6x8};
-
- display.draw(
-     text_6x8!(
-         "Hello Rust!",
-         stroke = Some(1u8.into()),
-         fill = Some(0u8.into())
-     )
-     .translate(Coord::new(20, 30))
- );

+ use embedded_graphics::{egtext, text_style, fonts::Font6x8, pixelcolor::BinaryColor, prelude::*};
+
+ egtext!(
+     text = "Hello Rust!",
+     top_left = (20, 30),
+     style = text_style!(
+         font = Font6x8,
+         text_color = BinaryColor::On,
+         background_color = BinaryColor::Off,
+     )
+ )
+ .draw(&mut display)?;
```

### Primitives

Primitives changes:

- Note: The `Triangle` primitive was added. If you were using 3 separate lines, this primitive is suggested instead.

- Primitive styles now default to a stroke width of `0` instead of `1`. If nothing is drawn to the display, check that you set the `stroke_width` in `PrimitiveStyleBuilder`.

### Images

The `ImageBMP` and `ImageTGA` built-in image types are removed in favour of using the new `graphics` features of the [tinybmp](https://crates.io/crates/tinybmp) and [tinytga](https://crates.io/crates/tinytga) crates. These must be wrapped in the `Image` struct for use with embedded-graphics.

First, change the imports in `Cargo.toml`:

```diff
[dependencies]
- embedded-graphics = { version = "0.5.1", features = [ "tga", "bmp" ] }

+ embedded-graphics = "0.6.0"
+ tinytga = { version = "0.3.0", features = [ "graphics" ] }
+ tinybmp = { version = "0.2.0", features = [ "graphics" ] }
```

Then any image code:

```diff
- use embedded_graphics::{prelude::*, image::ImageBmp};
-
- let image = ImageBmp::new(include_bytes!("../../../assets/patch_16bpp.bmp")).unwrap();
-
- display.draw(&image);

+ use embedded_graphics::{prelude::*, image::Image, geometry::Point}
+ use tinybmp::Bmp;
+
+ let bmp = Bmp::from_slice(include_bytes!("../../../assets/patch_16bpp.bmp")).unwrap();
+
+ let image = Image::new(&bmp, Point::zero());
+
+ image.draw(&mut display)?;
```
