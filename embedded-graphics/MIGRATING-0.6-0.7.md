# Migrating from embedded-graphics 0.6.x to 0.7.0

## Table of contents

// TODO

## New features

// TODO: This entire section should probably be moved into a GH release or blog post. Leaving here for now.

### Primitives

New primitives have been added:

- `Arc`
- `Sector`
- `Polyline`
- `Ellipse`

The `Line` primitive now supports stroke widths greater than 1px.

Primitives now support stroke alignment using the `StrokeAlignment` enum.

### Mock display

`MockDisplay` now supports all RGB and grayscale color types in its patterns.

New methods:

- `swap_xy` - copies the current display with X and Y coordinates swapped.
- `map` - create a copy of the current display with a predicate applied to all pixels.
- `affected_area` - gets the bounding box of all changes made to the display.

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

The `Rectangle::intersection` method was added to get the intersecting `Rectangle` between `self` and another given `Rectangle`.

### Color

The `ToBytes` trait has been added to support conversion of colors into byte arrays.

## For display driver authors

`DrawTarget` now uses an associated type for the target color instead of a type parameter.

`DrawTarget`s must also implement the `Dimensions` trait.

### Method changes

All `draw_*` methods to draw specific primitives (`draw_circle`, `draw_triangle`, etc) have been removed. The new methods provided by `DrawTarget` are as follows:

- `draw_iter`

  Draw individual pixels to the display without a defined order. This is the only required method in this trait, however will likely be the slowest pixel drawing implementation as it cannot take advantage of hardware accelerated features (e.g. filling a given area with a solid color with `fill_solid`).

- `fill_contiguous`

  Fills a given area with an iterator providing a contiguous stream of pixel colors. This may be used to efficiently draw an image or other non-transparent item to the display. The given pixel iterator can be assumed to be contiguous, iterating from top to bottom, each row left to right. This assumption potentially allows more efficient streaming of pixel data to a display.

- `fill_solid`

  Fills a given area with a solid color.

- `clear`

  Fill the entire display with a solid color.

These methods aim to be more compatible with hardware-accelerated drawing commands. Where possible, embedded-graphics primitives will use `fill_contiguous` and `fill_solid` to improve performance, however may fall back to `draw_iter` by default.

To reduce duplication, please search the `DrawTarget` documentation on <docs.rs/embedded-graphics> for more details on the usage and arguments of the above methods.

### Sub draw targets

The `DrawTargetExt` trait is introduced to allow a translated, cropped or clipped sub-area of the display to be drawn to.

`DrawTargetExt` is implemented for `DrawTarget`.

Please search for `DrawTargetExt` on <docs.rs/embedded-graphics> for usage examples.

## General

### `Drawable`

The `Drawable` trait now uses an associated type for its pixel color instead of a type parameters.

```diff
impl<'a, C: 'a> Drawable<C> for &Button<'a, C>
where
    C: PixelColor + From<BinaryColor>,
{
    fn draw<D>(self, display: &mut D) -> Result<(), D::Error> where D: DrawTarget<C> {
        // ...
    }
}
impl<C> Drawable for Button<'_, C>
where
    C: PixelColor + From<BinaryColor>,
{
    type Color = C;

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        Rectangle::new(self.top_left, self.size)
            .into_styled(PrimitiveStyle::with_fill(self.bg_color))
            .draw(display)?;
        Text::new(self.text, Point::new(6, 6))
            .into_styled(TextStyle::new(Font6x8, self.fg_color))
            .draw(display)
    }
}
```

### `IntoIterator` changes

All `IntoIterator` impls are now replaced with the `IntoPixels` trait which exposes the `into_pixels()` method. This trait is included in the prelude, or can be included by adding `embedded_graphics::iterator::IntoPixels` to the imports list. Using the prelude is recommended.

For example, chaining two items together now requires explicit calls to `into_pixels()`:

```diff
+ use embedded_graphics::prelude::*;

let background = Rectangle::new(...);
let text = Text::new(...);

- background.into_iter().chain(&text)
+ background.into_pixels().chain(text.into_pixels())
```

## Macros are removed

All text, primitive and style macros have been removed. To create text, primitives and styles, use the appropriate struct methods instead.

```diff
+ TODO: Example
```

## Primitives

### Circle

A circle is now defined by it's top-left corner and diameter.

```diff
// Create a circle centered around (30, 30) with a diameter of 20px

use embedded_graphics::{geometry::Point, primitives::Circle};

- let circle = Circle::new(Point::new(30, 30), 10);
+ let circle = Circle::new(Point::new(20, 20), 20);
```

This allows circles with odd diameters to be created.

To retain old behaviour and create a circle from a center point and radius, use `Circle::with_center`.

```diff
use embedded_graphics::{geometry::Point, primitives::Circle};

- let circle = Circle::new(Point::new(20, 20), 5);
+ let circle = Circle::with_center(Point::new(20, 20), 5);
```

### Rectangle

Rectangles are now defined by their top-left corner and size instead of the top-left and bottom-right corner.

```diff
use embedded_graphics::{geometry::{Point, Size}, primitives::Rectangle};

- let rectangle = Rectangle::new(Point::new(20, 30), Point::new(40, 50));
+ let rectangle = Rectangle::new(Point::new(20, 30), Size::new(20, 30));
```

To retain the old behaviour, use `Rectangle::with_corners` instead:

```diff
use embedded_graphics::{geometry::{Point}, primitives::Rectangle};

- let rectangle = Rectangle::new(Point::new(20, 30), Point::new(40, 50));
+ let rectangle = Rectangle::with_corners(Point::new(20, 30), Point::new(40, 50));
```

## Geometry

Implementations of the `Dimensions` trait now only require the `bounding_box` method to be implemented. This should return a rectangle encompasing the entire shape.

## Mock display

The `MockDisplay`, used often for unit testing, now checks for pixel overdraw by default. To disable this behaviour, call `set_allow_overdraw(false)` on the `MockDisplay` instance. It now also disallows out of bounds drawing by default. This behaviour can be changed by calling `display.set_allow_out_of_bounds_drawing(true)`.
