# Releases

Relese notes for [embedded-graphics](https://crates.io/crates/embedded-graphics).

## 0.7.0

### Primitives

New primitives have been added:

- `Arc`
- `Sector`
- `Polyline`
- `Ellipse`
- `RoundedRectangle`

The `Line` and `Triangle` primitives now support stroke widths greater than 1px.

Primitives now support stroke alignment using the `StrokeAlignment` enum.

### Geometry

Adds the `Point::x_axis`, `Point::y_axis`, `Size::x_axis` and `Size::y_axis` methods to get one axis of a point or size, with the other set to zero.

```rust
use embedded_graphics::geometry::Point;

let point = Point::new(15, 23);

assert_eq!(point.x_axis() = Point::new(15, 0));
assert_eq!(point.y_axis() = Point::new(0, 23));
```

`Point::new_equal` and `Size::new_equal` have also been added as convenience methods to create points or sizes with equal coordinates.

```diff
use embedded_graphics::geometry::Size;

- let size = Size::new(20, 20);
+ let size = Size::new_equal(20);
```

`Point::length_squared` was added to `Point`.

Other methods added to both `Point` and `Size` are:

- `component_min`
- `component_max`
- `component_mul`
- `component_div`

The `Rectangle::intersection` method was added to get the intersecting area of two rectangles.

### Color

New color constants for the CSS colors were added for all RGB color types. The color constants are
defined in the `WebColors` trait, which is included in the prelude. The names of the constants are
prefixed by `CSS_` to avoid naming conflicts with the existing color constants. The CSS color
`hotpink` can, for example, be accessed by using `Rgb888::CSS_HOT_PINK`.

The `ToBytes` trait has been added to support conversion of colors into byte arrays.

### Draw target adapters

The `DrawTargetExt` trait is introduced to allow a translated, cropped or clipped sub-area of a `DrawTarget` to be drawn to.

`DrawTargetExt` is implemented for `DrawTarget`.

Please search for `DrawTargetExt` at <https://docs.rs/embedded-graphics> for usage examples.

### Fonts and text

TODO(rfuest): Improve this section before release:

Support for external font renderers has been added. TODO: Expand

- Added support for external renderers
- `MonoTextStyleBuilder::new(Font)` -> `MonoTextStyle::new().font(&Font)`
- Added support for underline and strikethrough to the internal text renderer
- `Text` no longer requires `into_styled`, because it directly contains `character_style` and `text_style`
- Added `TextStyle` to set the horizontal alignment and baseline for `Text` drawables
- New default baseline is alphabetic
- New fonts with `ascii` and `latin1` glyph subsets
- `MonoFont` is now a struct instead of a trait

The list of fonts has changed to the following:

All fonts are provided in `embedded_graphics::mono_font::{ascii, latin1}::[font name]`

- `FONT_4X6`
- `FONT_5X7`
- `FONT_5X8`
- `FONT_6X10`
- `FONT_6X12`
- `FONT_6X13`
  - Also available in **bold** (`FONT_6X13_BOLD`) and _italic_ (`FONT_6X13_ITALIC`)
- `FONT_6X9`
- `FONT_7X13`
  - Also available in **bold** (`FONT_7X13_BOLD`) and _italic_ (`FONT_7X13_ITALIC`)
- `FONT_7X14`
  - Also available in **bold** (`FONT_7X14`)
- `FONT_8X13`
  - Also available in **bold** (`FONT_8X13_BOLD`) and _italic_ (`FONT_8X13_ITALIC`)
- `FONT_9X15`
  - Also available in **bold** (`FONT_9X15_BOLD`)
- `FONT_9X18`
  - Also available in **bold** (`FONT_9X18_BOLD`)
- `FONT_10X20`

Two character sets are now provided for each font. The `ascii` set contains fewer characters and therefore has a reduced memory footprint, while `latin1` contains a larger number of glyphs.

- `embedded_graphics::mono_font::ascii` provides the characters U+0020 to U+007F in the [Basic Latin code block](<https://en.wikipedia.org/wiki/Basic_Latin_(Unicode_block)>), excluding control characters.
- `embedded_graphics::mono_font::latin1` provides all all characters in the `ascii` variant with the addition of U+00A0 to U+00FF in the [Latin-1 Supplement code block](<https://en.wikipedia.org/wiki/Latin-1_Supplement_(Unicode_block)>) (excluding control characters).

`ascii` has full coverage of the English language. For other languages, `latin1` has complete coverage for [the languages listed here](https://en.wikipedia.org/wiki/ISO/IEC_8859-1#Modern_languages_with_complete_coverage), and partial coverage for [these languages](https://en.wikipedia.org/wiki/ISO/IEC_8859-1#Languages_with_incomplete_coverage).

### Mock display

`MockDisplay` now supports all RGB and grayscale color types in its patterns.

By setting the environment variable `EG_FANCY_PANIC=1` when running the tests, failing `MockDisplay` assertions can now pretty print the difference between two `MockDisplay`s.

New methods:

- `affected_area` - gets the bounding box of all changes made to the display.
- `assert_eq_with_message` - the same as above, but with the ability to write custom messages in the test output.
- `assert_eq` - check for equality against another `MockDisplay` and panic if they are not the same.
- `assert_pattern_with_message` - the same as above, but with the ability to write custom messages in the test output.
- `assert_pattern` - check for equality against a pattern and panic if they do not match.
- `diff` - compare the display against another `MockDisplay`, producing a new `MockDisplay` containing the colored difference between them.
- `eq` - check for equality between two `MockDisplay`s.
- `from_points` - create a `MockDisplay` from an iterator over `Point`s.
- `map` - create a copy of the current display with a predicate applied to all pixels.
- `set_allow_out_of_bounds_drawing` - if set to `true`, disables the panicking behaviour when a drawing operation attempts to draw pixels outside the visible mock display area.
- `set_allow_overdraw` - if set to `true`, disables the panicking behaviour when a pixel is drawn to twice.
- `set_pixels` - sets the points in an iterator to the given color.
- `swap_xy` - copies the current display with X and Y coordinates swapped.
