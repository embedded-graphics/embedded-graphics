# Migrating from embedded-graphics 0.6.x to 0.7.0

> Please note that this migration guide may be incomplete in some sections. If any missing or incorrect information is found, please [open an issue](https://github.com/embedded-graphics/embedded-graphics/issues/new) or [join the Matrix chatroom](https://matrix.to/#/#rust-embedded-graphics:matrix.org) to bring it to our attention.

## Table of contents

- [For display driver authors](#for-display-driver-authors)
  - [Method changes](#method-changes)
- [For crates that handle images](#for-crates-that-handle-images)
- [For text rendering crates](#for-text-rendering-crates)
  - [Monospace fonts](#monospace-fonts)
  - [More complex fonts](#more-complex-fonts)
- [General](#general)
  - [`Drawable`](#drawable)
  - [`IntoIterator` changes](#intoiterator-changes)
- [Macros are removed](#macros-are-removed)
- [Primitives](#primitives)
  - [Circle](#circle)
  - [Rectangle](#rectangle)
  - [Triangle](#triangle)
- [Geometry](#geometry)
- [Mock display](#mock-display)
- [Style module](#style-module)
- [Text and fonts](#text-and-fonts)

## For display driver authors

Driver authors should use `DrawTarget` exported by the [`embedded-graphics-core`](https://crates.io/crates/embedded-graphics-core) crate to integrate with embedded-graphics.

`DrawTarget` now uses an associated type for the target color instead of a type parameter. As this can be a limitation versus older code which implements `DrawTarget` for e.g. `C: Into<Rgb565>`, the `color_converted` method can be used to create a draw target which converts the drawable's color format to the display's color format.

The `DrawTarget` trait now has an additional bound on the `Dimensions` trait to replace the removed `size` method. By using the `Dimensions` trait the drawable area of a draw targets can be positioned freely and is no longer limited to start in the origin at `(0, 0)`. But for display drivers it is recommended that the drawable area does start at `(0, 0)`. To simplify implementation and provide a type level guarantee that the drawable area starts at the origin ,`OriginDimensions` can be implemented instead of `Dimensions`. The `Dimensions` trait is automatically implemented for all types that implement `OriginDimensions`.

For example, the `SSD1306` driver using the on/off `BinaryColor` would change as follows:

```diff
- use crate::{
-     drawable::Pixel,
-     geometry::Size,
-     pixelcolor::{PixelColor, BinaryColor},
-     DrawTarget,
- };
-
- impl DrawTarget<BinaryColor> for Ssd1306 {
-     type Error = core::convert::Infallible;
-
-     fn draw_pixel(&mut self, pixel: Pixel<BinaryColor>) -> Result<(), Self::Error> {
-         // ...
-
-         Ok(())
-     }
-
-     fn size(&self) -> Size {
-         // ...
-     }
- }
+ use embedded_graphics_core::{
+     draw_target::DrawTarget,
+     geometry::{OriginDimensions, Size},
+     pixelcolor::{PixelColor, BinaryColor},
+     Pixel,
+ };
+
+ DrawTarget for Ssd1306 {
+     type Color = BinaryColor;
+     type Error = core::convert::Infallible;
+
+     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
+     where
+         I: IntoIterator<Item = Pixel<Self::Color>>,
+     {
+         // ...
+
+         Ok(())
+     }
+ }
+
+ impl OriginDimensions for Ssd1306 {
+     fn size(&self) -> Size {
+         // ...
+     }
+ }
```

### Method changes

All `draw_*` methods to draw specific primitives (`draw_circle`, `draw_triangle`, etc) have been removed. These methods were hard to implement correctly and consistently between different drivers. The new lower level draw methods are easier to implement and still improve performance over pixel by pixel drawing.

- `draw_iter`

  Draws individual pixels to the display without a defined order. This is the only required method in this trait, however will likely be the slowest pixel drawing implementation as it cannot take advantage of hardware accelerated features (e.g. filling a given area with a solid color with `fill_solid`).

- `fill_contiguous`

  Fills a given area with an iterator providing a contiguous stream of pixel colors. This may be used to efficiently draw an image or other non-transparent item to the display. The given pixel iterator can be assumed to be contiguous, iterating from top to bottom, each row left to right. This assumption potentially allows more efficient streaming of pixel data to a display.

- `fill_solid`

  Fills a given area with a solid color.

- `clear`

  Fills the entire display with a solid color.

These methods aim to be more compatible with hardware-accelerated drawing commands. Where possible, embedded-graphics drawables will use `fill_contiguous` and `fill_solid` to improve performance, however may fall back to `draw_iter` by default.

To reduce duplication, please search the `DrawTarget` documentation on <https://docs.rs/embedded-graphics-core> for more details on the usage and arguments of the above methods.

## For crates that handle images

Crates that handle images must now implement the `ImageDrawable` and `OriginDimensions` traits to integrate with embedded-graphics.

The below examples shows an implementation for an imaginary `MyRgb888Image` which uses 24 bit color and draws to targets that support the same.

```rust
use embedded_graphics::{
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{OriginDimensions, Size},
    image::ImageDrawable,
    pixelcolor::{PixelColor, Rgb888},
    primitives::Rectangle,
};

struct MyRgb888Image {
  // ...
}

impl ImageDrawable for MyRgb888Image {
    type Color = Rgb888;

    fn draw<D>(&self, target: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Rgb888>,
    {
        // Draw the image to the target, e.g. by calling `target.fill_contiguous` or by using another `Drawable`.
    }

    fn draw_sub_image<D>(&self, target: &mut D, area: &Rectangle) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = Self::Color>,
    {
        // Delegate to the draw() method using a reduced draw target
        self.draw(&mut target.translated(-area.top_left).clipped(area))
    }
}

impl OriginDimensions for MyRgb888Image {
    fn size(&self) -> Size {
        // Return image width and height in pixels
    }
}
```

## For text rendering crates

### Monospace fonts

Monospaced fonts should now be built using the `MonoFontBuilder` struct and assigned to a `const`. The following example shows a migration of a font with 5x9 pixel characters using the Latin-1 encoding.

```diff
// The font bitmap has 32 character glyphs per row.
const CHARS_PER_ROW: u32 = 32;

// Each character is 5x9 px.
const GLYPH_SIZE: Size = Size::new(5, 9);

- // Map a given character to an index in the glyph bitmap
- fn char_offset_impl(c: char) -> u32 {
-     let fallback = '?' as u32 - ' ' as u32;
-     if c < ' ' {
-         return fallback;
-     }
-     if c <= '~' {
-         return c as u32 - ' ' as u32;
-     }
-     if c < '\u{00A0}' || c > 'ÿ' {
-         return fallback;
-     }
-     c as u32 - ' ' as u32 - 33
- }
-
- #[derive(Debug, Copy, Clone)]
- pub struct ExampleFont {}
- impl Font for ExampleFont {
-     const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ExampleFont.raw");
-     const CHARACTER_SIZE: Size = Size::new(5, 9);
-     const FONT_IMAGE_WIDTH: u32 = Self::CHARACTER_SIZE.width * CHARS_PER_ROW;
-
-     fn char_offset(c: char) -> u32 {
-         char_offset_impl(c)
-     }
- }
+ use embedded_graphics::{
+     geometry::Size,
+     image::ImageRaw,
+     mono_font::{GlyphIndices, GlyphRange, MonoFont, MonoFontBuilder},
+ };
+
+ pub const EXAMPLE_FONT: MonoFont = MonoFontBuilder::new()
+     .image(ImageRaw::new_binary(
+         include_bytes!("../data/ExampleFont.raw"),
+         CHARS_PER_ROW * GLYPH_SIZE.width,
+     ))
+     .character_size(GLYPH_SIZE)
+     .glyph_indices(GlyphIndices::new(
+         &[
+             // Base ASCII range
+             GlyphRange::new(' ', '~', 0),
+             // Latin1 range with offset 96 character positions into the glyph bitmap
+             GlyphRange::new('\u{00A0}', 'ÿ', 96),
+         ],
+         // Fallback: show unrecognised characters as a '?'
+         '?' as u32 - ' ' as u32,
+     ))
+     .build();
```

### More complex fonts

Crates that handle text rendering more complex than simple monospace fonts should now implement the
`CharacterStyle` and `TextRenderer` traits. These are used for both text styling and layout.

Please refer to their respective docs for implementation details.

An implementation of more complex font rendering using BDF font files is available in the [eg-bdf](https://github.com/embedded-graphics/embedded-bdf) crate, which may be useful as a reference for other implementations.

## General

### `Drawable`

The `Drawable` trait now uses an associated type for its pixel color instead of a type parameters.

An associated type, `Output`, has also been added which can be used to return values
from drawing operations. The unit type `()` can be used if the `draw` method doesn't need to return
anything, e.g. `type Output = ();`

```diff
- impl<'a, C: 'a> Drawable<C> for &Button<'a, C>
- where
-     C: PixelColor + From<BinaryColor>,
- {
-     fn draw<D>(self, display: &mut D) -> Result<(), D::Error> where D: DrawTarget<C> {
-         // ...
-     }
- }
+ impl<C> Drawable for Button<'_, C>
+ where
+     C: PixelColor + From<BinaryColor>,
+ {
+     type Color = C;
+
+     type Output = ();
+
+     fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
+     where
+         D: DrawTarget<Color = C>,
+     {
+         Rectangle::new(self.top_left, self.size)
+             .into_styled(PrimitiveStyle::with_fill(self.bg_color))
+             .draw(display)?;
+         Text::new(self.text, Point::new(6, 6))
+             .into_styled(TextStyle::new(Font6x8, self.fg_color))
+             .draw(display)
+     }
+ }
```

### `IntoIterator` changes

Styled primitives no longer implement `IntoIterator` to create a pixel iterator. Use the new `Styled::pixels` method instead.

For example, chaining two pixel iterators together now requires explicit calls to `pixels()`:

```diff
+ use embedded_graphics::prelude::*;

let background = Rectangle::new(...);
let text = Text::new(...);

- background.into_iter().chain(&text)
+ background.pixels().chain(text.pixels())
```

## Macros are removed

All text, primitive and style macros have been removed. To create text, primitives and styles, use the appropriate constructors or builders instead.

For example, a styled rectangle is now built like this:

```diff
- let filled_rect: Styled<Rectangle, PrimitiveStyle<Rgb565>> = egrectangle!(
-     top_left = (10, 20),
-     bottom_right = (30, 40),
-     style = primitive_style!(stroke_color = Rgb565::RED, fill_color = Rgb565::GREEN)
- );
+ let filled_rect = Rectangle::with_corners(Point::new(10, 20), Point::new(30, 40))
+     .into_styled(
+         PrimitiveStyleBuilder::new()
+            .stroke_color(Rgb565::RED)
+            .fill_color(Rgb565::GREEN)
+            .build()
+     );
```

## Primitives

Previously, drawing a filled shape with a transparent stroke of non-zero width would bleed the fill under the stroke. This is changed in 0.7 to honor the stroke width and alignment, even if it is the stroke color is `None`, allowing for filled shapes with transparent borders.

The stroke and fill color no longer affect the primitive's bounding box returned by `Dimensions::bounding_box`. The stroke width is now always considered even if the stroke is transparent.

### Circle

A circle is now defined by it's top-left corner and diameter. This has the advantage that circles
with odd diameters are now also supported.

```diff
// Create a circle centered around (30, 30) with a diameter of 20px

use embedded_graphics::{geometry::Point, primitives::Circle};

- let circle = Circle::new(Point::new(30, 30), 10);
+ let circle = Circle::new(Point::new(20, 20), 20);
```

To create a circle from a center point and diameter, use `Circle::with_center`:

```diff
use embedded_graphics::{geometry::Point, primitives::Circle};

- let circle = Circle::new(Point::new(20, 20), 5);
+ let circle = Circle::with_center(Point::new(20, 20), 10);
```

### Rectangle

Rectangles are now defined by their top-left corner and size instead of the top-left and bottom-right corner.

```diff
use embedded_graphics::{geometry::{Point, Size}, primitives::Rectangle};

- let rectangle = Rectangle::new(Point::new(20, 30), Point::new(40, 50));
+ let rectangle = Rectangle::new(Point::new(20, 30), Size::new(20, 30));
```

To retain the old behavior, use `Rectangle::with_corners` instead:

```diff
use embedded_graphics::{geometry::Point, primitives::Rectangle};

- let rectangle = Rectangle::new(Point::new(20, 30), Point::new(40, 50));
+ let rectangle = Rectangle::with_corners(Point::new(20, 30), Point::new(40, 50));
```

### Triangle

The vertices of a triangle are now stored in a single `vertices` field with the type `[Point; 3]`. Previously, they were stored in three separate fields `p1`, `p2` and `p3`.

To access an individual vertex of a triangle, use `triangle.vertices[]`.

```diff
use embedded_graphics::{prelude::*, primitives::Triangle};

let triangle = Triangle::new(Point::new(20, 30), Point::new(40, 50), Point::new(60, 70));

- let p1 = triangle.p1;
- let p2 = triangle.p2;
- let p3 = triangle.p3;
+ let p1 = triangle.vertices[0];
+ let p2 = triangle.vertices[1];
+ let p3 = triangle.vertices[2];
```

To create a triangle from a slice, use the new `Triangle::from_slice` method:

```rust
use embedded_graphics::{geometry::{Point}, primitives::Triangle};

let points = [Point::new(20, 30), Point::new(40, 50), Point::new(60, 70)];

let triangle = Triangle::from_slice(&points);
```

It is no longer possible to create a triangle from an array of `Point`s. Instead, pass a reference to `Triangle::from_slice`.

## Geometry

The three methods in the `Dimensions` trait were replaced by a single `bounding_box` method. This should return a `Rectangle` which encompasses the entire shape.

## Mock display

The `MockDisplay`, used often for unit testing, now checks for pixel overdraw and out of bounds drawing by default. These additional checks can be disabled by using the `set_allow_overdraw` and `set_allow_out_of_bounds_drawing` methods, if required.

The `width` and `height` methods have been removed. Use the `bounding_box` method provided by the `Dimensions` trait instead:

```rust
// Or: use embedded_graphics::prelude::*;
use embedded_graphics::geometry::Dimensions;

use embedded_graphics::mock_display::MockDisplay;

let display = MockDisplay::new();

let width = display.bounding_box().size.width;
let height = display.bounding_box().size.height;
```

An advanced visual representation of failing `MockDisplay` assertions can be enabled by setting the `EG_FANCY_PANIC` environment variable to `1`, for example, by calling `EG_FANCY_PANIC=1 cargo test`.

To make this output format possible assertions need to use the new `MockDisplay::assert_eq` and `assert_eq_with_message` methods instead of the `assert_eq!` macro. To ensure that the new methods are used the `PartialEq` implementation for `MockDisplay` was removed. In case that the equality of two mock `MockDisplay` needs to be tested outside of an assertion the `MockDisplay::eq` method can be used.

```rust
#[test]
fn check_equality() {
    let expected = MockDisplay::from_pattern(&[ /* ... */ ]);

    let mut display = MockDisplay::new();

    Circle::new(Point::new(1, 1), 1)
        .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
        .draw(&mut display)?;

    display.assert_eq(&expected);
}
```

The `assert_pattern` and `assert_pattern_with_message` can be used to check the display state against a pattern without using `MockDisplay::from_pattern` and a separate assertion.

```diff
- #[test]
- fn tiny_circle_filled() {
-     let mut display = MockDisplay::new();
-
-     Circle::new(Point::new(1, 1), 1)
-         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
-         .draw(&mut display)?;
-
-     assert_eq!(
-         display,
-         MockDisplay::from_pattern(&[
-             " # ",
-             "###",
-             " # "
-         ])
-     );
- }
+ #[test]
+ fn tiny_circle_filled() {
+     let mut display = MockDisplay::new();
+
+     Circle::new(Point::new(0, 0), 3)
+         .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
+         .draw(&mut display)
+         .unwrap();
+
+     display.assert_pattern(&[
+         " # ",
+         "###",
+         " # ",
+     ]);
+ }
```

## Style module

The `style` module has been removed. The items in it have been moved:

- `PrimitiveStyle`, `PrimitiveStyleBuilder` and `Styled` are now available in the `embedded_graphics::primitives` module.
- `TextStyle` and `TextStyleBuilder` were renamed are now available under `embedded_graphics::mono_font::{MonoTextStyle, MonoTextStyleBuilder}`.

  Note that usage with `Text` has changed. See [the text changes section](#Text-rendering) for more.

## Text and fonts

The collection of builtin fonts are now sourced from public domain BDF fonts in the XOrg project.
Due to this, they have slightly different dimensions and glyphs and so have changed names. Some
sizes are not the same in the new set, but a rough mapping is as follows:

| Old font                                                                                                                                                                             | Visually closest new font                                                                                                                                                                                                                                          |
| ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `fonts::Font6x6`<br>![Font6x6 glpyh bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.6.2/embedded-graphics/data/font6x6.png)       | `mono_font::{ascii, latin1}::FONT_4X6`<br>![FONT_4X6 glyph bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.7.0-beta.1/fonts/ascii/png/4x6.png)                                                                  |
| `fonts::Font6x8`<br>![Font6x8 glpyh bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.6.2/embedded-graphics/data/font6x8.png)       | `mono_font::{ascii, latin1}::FONT_6X10`<br>![FONT_6X10 glyph bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.7.0-beta.1/fonts/ascii/png/6x10.png)                                                               |
| `fonts::Font6x12`<br>![Font6x12 glpyh bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.6.2/embedded-graphics/data/font6x12.png)    | `mono_font::{ascii, latin1}::FONT_6X13`<br>![FONT_6X13 glyph bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.7.0-beta.1/fonts/ascii/png/6x13.png)                                                               |
| `fonts::Font8x16`<br>![Font8x16 glpyh bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.6.2/embedded-graphics/data/font8x16.png)    | `mono_font::{ascii, latin1}::FONT_9X15_BOLD`<br>![FONT_9X15_BOLD glyph bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.7.0-beta.1/fonts/ascii/png/9x15B.png)                                                    |
| `fonts::Font12x16`<br>![Font12x16 glpyh bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.6.2/embedded-graphics/data/font12x16.png) | `mono_font::{ascii, latin1}::FONT_10X20`<br>![FONT_10X20 glyph bitmap](https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/embedded-graphics-v0.7.0-beta.1/fonts/ascii/png/10x20.png)                                                            |
| `fonts::Font24x32`                                                                                                                                                                   | The largest available new font is `FONT_10X20`, which is significantly smaller than the old `Font24x32`. Larger fonts are available in [external crates](https://github.com/embedded-graphics/embedded-graphics#additional-functions-provided-by-external-crates). |

Note that the table above shows the new fonts' `ascii` variants only. Some new fonts also use taller bitmaps for the same cap height.

To style fonts, use the `MonoTextStyle` struct. `TextStyle` still exists, but has been repurposed to provide a more general interface to text styling. This more closely mirrors the way primitives are built and styled.

The default baseline for fonts is now the font's baseline instead of the top of the glyph bounding box. To retain the 0.6 behaviour and position text using its top-left corner, set the `baseline` property to `Baseline::Top`:

```rust
use embedded_graphics::text::{Baseline, TextStyle, TextStyleBuilder};

let style = TextStyle::with_baseline(Baseline::Top);

// OR

let style = TextStyleBuilder::new().baseline(Baseline::Top).build();
```
