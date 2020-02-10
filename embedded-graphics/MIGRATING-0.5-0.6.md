# Migrating from 0.5.x to 0.6.0

## Pixel colours

TODO for both driver authors and library consumers

## For driver authors

Adding embedded-graphics support to a display driver has changed somewhat. The `Drawing` trait has been replaced by the `DrawTarget` trait which provides a slightly different interface.

An associated error type is also added to `DrawTarget` to allow for better error handling than a `panic!()`.

```diff
// TODO: Mad example
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