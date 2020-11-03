# Migrating from embedded-graphics 0.5.x to 0.6.0

**Are you a driver author? Please see the [for driver authors](#for-driver-authors) section for required changes.**

**Are you a font author? Please see the [for font authors](#for-font-authors) section for required changes.**

As a general user of embedded-graphics, please read on.

## Table of contents

- [Table of contents](#table-of-contents)
- [Pixel colors](#pixel-colors)
  - [Color constants](#color-constants)
  - [Upgrading from the old `Rgb565` color type](#upgrading-from-the-old-rgb565-color-type)
- [General drawing operations](#general-drawing-operations)
- [Coordinates, sizing and positioning](#coordinates-sizing-and-positioning)
- [Text](#text)
- [Primitives and their styling](#primitives-and-their-styling)
- [Images](#images)
- [For driver authors](#for-driver-authors)
  - [Choosing the right pixel color type](#choosing-the-right-pixel-color-type)
- [For font authors](#for-font-authors)

## Pixel colors

The `u8`, `u16` or `u32` primitive types can no longer directly be used as pixel colors, because these don't allow the exact color format to be specified at the type level. This could lead to errors when colors were used in different places without being converted explicitly (e.g. when displaying a grayscale image on a color display).

Instead, multiple color types for commonly used color formats have been added. These pixel types distinguish their underlying storage from their color representation using the `Raw*` types. The table below lists the new colors and accompanying storage types.

| Type name     | Storage type | Underlying storage |
| ------------- | ------------ | ------------------ |
| `BinaryColor` | `RawU1`      | `u8`               |
| `Gray2`       | `RawU2`      | `u8`               |
| `Gray4`       | `RawU4`      | `u8`               |
| `Gray8`       | `RawU8`      | `u8`               |
| `Rgb555`      | `RawU16`     | `u16`              |
| `Rgb565`      | `RawU16`     | `u16`              |
| `Bgr555`      | `RawU16`     | `u16`              |
| `Bgr565`      | `RawU16`     | `u16`              |
| `Rgb888`      | `RawU24`     | `u32`              |
| `Bgr888`      | `RawU24`     | `u32`              |

Additional color types can be created for targets where no suitable builtin color type exists.

Creating various colors now works like this:

```rust
use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb565, Rgb888},
    prelude::*,
};

// Binary color (off/on)
let on = BinaryColor::On;

// Red with a small amount of green creates a deep orange color
let rust = Rgb565::new(0x1f, 0x07, 0x00);

// Use a color constant provided by the RGB color types
let magenta = Rgb888::MAGENTA;
```

Pixels can also be converted to their underlying storage by calling `.into_storage()`:

```rust
use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
};

let raw_value: u16 = Rgb565::MAGENTA.into_storage();
```

### Color constants

Predefined color constants have been added to all grayscale and RGB colors.

Usage:

```rust
use embedded_graphics::{prelude::*, pixelcolor::{Gray8, Rgb565}};

let white = Gray8::WHITE;
let cyan = Rgb565::CYAN;
```

#### Predefined colors for `Gray2`, `Gray4` and `Gray8`

These are provided by the `GrayColor` trait impl.

- `BLACK`
- `WHITE`

#### Predefined colors for `Rgb555` `Rgb565`, `Bgr555`, `Bgr565`, `Rgb888` and `Bgr888`

These are provided by the `RgbColor` trait impl.

- `BLACK`
- `WHITE`
- `RED`
- `GREEN`
- `BLUE`
- `CYAN`
- `MAGENTA`
- `YELLOW`

### Upgrading from the old `Rgb565` color type

0.5 also includes the `Rgb565` color type. Previously creating a new `Rgb565` required a tuple of 3 values. This is now replaced by the `Rgb565::new` method:

```diff
// Create an orange color
- let color: Rgb565 = (0x1f, 0x07, 0x00).into();
+ let color = Rgb565::new(0x1f, 0x07, 0x00);
```

## General drawing operations

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

Chaining now looks like this:

```rust
Circle::new(Point::new(64, 64), 64)
    .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
    .into_iter()
    .chain(
        &Line::new(Point::new(64, 64), Point::new(0, 64))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
    )
    .chain(
        &Line::new(Point::new(64, 64), Point::new(80, 80))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1)),
    )
    .chain(
        &Text::new("Hello World!", Point::new(5, 50))
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On)),
    )
    .draw(&mut display);
```

Drawing operations are now fallible, with `.draw()` calls returning a `Result`. This allows for error handling if an error occurs during a drawing operation. The error type of this `Result` is dependent on the associated `Error` type as defined by the display driver's `DrawTarget` impl.

## Coordinates, sizing and positioning

The `Coord` and `UnsignedCoord` have been replaced with `Point` and `Size` respectively. Both items are no longer tuple structs, but structs with the named fields `x` and `y` for `Point` and `width` and `height` for `Size`. `Point`s can store negative coordinates, whereas `Size`s must be positive as each dimension is stored as a `u32`.

It is important to note that the `Pixel` type now uses `Point` for it's coordinate component. In 0.5, `UnsignedCoord` was used. This change now allows `Pixel`s to represent negative coordinates, however drivers must check that pixels to be drawn to the display are non-negative.

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
+ println!("X: {}, Y: {}", p.width, p.height);
```

## Text

Text is now drawn with the `Text` struct. A `TextStyle` must be provided which selects which font and color to draw the text with. The `stroke_color` property from 0.5 is renamed to `text_color`. Likewise, `fill_color` is renamed to `background_color`.

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

- A new `Text` struct is introduced. Instead of `YourFont::render_str("text")`, use `Text::new("text", Point::zero())`.
- Text must be given a `TextStyle` for it to be drawn on a display. Create a style with either:

  - `TextStyleBuilder` or
  - `TextStyle::new(font, text_color)` to create a basic style in a more concise way.

  Styles are applied to the text object by calling `.into_styled(style)` as shown above.

- The chosen font is now part of the `TextStyleBuilder` creation process. Set it with `TextStyleBuilder::new(<your font here>)`.

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

- All built in `text_*!()` macros are removed and replaced with the `egtext!()` macro.
- The `text_style!()` macro can be used to create more complex text styles. For simple styles, `TextStyle::new(font, text_color)` can be used instead:

  ```rust
  egtext!(
      text = "Hello Rust!",
      top_left = (20, 30),
      style = TextStyle::new(Font6x8, BinaryColor::On)
  )
  .draw(&mut display)?;
  ```

Multiline text is now supported:

```diff
- use embedded_graphics::{prelude::*, text_6x8};
-
- display.draw(
-     text_6x8!(
-         "Hello,",
-         stroke = Some(1u8.into()),
-         fill = Some(0u8.into())
-     )
-     .translate(Coord::new(0, 8))
- );
- display.draw(
-     text_6x8!(
-         "World!",
-         stroke = Some(1u8.into()),
-         fill = Some(0u8.into())
-     )
-     .translate(Coord::new(0, 8))
- );

+ use embedded_graphics::{egtext, text_style, fonts::Font6x8, pixelcolor::BinaryColor, prelude::*};
+
+ egtext!(
+     text = "Hello,\nworld!",
+     top_left = (0, 0),
+     style = text_style!(
+         font = Font6x8,
+         text_color = BinaryColor::On,
+         background_color = BinaryColor::Off,
+     )
+ )
+ .draw(&mut display)?;
```

## Primitives and their styling

Note: The `Triangle` primitive was added in 0.4.8. If you were using 3 separate lines, this primitive is suggested instead.

**Primitive styles now default to a stroke width of `0` instead of `1`.** If nothing is drawn to the display, it may be that `stroke_width` is set to 0.

To style a primitive, a `PrimitiveStyle` must be used. To create simple styles, the `PrimitiveStyle::with_stroke()` and `PrimitiveStyle::with_fill()` methods can be used. For a more flexible way of creating styles, use `PrimitiveStyleBuilder`:

```rust
use embedded_graphics::{
    pixelcolor::{BinaryColor, Rgb565},
    prelude::*,
    style::{PrimitiveStyle, PrimitiveStyleBuilder},
};

// Simple style with only a 1px stroke
let style = PrimitiveStyle::with_stroke(BinaryColor::On, 1);

// Create a more complex style using the builder
let style: PrimitiveStyle<Rgb565> = PrimitiveStyleBuilder::new()
    .stroke_color(Rgb565::RED)
    .stroke_width(3)
    .fill_color(Rgb565::GREEN)
    .build();
```

## Images

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

## For driver authors

Adding embedded-graphics support to a display driver has changed somewhat. The `Drawing` trait has been replaced by the `DrawTarget` trait which provides a slightly different interface.

An associated error type is also added to `DrawTarget` to allow for better error handling than a `panic!()`.

Below is a reduced example taken from the [ssd1306](https://crates.io/crates/ssd1306) driver. It uses `BinaryColor` as pixels on the SSD1306 can only be on or off.

```rust
use embedded_graphics::{
    drawable,
    geometry::Size,
    pixelcolor::{
        raw::{RawData, RawU1},
        BinaryColor, PixelColor,
    },
    DrawTarget,
};

impl DrawTarget<BinaryColor> for DisplayDriver
{
    type Error = DI::Error;

    fn draw_pixel(&mut self, pixel: drawable::Pixel<BinaryColor>) -> Result<(), Self::Error> {
        let drawable::Pixel(pos, color) = pixel;

        // Guard against negative values. All positive i32 values from `pos` can be represented in
        // the `u32`s that `set_pixel()` accepts...
        if pos.x < 0 || pos.y < 0 {
            return Ok(());
        }

        // ... which makes the `as` coercions here safe.
        self.set_pixel(
            pos.x as u32,
            pos.y as u32,
            RawU1::from(color).into_inner()
        );

        Ok(())
    }

    fn size(&self) -> Size {
        let (w, h) = self.get_dimensions();

        Size::new(w as u32, h as u32)
    }
}
```

Some notes on the above:

- The `Drawing` trait is renamed to `DrawTarget`.
- This implementation takes a `BinaryColor` as it's color type as pixels the SSD1306 can only be off or on. [Other pixel types](#pixel-colors) can be used for displays with a higher color depth.
- The `Drawing::draw` method is replaced by `DrawTarget::draw_pixel`. This method should handle setting of individual pixels on the display. How it does this is at the discretion of the display driver (pixel buffer, immediate mode, etc).
- A new `DrawTarget::size` method is now a required, which should return the width and height of the display in pixels as a `Size`.
- The implementation of `DrawTarget` must now provide an associated `Error` type. If drawing operations can't fail, for example if the driver uses a framebuffer in RAM, `core::convert::Infallible` can used. If drawing operations can fail a better error type should be used to communicate hardware failures, etc to the user.
- Any pixels drawn to offscreen coordinates need to be ignored by `DrawTarget` implementations and not result in panics or any modification of the display content. In the example above, `self.set_pixel()` (defined by the SSD1306 driver) will not attempt to set any pixels beyond the positive screen limits.
- `draw_pixel()` now returns `Result<(), Self::Error>` to account for driver error handling.

New methods have been added to `DrawTarget` that can be overridden by hardware accelerated implementations:

- `DrawTarget::draw_circle`
- `DrawTarget::draw_rectangle`
- `DrawTarget::draw_triangle`
- `DrawTarget::draw_line`
- `DrawTarget::draw_image`

Methods that aren't overridden by the `DrawTarget` impl delegate to `DrawTarget::draw_iter` by default.

More information is provided in the [`DrawTarget` trait documentation](https://docs.rs/embedded-graphics/0.6.0/embedded_graphics/trait.DrawTarget.html).

### Choosing the right pixel color type

Below are some common use cases to help choose the right pixel color for a given display module. The full list of pixel colors is described in [the table above](#pixel-colors).

#### `BinaryColor`

If the display only supports two states, use `BinaryColor`. This is applicable to monochrome OLED displays like the SSD1306 or SH1106, character/bitmap LCDs and even LED matrices.

#### `Rgb565`

Use `Rgb565` for displays advertised as 16 bit color displays will often use a 5R 6G 5B pixel packing, storing each pixel color as a `u16`.

#### `Bgr565`

If the display's pixel order is reversed (BGR instead of RGB) and cannot be changed in configuration, use this type instead of `Rgb565`.

#### `Rgb888`

24 bit color, most useful in the embedded graphics simulator, or for high color display modules.

## For font authors

The `FontBuilderConf` trait is renamed to `Font`. This is the only trait impl that is required for embedded-graphics integration. `FontBuilder` is now removed, along with the `embedded_graphics::fonts::font_builder` module.

The `Font` trait implementation has changed slightly, replacing `CHAR_WIDTH` and `CHAR_HEIGHT` with a single `CHARACTER_SIZE` constant, using the `Size` struct:

```diff
- use embedded_graphics::fonts::font_builder::{FontBuilder, FontBuilderConf};
+ use embedded_graphics::fonts::Font;

+ use embedded_graphics::geometry::Size;

  #[derive(Debug, Copy, Clone)]
  pub struct MyFont;

  impl Font for MyFont {
      const FONT_IMAGE: &'static [u8] = include_bytes!("../data/my_font.raw");
-     const CHAR_WIDTH: u32 = 12;
-     const CHAR_HEIGHT: u32 = 22;
+     const CHARACTER_SIZE: Size = Size::new(12, 22);
      const FONT_IMAGE_WIDTH: u32 = 480;
  }
```

The `CHARACTER_SPACING` constant is added, providing configurable fixed spacing between characters. This is useful for fonts where there is no built-in spacing in the source bitmap data.

The `VARIABLE_WIDTH` constants is also added, creating support for basic variable width fonts. Setting this to `true` will trim empty pixels to the right of a glyph and add `CHARACTER_SPACING` between each character.
