# Changelog

Embedded Graphics is a `no_std` library for adding graphics features to display drivers. It aims to use the minimum amount of memory for builtin graphics objects by leveraging Rust's iterators to avoid large allocations. It targets embedded environments, but can run anywhere like a Raspberry Pi up to full desktop machines.

## 0.5.0

A big release, focussed on ergonomics. There are new macros to make drawing and positioning primitives and text much less noisy, as well as changes to the `Drawing` trait to remove the explicit `.into_iter()` call when passing objects to it.

### Added

* Add `SizedDrawing` trait. This is useful for displays that support partial screen updates. If the passed object has known dimensions (via the `Dimensions`) trait, a smaller draw area can be specified, reducing the number of bytes sent over the wire. This also opens up the possibility of bufferless display drivers!
* Macros for primitives, text, `UnsignedCoord` and `Coord`! This should make graphics-heavy code much quicker to write, and much cleaner to read. For example, to create a line and a circle:

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
* Added `pixelcolor::RGB565` to make working with displays and images in the common [RGB565](http://www.barth-dev.de/online/rgb565-color-picker/) pixel format.

### Changed

* `Drawing#draw` now accepts `IntoIterator` instead of `Iter`.

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
* **(breaking)** All `with_<prop>()` style methods are replaced by their unprefixed `<prop>()` counterparts - #106
  * `with_style()` -> `style()`
  * `with_stroke()` -> `stroke()`
  * `with_stroke_width()` -> `stroke_width()`
  * `with_fill()` -> `fill()`
* **(breaking)** `ImageBMP` and `ImageTGA` are now disabled by default behind Cargo features
  * Get `ImageBMP` by adding the `bmp` feature to your `Cargo.toml`
  * Get `ImageTGA` by adding the `tga` feature to your `Cargo.toml`
* **(breaking)** fonts now render with a transparent background by default. To get the old behaviour back, add a `fill` like this:

  ```rust
  // Without macros
  Font6x8::render_str("Hello Rust!").fill(Some(1u8.into()));

  // With macros
  text_6x8!("Hello Rust!", fill = Some(1u8.into()));
  ```
* Added a bunch of examples and docs. I hope it makes the crate easier to use! Please open an issue if anything is missing or hard to understand.
* The builtin simulator now supports colour pixel types, like `RGB565`.
* `From` is implemented for a few more types for `Coord` and `UnsignedCoord`. Among other things, they can now be converted to tuples by calling `.into()`.

### Deprecated

* None

### Removed

* **(breaking)** `PixelColorU*` types. Use vanilla `u8`, `u16` or `u32` instead.
  * `PixelColorU8` -> `u8`
  * `PixelColorU16` -> `u16`
  * `PixelColorU32` -> `u32`
* **(breaking)** The deprecated `.dimensions()` method for fonts is replaced by the `.size()` method from the `WithStyle` trait. This makes fonts consistent with other embedded-graphics objects

### Fixed

* Circles with no stroke but `Some(...)` fill are now rendered instead of skipped.
* Embedded graphics objects can now be returned from functions, chained or not. For example:

  ```rust
  fn multi() -> impl Iterator<Item = Pixel<u8>> {
      let line = Line::new(Coord::new(0, 1), Coord::new(2, 3))
          .stroke(Some(1u8.into()));

      let circle = Circle::new(Coord::new(5, 5), 3)
          .stroke(Some(1u8.into()));

      line.into_iter().chain(circle)
  }
  ```

### Security

* None
