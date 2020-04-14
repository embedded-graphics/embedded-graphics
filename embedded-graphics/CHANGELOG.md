# Changelog

Embedded Graphics is a `no_std` library for adding graphics features to display drivers. It aims to use the minimum amount of memory for builtin graphics objects by leveraging Rust's iterators to avoid large allocations. It targets embedded environments, but can run anywhere like a Raspberry Pi up to full desktop machines.

<!-- next-header -->

## [Unreleased] - ReleaseDate

### Added

- [#307](https://github.com/jamwaffles/embedded-graphics/pull/307) Added `Primitive::points` to get an iterator over all points inside a primitive.
- [#317](https://github.com/jamwaffles/embedded-graphics/pull/317) Added `Rectangle::center` to get the center point of a rectangle.
- [#318](https://github.com/jamwaffles/embedded-graphics/pull/317) Added `ContainsPoint` trait to check if a point is inside a closed shape.
- [#320](https://github.com/jamwaffles/embedded-graphics/pull/320) Added the `Ellipse` primitive.

### Changed

- **(breaking)** [#274](https://github.com/jamwaffles/embedded-graphics/pull/274) The `Circle` is now defined by its bounding box top-left corner and its diameter instead of its center and its radius. To convert your code, you can replace `Circle::new(point, radius)` by `Circle::with_center(point, 2 * radius + 1)`.
- **(breaking)** [#306](https://github.com/jamwaffles/embedded-graphics/pull/306) The `Rectangle` is now defined by its top-left corner and its size instead of the top-left and bottom-right corner. To convert your code, you can replace `Rectangle::new` by `Rectangle::with_corners`.
- **(breaking)** [#312](https://github.com/jamwaffles/embedded-graphics/pull/312) The methods in the `Dimension` trait are replaced by a single `bounding_box` method that returns a `Rectangle`.

### Fixed

- [#309](https://github.com/jamwaffles/embedded-graphics/pull/309) Prevent triangles with off-screen vertices from infinitely looping.
- [#317](https://github.com/jamwaffles/embedded-graphics/pull/317) The bounding box size for `Circle`s was off by one.

## [0.6.1] - 2020-04-01

### Added

- [#285](https://github.com/jamwaffles/embedded-graphics/pull/285) Add multiplication and division by a scalar for `Point` and `Size`.

### Fixed

- [#271](https://github.com/jamwaffles/embedded-graphics/pull/271) Styled `Line`s are now drawn using the `stroke_with` specified in the `PrimitiveStyle` and no longer default to 1px width.

## [0.6.0] - 2020-03-20

### Added

- [#269](https://github.com/jamwaffles/embedded-graphics/pull/269) Support for multiline text.
- [#267](https://github.com/jamwaffles/embedded-graphics/pull/267) Added a `6x6` hand-drawn variable width font.
- [#267](https://github.com/jamwaffles/embedded-graphics/pull/267) Support for variable width fonts.

### Fixed

- [#267](https://github.com/jamwaffles/embedded-graphics/pull/267) Fonts with CHARACTER_SPACING now properly render the background color between characters.
- [#276](https://github.com/jamwaffles/embedded-graphics/pull/276) Fixed Chars like '°' and similar for `Font12x16` & `Font24x32`

## [0.6.0-beta.2] - 2020-03-06

### Fixed

- [#262](https://github.com/jamwaffles/embedded-graphics/pull/262) `Triangle`s with a fill color but no stroke color are now rendered instead of not rendered at all.
- [#262](https://github.com/jamwaffles/embedded-graphics/pull/262) A triangle with a different stroke and fill color now renders its left-most border with the stroke color, instead of fill color
- [#262](https://github.com/jamwaffles/embedded-graphics/pull/262) Triangles and lines with a stroke width of 0 (and no fill for triangles) are no longer drawn.

### Added

- Added `draw_image` to `DrawTarget` trait with default implementation.

## [0.6.0-beta.1] - 2020-02-17

### Added

- [#257](https://github.com/jamwaffles/embedded-graphics/pull/257) added the `.into_storage()` method on `PixelColor`s via the `IntoStorage` trait to allow for more ergonomic conversion from a color to its raw storage type.

  ```diff
  // IntoStorage is included in the prelude.
  // You can also import it with use embedded_graphics::pixelcolor::IntoStorage.
  + use embedded_graphics::prelude::*;

  - RawU1::from(color).into_inner()
  + color.into_storage()
  ```

- Added as many `#[derive()]`s as possible to all embedded-graphics, tinybmp and tinytga types.
- Added `From<Point> for [i32; 2]`
- Added `From<Size> for [u32; 2]`
- Added the following fallible conversions to/from `Point`
  - `TryFrom<Point>` for `(u32, u32)`
  - `TryFrom<(u32, u32)>` for `Point`
  - `TryFrom<Point>` for `[u32; 2]`
  - `TryFrom<[u32; 2]>` for `Point`
  - `TryFrom<&[u32; 2]>` for `Point`
- #247 Added the `ImageData` trait. This trait can be implemented in an image format crate to add embedded-graphics support.
- #247 Added `Image` drawable. The `Image` drawable is used to make an implementation of `ImageData`, e.g. from `tinytga` or `tinybmp`, drawable to a `DrawTarget`, like a display.

### Changed

- **(breaking)** [#256](https://github.com/jamwaffles/embedded-graphics/pull/256) Make `PrimitiveStyleBuilder` and `TextStyleBuilder` consuming by following the [consuming builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html#consuming-builders:).
- **(breaking)** #247 The raw data `Image` struct is renamed to `ImageRaw`. The `Image` name is reused as a wrapper for other image types that implement the `ImageData` trait.
- **(breaking)** #247 `ImageLE` and `ImageBE` are renamed to `ImageRawLE` and `ImageRawBE` respectively.
- **(breaking)** #247 `ImageTga` and `ImageTga` are removed, along with their respective enabling features `bmp` and `tga`. To use BMP or TGA images, add `tinybmp` or `tinytga` to your dependencies with the `graphics` feature enabled. This allows usage of `Bmp` and `Tga` in embedded-graphics contexts. Tinybmp or tinytga usage now looks like this:

  `Cargo.toml`:

  ```toml
  [dependencies]
  embedded-graphics = "0.6.0"
  tinybmp = { version = "0.2.0", features = [ "graphics" ]}
  ```

  Your code:

  ```rust
  use embedded_graphics::{prelude::*, image::Image};
  use tinybmp::Bmp;

  let image = Bmp::new(include_bytes!("../../../assets/patch.bmp")).unwrap();
  let image = Image::new(&image);
  display.draw(&image);
  ```

- **(breaking)** #243 All methods on `DrawTarget` that handle drawing to/updating the display are now fallible and return `Result<(), Self::Error>` to allow for error handling in user code. This also affects `Drawable::draw()`, which now propagates any errors that occur while drawing.
- **(breaking)** #243 `DrawTarget::flush()` is removed. The display driver itself should provide this method if required.
- Expose `mock_display::MockDisplay` for use in tests, examples, etc in crates that pull embedded-graphics in as a dependency.

## 0.6.0-alpha.3

### Added

- #170 Added a `24x32` font based on the existing `12x16` font.

* #221 Implemented `Drawable` for `Pixel` to draw single pixels.

* #215 Added `PrimitiveStyleBuilder`.

* #215 Added `TextStyleBuilder`.

* #224 Added `Triangle::from_points` which accepts either `[Point; 3]`, or an array of length 3 containing any item that implements `Into<Point>`.

### Fixed

- #143, #209 Circles with no stroke are now drawn correctly

### Changed

- **(breaking)** The `Drawable` trait now has a required trait method `draw()`, which describes how the object will be drawn on the screen. See the docs for more details.

- **(breaking)** The `Drawing` trait has been renamed to `DrawTarget`. The required trait method to implement has changed from `draw()` to `draw_pixel()`, and optional trait methods have been added to allow an implementing display driver to specify hardware-accelerated methods for drawing graphics primitives.

- **(breaking)** #161 The `.fill()` and `.stroke()` style methods are renamed to `.fill_color()` and `.stroke_color()` respectively. This is to reduce confusion between names like `.stroke()` and `.stroke_width()`. Example:

  ```rust
  use embedded_graphics::{
    prelude::*,
    primitives::Circle,
    egcircle
  };

  let circle = Circle::new(Point::new(20, 20), 5)
      .stroke_width(10)
      .stroke_color(Some(BinaryColor::On))
      .fill_color(Some(BinaryColor::Off));
  ```

* **(breaking)** The type of `Style::stroke_width` changed from `u8` to `u32`.

* **(breaking)** Primitives shapes need to be converted into `Styled`s to be drawn.

* **(breaking)** The primitive macros like `egline` don't accept a `style` setting anymore. Use `object.style = new_style` instead.

* **(breaking)** The `Style` struct was replaced by `PrimitiveStyle` and `TextStyle`.

- **(breaking)** Text rendering is now implemented using `Text`, `TextStyle` and `Font` objects instead of using `Font::render_str`.

* **(breaking)** #224 Embedded Graphics macros now use named instead of positional parameters. Styling is now achieved using the `style` property which receives a `PrimitiveStyle` for primitives, and a `TextStyle`. There are convenience macros (`primitive_style!()` and `text_style!()` respectively) to make styling objects easier.

  ```rust
  use embedded_graphics::{egcircle, egline, egrectangle, egtext, egtriangle, text_style};

  // OLD
  egcircle!(
      (15, 20),
      10,
      stroke = Some(Rgb565::RED),
      fill = Some(Rgb565::GREEN)
  );
  // NEW
  egcircle!(
      center = (15, 20),
      radius = 10,
      style = primitive_style!(stroke = Rgb565::RED, fill = Rgb565::GREEN)
  );

  // OLD
  egrectangle!(
      (0, 0),
      (64, 64),
      stroke = Some(Rgb565::RED),
      fill = Some(Rgb565::GREEN)
  );
  // NEW
  egrectangle!(
      top_left = (0, 0),
      bottom_right = (64, 64),
      style = primitive_style!(stroke = Rgb565::RED, fill = Rgb565::GREEN)
  );

  // OLD
  egtriangle!((32, 0), (0, 64), (64, 64));
  // NEW
  egtriangle!(
      points = [(32, 0), (0, 64), (64, 64)],
      style = primitive_style!(stroke = Rgb565::RED, fill = Rgb565::GREEN)
  );

  // OLD
  egline!((32, 0), (0, 64));
  // NEW
  egline!(
      start = (32, 0),
      end = (0, 64),
      style = primitive_style!(stroke = Rgb565::RED,)
  );

  // OLD
  egtext!("456", (10, 10), font = Font6x8, stroke = Some(Rgb565::RED));
  // NEW
  egtext!(
      text = "456",
      top_left = (10, 10),
      style = text_style!(font = Font6x8, text_color = Rgb565::RED)
  );
  ```

### Removed

- **(breaking)** The `SizedDrawing` trait is removed.

## 0.6.0-alpha.2

### Changed

- **(breaking)** `Coord` and `UnsignedCoord` are replaced by [`Point`] and [`Size`].

- **(breaking)** The `Image` struct is removed from the prelude. Import it with `use embedded_graphics::image::Image` instead.

- **(breaking)** Integration with Nalgebra through the `nalgebra_support` feature is now achieved by converting Nalgebra types into `Point` or `Size` instead of Embedded Graphics aliasing [`Point`] and [`Size`] to [`nalgebra::Vector2<i32>`] and [`nalgebra::Vector2<u32>`] respectively. Integration now requires calling `Point::from(my_nalgebra_var)` or `my_nalgebra_var.into()`.

  The benefit of this change is to allow more integer primitive types in [`Vector2`]. Embedded Graphics should now support `u8`, `u16` and `u32` conversions to `Size`, and `u8`, `u16`, `i8`, `i16` and `i32` conversions to [`Point`]. It also reduces coupling between Nalgebra and Embedded Graphics.

- **(breaking)** `Point`s can no longer be created from `(u32, u32)`, `[u32; 2]` or `&[u32; 2]`; these conversions are dangerous as the full range of `u32` values cannot be represented by the `i32` used for storage inside [`Point`].

* **(breaking)** `Pixel` now uses the signed [`Point`] type as the first element. Display drivers need to implement an additional check if `x` and `y` are greater or equal to zero.

* **(breaking)** The image module has been rewritten to support big- and little-endian image formats. [`Image1BPP`], [`Image8BPP`] and [`Image16BPP`] are no longer available, and have been replaced with the single [`Image`] type. To migrate from the previous image types, use [`Image`] with a specified pixel color, like this:

  ```rust
  use embedded_graphics::{
    image::Image,
    pixelcolor::{BinaryColor, Gray8, Rgb565}
  };

  // Image1BPP
  let image: Image<BinaryColor> = Image::new(DATA, 12, 5);

  // Image8BPP
  let image: Image<Gray8> = Image::new(DATA, 12, 5);

  // Image16BPP
  let image: Image<Rgb565> = Image::new(DATA, 12, 5);
  ```

  There are other pixel color types available. Take a look at the [`pixelcolor`] module for a full list.

  If you need to specify an endianness for the image data (like when using multiple bytes per pixel), the [`ImageLE`] and [`ImageBE`] type aliases have been added.

### Removed

- **(breaking)** `Coord::clamp_positive` is removed.

- **(breaking)** The `icoord!()` and `ucoord!()` macros are removed. Use [`Point::new()`] or [`Size::new()`] respectively instead.

### Fixed

- The code examples `README.md` are now checked in CI during crate compilation. They were woefully outdated and have now been fixed.

[`point`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/geometry/struct.Point.html
[`point::new()`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/geometry/struct.Point.html#method.new
[`size`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/geometry/struct.Size.html
[`size::new()`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/geometry/struct.Size.html#method.new
[`pixelcolor`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/pixelcolor/index.html
[`image1bpp`]: https://docs.rs/embedded-graphics/0.5.1/embedded_graphics/image/type.Image1BPP.html
[`image8bpp`]: https://docs.rs/embedded-graphics/0.5.1/embedded_graphics/image/type.Image8BPP.html
[`image16bpp`]: https://docs.rs/embedded-graphics/0.5.1/embedded_graphics/image/type.Image16BPP.html
[`image`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/image/struct.Image.html
[`imagele`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/image/type.ImageLE.html
[`imagebe`]: https://docs.rs/embedded-graphics/0.6.0-alpha.2/embedded_graphics/image/type.ImageBE.html
[`nalgebra::vector2<i32>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
[`nalgebra::vector2<u32>`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html
[`vector2`]: https://docs.rs/nalgebra/0.18.0/nalgebra/base/type.Vector2.html

## 0.6.0-alpha.1

Major breaking changes ahead! @rfuest has been hard at work making the colours story in Embedded Graphics much richer and easier to use.

As this is an alpha version, please give it a test and report back any issues you find!

### Changed

- **(breaking)** Added many colour types

## 0.5.2

Small doc fixes and other minor changes.

### Added

- Added low-effort Embedded Graphics logo for <https://docs.rs/embedded-graphics>

### Fixed

- Wrap `Coord` code example in backticks so it's rendered as code by Rustdoc

## 0.5.1

A couple of breaking changes around naming, mostly polish around public APIs

### Added

### Changed

- **(breaking)** Primitives macros have been renamed. This is primarily to fix conflicts with `std`'s `line!()` macro, but I thought I'd take the opportunity to make the names a bit better/more consistent at the same time:
  - `line` -> `egline`
  - `triangle` -> `egtriangle`
  - `rect` -> `egrectangle`
  - `circle` -> `egcircle`
- **(breaking)** The `Rect` primitive is now renamed to `Rectangle` to fit with the other non-truncated primitive names.

## 0.5.0

A big release, focussed on ergonomics. There are new macros to make drawing and positioning primitives and text much less noisy, as well as changes to the `Drawing` trait to remove the explicit `.into_iter()` call when passing objects to it.

### Added

- Add `SizedDrawing` trait. This is useful for displays that support partial screen updates. If the passed object has known dimensions (via the `Dimensions`) trait, a smaller draw area can be specified, reducing the number of bytes sent over the wire. This also opens up the possibility of bufferless display drivers!
- Macros for primitives, text, `UnsignedCoord` and `Coord`! This should make graphics-heavy code much quicker to write, and much cleaner to read. For example, to create a line and a circle:

  Code that looked like this:

  ```rust
  display.draw(
      Circle::new(Coord::new(10, 10), 10 as u32)
          .with_stroke(Some(0u8))
          .with_fill(Some(1u8))
          .into_iter(),
  );
  display.draw(
      Circle::new(Coord::new(10, 10), 10 as u32)
          .translate(Coord::new(10, 10))
          .with_stroke(Some(0u8))
          .with_fill(Some(0u8))
          .into_iter(),
  );
  display.draw(
      Rect::new(Coord::new(0, 0), Coord::new(64, 64))
          .translate(Coord::new(96 + 32, 32))
          .with_stroke(Some(0u8))
          .with_fill(Some(0u8))
          .into_iter(),
  );
  display.draw(
      Triangle::new(Coord::new(32, 0), Coord::new(0, 64), Coord::new(64, 64))
          .translate(Coord::new(96 * 2 + 16, 16))
          .with_stroke(Some(0u8))
          .with_fill(Some(1u8))
          .into_iter(),
  );
  ```

  Now looks like this:

  ```rust
  display.draw(circle!((10, 10), 10 as u32, stroke = Some(0u8), fill = Some(1u8)));
  display.draw(
      circle!((10, 10), 10 as u32, stroke = Some(0u8), fill = Some(0u8))
          .translate(icoord!(10, 10)),
  );
  display.draw(
      rect!((0, 0), (64, 64), stroke = Some(0u8), fill = Some(1u8))
          .translate(icoord!(96 + 16, 16)),
  );
  display.draw(
      triangle!(
          (32, 0),
          (0, 64),
          (64, 64),
          stroke = Some(0u8),
          fill = Some(1u8)
      )
      .translate(icoord!(96 * 2 + 16, 16)),
  );
  ```

- Added `pixelcolor::RGB565` to make working with displays and images in the common [RGB565](http://www.barth-dev.de/online/rgb565-color-picker/) pixel format.

### Changed

- `Drawing#draw` now accepts `IntoIterator` instead of `Iter`.

  **This is a breaking change for driver implementors. Client code should still be fine, as `.into_iter()` can still be called.**

  This allows passing of embedded_graphics objects without having to explicitly call `.into_iter`:

  ```rust
  // Before (still works)
  display.draw(
      Circle::new(Coord::new(10, 10), 10 as u32)
        .into_iter()
  );

  // After
  display.draw(
      Circle::new(Coord::new(10, 10), 10 as u32)
  );
  ```

  This also means that objects can be passed by reference too:

  ```rust
  let circle = Circle::new(Coord::new(10, 10), 10 as u32);

  display.draw(&circle);

  // Reuse `circle` here
  ```

- **(breaking)** All `with_<prop>()` style methods are replaced by their unprefixed `<prop>()` counterparts - #106
  - `with_style()` -> `style()`
  - `with_stroke()` -> `stroke()`
  - `with_stroke_width()` -> `stroke_width()`
  - `with_fill()` -> `fill()`
- **(breaking)** `ImageBMP` and `ImageTGA` are now disabled by default behind Cargo features
  - Get `ImageBMP` by adding the `bmp` feature to your `Cargo.toml`
  - Get `ImageTGA` by adding the `tga` feature to your `Cargo.toml`
- **(breaking)** fonts now render with a transparent background by default. To get the old behaviour back, add a `fill` like this:

  ```rust
  // Without macros
  Font6x8::render_str("Hello Rust!").fill(Some(1u8.into()));

  // With macros
  text_6x8!("Hello Rust!", fill = Some(1u8.into()));
  ```

- Added a bunch of examples and docs. I hope it makes the crate easier to use! Please open an issue if anything is missing or hard to understand.
- `From` is implemented for a few more types for `Coord` and `UnsignedCoord`. Among other things, they can now be converted to tuples by calling `.into()`.

### Removed

- **(breaking)** `PixelColorU*` types. Use vanilla `u8`, `u16` or `u32` instead.
  - `PixelColorU8` -> `u8`
  - `PixelColorU16` -> `u16`
  - `PixelColorU32` -> `u32`
- **(breaking)** The deprecated `.dimensions()` method for fonts is replaced by the `.size()` method from the `WithStyle` trait. This makes fonts consistent with other embedded-graphics objects

### Fixed

- Circles with no stroke but `Some(...)` fill are now rendered instead of skipped.
- Embedded graphics objects can now be returned from functions, chained or not. For example:

  ```rust
  fn multi() -> impl Iterator<Item = Pixel<u8>> {
      let line = Line::new(Coord::new(0, 1), Coord::new(2, 3))
          .stroke(Some(1u8.into()));

      let circle = Circle::new(Coord::new(5, 5), 3)
          .stroke(Some(1u8.into()));

      line.into_iter().chain(circle)
  }
  ```

<!-- next-url -->

[unreleased]: https://github.com/jamwaffles/embedded-graphics/compare/embedded-graphics-v0.6.1...HEAD
[0.6.1]: https://github.com/jamwaffles/embedded-graphics/compare/embedded-graphics-v0.6.0...embedded-graphics-v0.6.1
[0.6.0]: https://github.com/jamwaffles/embedded-graphics/compare/embedded-graphics-v0.6.0-beta.2...embedded-graphics-v0.6.0
[0.6.0-beta.2]: https://github.com/jamwaffles/embedded-graphics/compare/embedded-graphics-v0.6.0-beta.1...embedded-graphics-v0.6.0-beta.2
[0.6.0-beta.1]: https://github.com/jamwaffles/embedded-graphics/compare/embedded-graphics-v0.6.0-alpha.3...embedded-graphics-v0.6.0-beta.1
