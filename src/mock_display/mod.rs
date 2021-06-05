//! Mock display for use in tests.
//!
//! [`MockDisplay`] can be used to replace a real display in tests. The internal
//! framebuffer wraps the color values in `Option` to be able to test which
//! pixels were modified by drawing operations.
//!
//! The [`from_pattern`] method provides a convenient way of creating expected
//! test results. The same patterns are used by the implementation of `Debug`
//! and will be shown in failing tests.
//!
//! The display is internally capped at 64x64px.
//!
//! # Assertions
//!
//! [`MockDisplay`] provides the [`assert_eq`] and [`assert_pattern`] methods to check if the
//! display is in the correct state after some drawing operations were executed. It's recommended
//! to use these methods instead of the standard `assert_eq!` macro, because they provide an
//! optional improved debug output for failing tests. If the `EG_FANCY_PANIC` environment variable
//! is set to `1` at compile time a graphic representation of the display content and a diff of the
//! display and the expected output will be shown:
//!
//! ```bash
//! EG_FANCY_PANIC=1 cargo test
//! ```
//!
//! Enabling the advanced test output requires a terminal that supports 24 BPP colors and a font
//! that includes the upper half block character `'\u{2580}'`.
//!
//! The color code used to show the difference between the display and the expected output is shown
//! in the documentation of the [`diff`] method.
//!
//! # Additional out of bounds and overdraw checks
//!
//! [`MockDisplay`] implements additional checks during drawing operations that will cause a panic if
//! any pixel is drawn outside the framebuffer or if a pixel is drawn more than once. These
//! stricter checks were added to help with testing and shouldn't be implemented by normal
//! [`DrawTarget`]s.
//!
//! If a test relies on out of bounds drawing or overdrawing the additional checks can explicitly
//! be disabled  by using [`set_allow_out_of_bounds_drawing`] and [`set_allow_overdraw`].
//!
//! # Characters used in `BinaryColor` patterns
//!
//! The following mappings are available for [`BinaryColor`]:
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'.'`     | `Some(BinaryColor::Off)` | Pixel was changed to `BinaryColor::Off` |
//! | `'#'`     | `Some(BinaryColor::On)`  | Pixel was changed to `BinaryColor::On`  |
//!
//! # Characters used in [`Gray2`] patterns
//!
//! The following mappings are available for [`Gray2`]:
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'0'`     | `Some(Gray2::new(0x0))`  | Pixel was changed to `Gray2::new(0x0)`  |
//! | `'1'`     | `Some(Gray2::new(0x1))`  | Pixel was changed to `Gray2::new(0x1)`  |
//! | `'2'`     | `Some(Gray2::new(0x2))`  | Pixel was changed to `Gray2::new(0x2)`  |
//! | `'3'`     | `Some(Gray2::new(0x3))`  | Pixel was changed to `Gray2::new(0x3)`  |
//!
//! # Characters used in [`Gray4`] patterns
//!
//! The following mappings are available for [`Gray4`]:
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'0'`     | `Some(Gray4::new(0x0))`  | Pixel was changed to `Gray4::new(0x0)`  |
//! | `'1'`     | `Some(Gray4::new(0x1))`  | Pixel was changed to `Gray4::new(0x1)`  |
//! | ⋮         | ⋮                        | ⋮                                      |
//! | `'E'`     | `Some(Gray4::new(0xE))`  | Pixel was changed to `Gray4::new(0xE)`  |
//! | `'F'`     | `Some(Gray4::new(0xF))`  | Pixel was changed to `Gray4::new(0xF)`  |
//!
//! # Characters used in [`Gray8`] patterns
//!
//! The following mappings are available for [`Gray8`]:
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'0'`     | `Some(Gray8::new(0x00))` | Pixel was changed to `Gray8::new(0x00)` |
//! | `'1'`     | `Some(Gray8::new(0x11))` | Pixel was changed to `Gray8::new(0x11)` |
//! | ⋮         | ⋮                        | ⋮                                      |
//! | `'E'`     | `Some(Gray8::new(0xEE))` | Pixel was changed to `Gray8::new(0xEE)` |
//! | `'F'`     | `Some(Gray8::new(0xFF))` | Pixel was changed to `Gray8::new(0xFF)` |
//!
//! Note: `Gray8` uses a different mapping than `Gray2` and `Gray4`, by duplicating the pattern
//! value into the high and low nibble. This allows using a single digit to test luma values ranging
//! from 0 to 255.
//!
//! # Characters used in RGB color patterns
//!
//! The following mappings are available for all RGB color types in the [`pixelcolor`] module,
//! like [`Rgb565`] or [`Rgb888`]:
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'K'`     | `Some(C::BLACK)`         | Pixel was changed to `C::BLACK`         |
//! | `'R'`     | `Some(C::RED)`           | Pixel was changed to `C::RED`           |
//! | `'G'`     | `Some(C::GREEN)`         | Pixel was changed to `C::GREEN`         |
//! | `'B'`     | `Some(C::BLUE)`          | Pixel was changed to `C::BLUE`          |
//! | `'Y'`     | `Some(C::YELLOW)`        | Pixel was changed to `C::YELLOW`        |
//! | `'M'`     | `Some(C::MAGENTA)`       | Pixel was changed to `C::MAGENTA`       |
//! | `'C'`     | `Some(C::CYAN)`          | Pixel was changed to `C::CYAN`          |
//! | `'W'`     | `Some(C::WHITE)`         | Pixel was changed to `C::WHITE`         |
//!
//! Note: The table used `C` as a placeholder for the actual color type, like `Rgb565::BLACK`.
//!
//! # Examples
//!
//! ## Assert that a modified display matches the expected value
//!
//! This example sets three pixels on the display and checks that they're turned on.
//!
//! ```rust
//! use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor, prelude::*};
//!
//! let mut display = MockDisplay::new();
//!
//! Pixel(Point::new(0, 0), BinaryColor::On).draw(&mut display);
//! Pixel(Point::new(2, 1), BinaryColor::On).draw(&mut display);
//! Pixel(Point::new(1, 2), BinaryColor::On).draw(&mut display);
//!
//! display.assert_pattern(&[
//!     "#  ", //
//!     "  #", //
//!     " # ", //
//! ]);
//! ```
//!
//! ## Load and validate a 16BPP image
//!
//! This example loads the following test image (scaled 10x to make it visible) and tests the
//! returned pixels against an expected pattern.
//!
//! ![Test image, scaled 1000%](data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAACgAAAAUCAIAAABwJOjsAAAAMUlEQVRIx2NkwAv+45Vl/E++XiaGAQKjFo9aPGrx0LeYhVAJMxrUoxaPWjxq8aCzGAAVwwQnmlfSgwAAAABJRU5ErkJggg==)
//!
//! ```rust
//! use embedded_graphics::{
//!     image::{Image, ImageRaw, ImageRawBE},
//!     mock_display::MockDisplay,
//!     pixelcolor::{Rgb565, RgbColor},
//!     prelude::*,
//! };
//!
//! let data = [
//!     0x00, 0x00, 0xF8, 0x00, 0x07, 0xE0, 0xFF, 0xE0, //
//!     0x00, 0x1F, 0x07, 0xFF, 0xF8, 0x1F, 0xFF, 0xFF, //
//! ];
//!
//! let raw: ImageRawBE<Rgb565> = ImageRaw::new(&data, 4);
//!
//! let image = Image::new(&raw, Point::zero());
//!
//! let mut display: MockDisplay<Rgb565> = MockDisplay::new();
//!
//! image.draw(&mut display);
//!
//! display.assert_pattern(&[
//!     "KRGY", //
//!     "BCMW", //
//! ]);
//! ```
//!
//! [`pixelcolor`]: ../pixelcolor/index.html#structs
//! [`BinaryColor`]: ../pixelcolor/enum.BinaryColor.html
//! [`Gray2`]: ../pixelcolor/struct.Gray2.html
//! [`Gray4`]: ../pixelcolor/struct.Gray4.html
//! [`Gray8`]: ../pixelcolor/struct.Gray8.html
//! [`Rgb565`]: ../pixelcolor/struct.Rgb565.html
//! [`Rgb888`]: ../pixelcolor/struct.Rgb888.html
//! [`DrawTarget`]: ../draw_target/trait.DrawTarget.html
//! [`MockDisplay`]: struct.MockDisplay.html
//! [`assert_eq`]: struct.MockDisplay.html#method.assert_eq
//! [`assert_pattern`]: struct.MockDisplay.html#method.assert_pattern
//! [`diff`]: struct.MockDisplay.html#method.diff
//! [`from_pattern`]: struct.MockDisplay.html#method.from_pattern
//! [`set_allow_overdraw`]: struct.MockDisplay.html#method.set_allow_overdraw
//! [`set_allow_out_of_bounds_drawing`]: struct.MockDisplay.html#method.set_allow_out_of_bounds_drawing

mod color_mapping;
mod fancy_panic;

use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    pixelcolor::{PixelColor, Rgb888, RgbColor},
    primitives::{PointsIter, Rectangle},
    Pixel,
};
pub use color_mapping::ColorMapping;
use core::{
    fmt::{self, Write},
    iter,
};
use fancy_panic::FancyPanic;

const SIZE: usize = 64;
const DISPLAY_AREA: Rectangle = Rectangle::new(Point::zero(), Size::new_equal(SIZE as u32));

/// Mock display struct
///
/// See the [module documentation](./index.html) for usage and examples.
#[derive(Clone)]
pub struct MockDisplay<C>
where
    C: PixelColor,
{
    pixels: [Option<C>; SIZE * SIZE],
    allow_overdraw: bool,
    allow_out_of_bounds_drawing: bool,
}

impl<C> MockDisplay<C>
where
    C: PixelColor,
{
    /// Creates a new empty mock display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a mock display from an iterator of [`Point`]s.
    ///
    /// This method can be used to create a mock display from the iterator produced by the
    /// [`PointsIter::points`] method.
    ///
    /// # Panics
    ///
    /// This method will panic if the iterator returns a point that is outside the display bounding
    /// box.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_graphics::{prelude::*, pixelcolor::BinaryColor, primitives::Circle, mock_display::MockDisplay};
    ///
    /// let circle = Circle::new(Point::new(0, 0), 4);
    ///
    /// let mut display = MockDisplay::from_points(circle.points(), BinaryColor::On);
    ///
    /// display.assert_pattern(&[
    ///     " ## ",
    ///     "####",
    ///     "####",
    ///     " ## ",
    /// ]);
    /// ```
    ///
    /// [`Point`]: ../geometry/struct.Point.html
    /// [`PointsIter::points`]: ../primitives/trait.PointsIter.html#tymethod.points
    /// [`map`]: #method.map
    /// [`BinaryColor`]: ../pixelcolor/enum.BinaryColor.html
    pub fn from_points<I>(points: I, color: C) -> Self
    where
        I: IntoIterator<Item = Point>,
    {
        let mut display = Self::new();
        display.set_pixels(points, Some(color));

        display
    }

    /// Sets if out of bounds drawing is allowed.
    ///
    /// If this is set to `true` the bounds checks during drawing are disabled.
    pub fn set_allow_out_of_bounds_drawing(&mut self, value: bool) {
        self.allow_out_of_bounds_drawing = value;
    }

    /// Sets if overdrawing is allowed.
    ///
    /// If this is set to `true` the overdrawing is allowed.
    pub fn set_allow_overdraw(&mut self, value: bool) {
        self.allow_overdraw = value;
    }

    /// Returns the color of a pixel.
    pub fn get_pixel(&self, p: Point) -> Option<C> {
        let Point { x, y } = p;

        self.pixels[x as usize + y as usize * SIZE]
    }

    /// Changes the value of a pixel without bounds checking.
    ///
    /// # Panics
    ///
    /// This method will panic if `point` is outside the display bounding box.
    pub fn set_pixel(&mut self, point: Point, color: Option<C>) {
        assert!(
            point.x >= 0 && point.y >= 0 && point.x < SIZE as i32 && point.y < SIZE as i32,
            "point must be inside display bounding box: {:?}",
            point
        );

        let i = point.x + point.y * SIZE as i32;
        self.pixels[i as usize] = color;
    }

    /// Changes the value of a pixel without bounds checking.
    ///
    /// # Panics
    ///
    /// This method will panic if `point` is outside the display bounding box.
    fn set_pixel_unchecked(&mut self, point: Point, color: Option<C>) {
        let i = point.x + point.y * SIZE as i32;
        self.pixels[i as usize] = color;
    }

    /// Sets the points in an iterator to the given color.
    ///
    /// # Panics
    ///
    /// This method will panic if the iterator returns points outside the display bounding box.
    pub fn set_pixels(&mut self, points: impl IntoIterator<Item = Point>, color: Option<C>) {
        for point in points {
            self.set_pixel(point, color);
        }
    }

    /// Returns the area that was affected by drawing operations.
    pub fn affected_area(&self) -> Rectangle {
        let (tl, br) = self
            .bounding_box()
            .points()
            .zip(self.pixels.iter())
            .filter_map(|(point, color)| color.map(|_| point))
            .fold(
                (None, None),
                |(tl, br): (Option<Point>, Option<Point>), point| {
                    (
                        tl.map(|tl| tl.component_min(point)).or(Some(point)),
                        br.map(|br| br.component_max(point)).or(Some(point)),
                    )
                },
            );

        if let (Some(tl), Some(br)) = (tl, br) {
            Rectangle::with_corners(tl, br)
        } else {
            Rectangle::zero()
        }
    }

    /// Returns the `affected_area` with the top left corner extended to `(0, 0)`.
    fn affected_area_origin(&self) -> Rectangle {
        self.affected_area()
            .bottom_right()
            .map(|bottom_right| Rectangle::with_corners(Point::zero(), bottom_right))
            .unwrap_or_default()
    }

    /// Changes the color of a pixel.
    ///
    /// # Panics
    ///
    /// If out of bounds draw checking is enabled (default), this method will panic if the point
    /// lies outside the display area. This behavior can be disabled by calling
    /// [`set_allow_out_of_bounds_drawing(true)`].
    ///
    /// Similarly, overdraw is checked by default and will panic if a point is drawn to the same
    /// coordinate twice. This behavior can be disabled by calling [`set_allow_overdraw(true)`].
    ///
    /// [`set_allow_out_of_bounds_drawing(true)`]: #method.set_allow_out_of_bounds_drawing
    /// [`set_allow_overdraw(true)`]: #method.set_allow_overdraw
    pub fn draw_pixel(&mut self, point: Point, color: C) {
        if !DISPLAY_AREA.contains(point) {
            if !self.allow_out_of_bounds_drawing {
                panic!(
                    "tried to draw pixel outside the display area (x: {}, y: {})",
                    point.x, point.y
                );
            } else {
                return;
            }
        }

        if !self.allow_overdraw && self.get_pixel(point).is_some() {
            panic!("tried to draw pixel twice (x: {}, y: {})", point.x, point.y);
        }

        self.set_pixel_unchecked(point, Some(color));
    }

    /// Returns a copy of with the content mirrored by swapping x and y.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor};
    ///
    /// let display: MockDisplay<BinaryColor> = MockDisplay::from_pattern(&[
    ///     "#### ####",
    ///     "#    #   ",
    ///     "###  # ##",
    ///     "#    #  #",
    ///     "#### ####",
    /// ]);
    ///
    /// let mirrored = display.swap_xy();
    /// mirrored.assert_pattern(&[
    ///     "#####",
    ///     "# # #",
    ///     "# # #",
    ///     "#   #",
    ///     "     ",
    ///     "#####",
    ///     "#   #",
    ///     "# # #",
    ///     "# ###",
    /// ]);
    /// ```
    pub fn swap_xy(&self) -> MockDisplay<C> {
        let mut mirrored = MockDisplay::new();

        for point in self.bounding_box().points() {
            mirrored.set_pixel_unchecked(point, self.get_pixel(Point::new(point.y, point.x)));
        }

        mirrored
    }

    /// Maps a `MockDisplay<C>' to a `MockDisplay<CT>` by applying a function
    /// to each pixel.
    ///
    /// # Examples
    ///
    /// Invert a `MockDisplay` by applying [`BinaryColor::invert`] to the color of each pixel.
    ///
    /// [`BinaryColor::invert`]: ../pixelcolor/enum.BinaryColor.html#method.invert
    ///
    /// ```
    /// use embedded_graphics::{mock_display::MockDisplay, pixelcolor::BinaryColor};
    ///
    /// let display: MockDisplay<BinaryColor> = MockDisplay::from_pattern(&[
    ///     "####",
    ///     "#  .",
    ///     "....",
    /// ]);
    ///
    /// let inverted = display.map(|c| c.invert());
    /// inverted.assert_pattern(&[
    ///     "....",
    ///     ".  #",
    ///     "####",
    /// ]);
    /// ```
    pub fn map<CT, F>(&self, f: F) -> MockDisplay<CT>
    where
        CT: PixelColor,
        F: Fn(C) -> CT + Copy,
    {
        let mut target = MockDisplay::new();

        for point in self.bounding_box().points() {
            target.set_pixel_unchecked(point, self.get_pixel(point).map(f))
        }

        target
    }

    /// Compares the display to another display.
    ///
    /// The following color code is used to show the difference between the displays:
    ///
    /// | Color               | Description                                                   |
    /// |---------------------|---------------------------------------------------------------|
    /// | None                | The color of the pixel is equal in both displays.             |
    /// | Some(Rgb888::GREEN) | The pixel was only set in `self`                              |
    /// | Some(Rgb888::RED)   | The pixel was only set in `other`                             |
    /// | Some(Rgb888::BLUE)  | The pixel was set to a different colors in `self` and `other` |
    pub fn diff(&self, other: &MockDisplay<C>) -> MockDisplay<Rgb888> {
        let mut display = MockDisplay::new();

        for point in display.bounding_box().points() {
            let self_color = self.get_pixel(point);
            let other_color = other.get_pixel(point);

            let diff_color = match (self_color, other_color) {
                (Some(_), None) => Some(Rgb888::GREEN),
                (None, Some(_)) => Some(Rgb888::RED),
                (Some(s), Some(o)) if s != o => Some(Rgb888::BLUE),
                _ => None,
            };

            display.set_pixel_unchecked(point, diff_color);
        }

        display
    }
}

impl<C: PixelColor> PartialEq for MockDisplay<C> {
    fn eq(&self, other: &Self) -> bool {
        self.pixels.iter().eq(other.pixels.iter())
    }
}

impl<C> MockDisplay<C>
where
    C: PixelColor + ColorMapping,
{
    /// Creates a new mock display from a character pattern.
    ///
    /// The color pattern is specified by a slice of string slices. Each string
    /// slice represents a row of pixels and every character a single pixel.
    ///
    /// A space character in the pattern represents a pixel which wasn't
    /// modified by any drawing routine and is left in the default state.
    /// All other characters are converted by implementations of the
    /// [`ColorMapping`] trait.
    ///
    /// [`ColorMapping`]: trait.ColorMapping.html
    pub fn from_pattern(pattern: &[&str]) -> MockDisplay<C> {
        // Check pattern dimensions.
        let pattern_width = pattern.first().map_or(0, |row| row.len());
        let pattern_height = pattern.len();
        assert!(
            pattern_width <= SIZE,
            "Test pattern must not be wider than {} columns",
            SIZE
        );
        assert!(
            pattern_height <= SIZE,
            "Test pattern must not be taller than {} rows",
            SIZE
        );
        for (row_idx, row) in pattern.iter().enumerate() {
            assert_eq!(
                row.len(),
                pattern_width,
                "Row #{} is {} characters wide (must be {} characters to match previous rows)",
                row_idx + 1,
                row.len(),
                pattern_width
            );
        }

        // Convert pattern to colors and pad pattern with None.
        let pattern_colors = pattern
            .iter()
            .flat_map(|row| {
                row.chars()
                    .map(|c| match c {
                        ' ' => None,
                        _ => Some(C::char_to_color(c)),
                    })
                    .chain(iter::repeat(None))
                    .take(SIZE)
            })
            .chain(iter::repeat(None))
            .take(SIZE * SIZE);

        // Copy pattern to display.
        let mut display = MockDisplay::new();
        for (i, color) in pattern_colors.enumerate() {
            display.pixels[i] = color;
        }

        display
    }

    /// Checks if the displays are equal.
    ///
    /// An advanced output for failing tests can be enabled by setting the environment variable
    /// `EG_FANCY_PANIC=1`. See the [module-level documentation] for more details.
    ///
    /// # Panics
    ///
    /// Panics if the displays aren't equal.
    ///
    /// [module-level documentation]: index.html#assertions
    // MSRV: add track_caller attribute to get better error messages for rust >= 1.46.0
    // #[track_caller]
    pub fn assert_eq(&self, other: &MockDisplay<C>) {
        if !self.eq(other) {
            if option_env!("EG_FANCY_PANIC") == Some("1") {
                let fancy_panic = FancyPanic::new(self, other, 30);
                panic!("\n{}", fancy_panic);
            } else {
                panic!("\ndisplay\n{:?}\nexpected\n{:?}", self, other);
            }
        }
    }

    /// Checks if the displays are equal.
    ///
    /// An advanced output for failing tests can be enabled by setting the environment variable
    /// `EG_FANCY_PANIC=1`. See the [module-level documentation] for more details.
    ///
    /// The output of the `msg` function will be prepended to the output if the assertion fails.
    ///
    /// # Panics
    ///
    /// Panics if the displays aren't equal.
    ///
    /// [module-level documentation]: index.html#assertions
    // MSRV: add track_caller attribute to get better error messages for rust >= 1.46.0
    // #[track_caller]
    pub fn assert_eq_with_message<F>(&self, other: &MockDisplay<C>, msg: F)
    where
        F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        if !self.eq(other) {
            if option_env!("EG_FANCY_PANIC") == Some("1") {
                let fancy_panic = FancyPanic::new(self, other, 30);
                panic!("\n{}\n\n{}", MessageWrapper(msg), fancy_panic);
            } else {
                panic!(
                    "\n{}\n\ndisplay:\n{:?}\nexpected:\n{:?}",
                    MessageWrapper(msg),
                    self,
                    other
                );
            }
        }
    }

    /// Checks if the display is equal to the given pattern.
    ///
    /// An advanced output for failing tests can be enabled, see the [module-level documentation]
    /// for more details.
    ///
    /// # Panics
    ///
    /// Panics if the display content isn't equal to the pattern.
    ///
    /// [module-level documentation]: index.html#assertions
    // MSRV: add track_caller attribute to get better error messages for rust >= 1.46.0
    // #[track_caller]
    pub fn assert_pattern(&self, pattern: &[&str]) {
        let other = MockDisplay::<C>::from_pattern(pattern);

        self.assert_eq(&other);
    }

    /// Checks if the display is equal to the given pattern.
    ///
    /// An advanced output for failing tests can be enabled, see the [module-level documentation]
    /// for more details.
    ///
    /// The output of the `msg` function will be prepended to the output if the assertion fails.
    ///
    /// # Panics
    ///
    /// Panics if the display content isn't equal to the pattern.
    ///
    /// [module-level documentation]: index.html#assertions
    // MSRV: add track_caller attribute to get better error messages for rust >= 1.46.0
    // #[track_caller]
    pub fn assert_pattern_with_message<F>(&self, pattern: &[&str], msg: F)
    where
        F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
    {
        let other = MockDisplay::<C>::from_pattern(pattern);

        self.assert_eq_with_message(&other, msg);
    }
}

impl<C> Default for MockDisplay<C>
where
    C: PixelColor,
{
    fn default() -> Self {
        Self {
            pixels: [None; SIZE * SIZE],
            allow_overdraw: false,
            allow_out_of_bounds_drawing: false,
        }
    }
}

impl<C> fmt::Debug for MockDisplay<C>
where
    C: PixelColor + ColorMapping,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let empty_rows = self
            .pixels
            .rchunks(SIZE)
            .take_while(|row| row.iter().all(Option::is_none))
            .count();

        writeln!(f, "MockDisplay[")?;
        for row in self.pixels.chunks(SIZE).take(SIZE - empty_rows) {
            for color in row {
                f.write_char(color.map_or(' ', C::color_to_char))?;
            }
            writeln!(f)?;
        }
        if empty_rows > 0 {
            writeln!(f, "({} empty rows skipped)", empty_rows)?;
        }
        writeln!(f, "]")?;

        Ok(())
    }
}

impl<C> DrawTarget for MockDisplay<C>
where
    C: PixelColor,
{
    type Color = C;
    type Error = core::convert::Infallible;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        for pixel in pixels.into_iter() {
            let Pixel(point, color) = pixel;

            self.draw_pixel(point, color);
        }

        Ok(())
    }
}

impl<C> OriginDimensions for MockDisplay<C>
where
    C: PixelColor,
{
    fn size(&self) -> Size {
        DISPLAY_AREA.size
    }
}

/// Wrapper to implement `Display` for formatting function.
struct MessageWrapper<F>(F);

impl<F> fmt::Display for MessageWrapper<F>
where
    F: Fn(&mut fmt::Formatter<'_>) -> fmt::Result,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        pixelcolor::{BinaryColor, Rgb565},
        Drawable,
    };

    #[test]
    #[should_panic(expected = "tried to draw pixel outside the display area (x: 65, y: 0)")]
    fn panic_on_out_of_bounds_drawing() {
        let mut display = MockDisplay::new();

        Pixel(Point::new(65, 0), BinaryColor::On)
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    fn allow_out_of_bounds_drawing() {
        let mut display = MockDisplay::new();
        display.set_allow_out_of_bounds_drawing(true);

        Pixel(Point::new(65, 0), BinaryColor::On)
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "tried to draw pixel twice (x: 1, y: 2)")]
    fn panic_on_overdraw() {
        let mut display = MockDisplay::new();

        let p = Pixel(Point::new(1, 2), BinaryColor::On);
        p.draw(&mut display).unwrap();
        p.draw(&mut display).unwrap();
    }

    #[test]
    fn allow_overdraw() {
        let mut display = MockDisplay::new();
        display.set_allow_overdraw(true);

        let p = Pixel(Point::new(1, 2), BinaryColor::On);
        p.draw(&mut display).unwrap();
        p.draw(&mut display).unwrap();
    }

    #[test]
    fn zero_sized_affected_area() {
        let disp: MockDisplay<BinaryColor> = MockDisplay::new();
        assert!(disp.affected_area().is_zero_sized(),);
    }

    #[test]
    fn diff() {
        let display1 = MockDisplay::<Rgb565>::from_pattern(&[" R RR"]);
        let display2 = MockDisplay::<Rgb565>::from_pattern(&[" RR B"]);
        let expected = MockDisplay::<Rgb888>::from_pattern(&["  RGB"]);

        display1.diff(&display2).assert_eq(&expected);
    }
}
