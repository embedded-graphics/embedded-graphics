# Releases

Release notes for [embedded-graphics](https://crates.io/crates/embedded-graphics).

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

The outline of all closed shapes now can now be offset by using the `OffsetOutline::offset` method.

The `ContainsPoint::contains` method can be used to check if a point is inside a primitive.

All primitives can be converted into an iterator over the contained points by using the `PointsIter::points` method.

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

Other methods added to both `Point` and `Size` are:

- `component_min`
- `component_max`
- `component_mul`
- `component_div`

`Angle` was added to represent angles in `Arc` and `Sector` primitives.

The `Rectangle::intersection` method was added to get the intersecting area of two rectangles.

### Color

New color constants for the CSS colors were added for all RGB color types. The color constants are
defined in the `WebColors` trait, which is included in the prelude. The names of the constants are
prefixed by `CSS_` to avoid naming conflicts with the existing color constants. The CSS color
`hotpink` can, for example, be accessed by using `Rgb888::CSS_HOT_PINK`.

The `ToBytes` trait has been added to support conversion of colors into byte arrays.

All builtin color types can now be converted to any other builtin color type by using `From` or `Into`.

### Draw target adapters

The `DrawTargetExt` trait is introduced to allow a translated, cropped or clipped sub-area of a `DrawTarget` to be drawn to.

`DrawTargetExt` is implemented for `DrawTarget`.

Please search for `DrawTargetExt` at <https://docs.rs/embedded-graphics> for usage examples.

### Fonts and text

Support for external text renderers was added. To support this the existing `fonts` module has been
split into one for the generic `Text` drawable and one for the builtin monospaced text renderer.
Refer to the migration guide for more information about this change.

Additional languages are supported by new fonts which are available in ASCII, multiple ISO-8859 and
JIS X 201 glpyh subsets.

`Text` drawables now support horizontal alignment, baseline and line height settings.

The builtin monospaced text renderer now supports underline and strikethough decorations.

```rust
use embedded_graphics::{
    mono_font::{ascii::FONT_8X13, MonoTextStyleBuilder},
    pixelcolor::Rgb888,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyleBuilder},
};

// Use the builtin 8x13 monospace font with red foreground color
let character_style = MonoTextStyleBuilder::new()
    .font(&FONT_8X13)
    .text_color(Rgb888::CSS_TOMATO)
    .build();

// Anchor the text relative to its horizontal center point and vertical middle
let center_aligned = TextStyleBuilder::new()
    .alignment(Alignment::Center)
    .baseline(Baseline::Middle)
    .build();

Text::with_text_style(
    "Center aligned text",
    Point::new(100, 20),
    character_style,
    center_aligned,
)
.draw(&mut display)?;
```

### Images

`SubImage` was added to draw a part of an image.

```rust
use embedded_graphics::{image::Image, pixelcolor::Rgb888, prelude::*, primitives::Rectangle};
use tinytga::Tga;

// Load spritemap TGA image
let tiles: Tga<Rgb888> = Tga::from_slice(include_bytes!("assets/sprites.tga")).unwrap();

// Grab a 64x64px subsection of the image
let sprite_a = tiles.sub_image(&Rectangle::new(Point::new(0, 0), Size::new(64, 64)));

// Draw the sprite with its top left corner at (25, 35)
Image::new(&sprite_a, Point::new(25, 35)).draw(&mut display)?;
```

### Performance

The performance of many drawing operations has been improved by drawing larger contiguous regions
of pixels instead of individual pixels.

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
- `from_points` - create a `MockDisplay` from an iterator over `Point`s.
- `map` - create a copy of the current display with a predicate applied to all pixels.
- `set_allow_out_of_bounds_drawing` - if set to `true`, disables the panicking behavior when a drawing operation attempts to draw pixels outside the visible mock display area.
- `set_allow_overdraw` - if set to `true`, disables the panicking behavior when a pixel is drawn to twice.
- `set_pixels` - sets the points in an iterator to the given color.
- `swap_xy` - copies the current display with X and Y coordinates swapped.
