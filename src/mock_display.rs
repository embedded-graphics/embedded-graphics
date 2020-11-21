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
//! #[rustfmt::skip]
//! assert_eq!(
//!     display,
//!     MockDisplay::from_pattern(&[
//!         "#  ",
//!         "  #",
//!         " # ",
//!     ])
//! );
//! ```
//!
//! ## Load and validate a 24BPP TGA image
//!
//! This example loads the following test image (scaled 10x to make it visible) and tests the
//! returned pixels against an expected pattern.
//!
//! The `graphics` feature of `tinytga` needs to be enabled in `Cargo.toml` to use the `Tga` object
//! with embedded-graphics.
//!
//! ![TGA test image, scaled 1000%]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFoAAAAyBAMAAAAuIdEGAAAAGFBMVEUAAAD/AAAA/wD//wAAAP//AP8A//////8V3DX3AAAACXBIWXMAAC4jAAAuIwF4pT92AAAAB3RJTUUH5AEODTI3wavPmgAAAEtJREFUSMdjKAcBBhAQBAElEHABAWMQCAWBNBCAqBtVPZhVI8Co6qGuejR9D8d8OQqGOkDEKJgFjmVwfINjHpwGwKkBnC5GVQ9m1QAE0rmq9y8gWgAAAABJRU5ErkJggg==)
//!
//! ```rust,ignore
//! use embedded_graphics::{
//!     image::Image,
//!     mock_display::MockDisplay,
//!     pixelcolor::{Rgb888, RgbColor},
//!     prelude::*,
//! };
//! use tinytga::Tga;
//!
//! let data = include_bytes!("../../tinytga/tests/type1_24bpp_tl.tga");
//!
//! let tga: Tga<Rgb888> = Tga::from_slice(data).unwrap();
//!
//! let image = Image::new(&tga, Point::zero());
//!
//! let mut display: MockDisplay<Rgb888> = MockDisplay::new();
//!
//! image.draw(&mut display);
//!
//! assert_eq!(
//!     display,
//!     MockDisplay::from_pattern(&[
//!         "WKRGBYMCW",
//!         "KKRGBYMCW",
//!         "WKRGBYMCW",
//!         "KKKKKKKKK",
//!         "WKWCMYBGR",
//!     ])
//! );
//! ```
//!
//! [`pixelcolor`]: ../pixelcolor/index.html#structs
//! [`BinaryColor`]: ../pixelcolor/enum.BinaryColor.html
//! [`Gray8`]: ../pixelcolor/struct.Gray8.html
//! [`Rgb565`]: ../pixelcolor/struct.Rgb565.html
//! [`Rgb888`]: ../pixelcolor/struct.Rgb888.html
//! [`DrawTarget`]: ../draw_target/trait.DrawTarget.html
//! [`MockDisplay`]: struct.MockDisplay.html
//! [`from_pattern`]: struct.MockDisplay.html#method.from_pattern
//! [`set_allow_overdraw`]: struct.MockDisplay.html#method.set_allow_overdraw
//! [`set_allow_out_of_bounds_drawing`]: struct.MockDisplay.html#method.set_allow_out_of_bounds_drawing

use crate::{
    draw_target::DrawTarget,
    drawable::Pixel,
    geometry::{Dimensions, OriginDimensions, Point, Size},
    pixelcolor::{
        Bgr555, Bgr565, Bgr888, BinaryColor, Gray2, Gray4, Gray8, GrayColor, PixelColor, Rgb555,
        Rgb565, Rgb888, RgbColor, WebColors,
    },
    prelude::Primitive,
    primitives::{ContainsPoint, Rectangle},
};
use core::{
    cmp::PartialEq,
    fmt::{self, Display, Write},
    iter,
};

const SIZE: usize = 64;
const DISPLAY_AREA: Rectangle = Rectangle::new(Point::zero(), Size::new_equal(SIZE as u32));

/// Mock display struct
///
/// See the [module documentation](./index.html) for usage and examples.
#[derive(Copy, Clone)]
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
    pub fn set_pixel(&mut self, point: Point, color: Option<C>) {
        let i = point.x + point.y * SIZE as i32;
        self.pixels[i as usize] = color;
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
            Rectangle::new(Point::zero(), Size::zero())
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
    /// lies outside the display area. This behaviour can be disabled by calling
    /// [`set_allow_out_of_bounds_drawing(true)`].
    ///
    /// Similarly, overdraw is checked by default and will panic if a point is drawn to the same
    /// coordinate twice. This behaviour can be disabled by calling [`set_allow_overdraw(true)`].
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

        self.set_pixel(point, Some(color));
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
    /// assert_eq!(
    ///     mirrored,
    ///     MockDisplay::from_pattern(&[
    ///         "#####",
    ///         "# # #",
    ///         "# # #",
    ///         "#   #",
    ///         "     ",
    ///         "#####",
    ///         "#   #",
    ///         "# # #",
    ///         "# ###",
    ///     ])
    /// );
    /// ```
    pub fn swap_xy(&self) -> MockDisplay<C> {
        let mut mirrored = MockDisplay::new();

        for point in Rectangle::new(Point::zero(), self.size()).points() {
            mirrored.set_pixel(point, self.get_pixel(Point::new(point.y, point.x)));
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
    /// assert_eq!(inverted, MockDisplay::from_pattern(&[
    ///     "....",
    ///     ".  #",
    ///     "####",
    /// ]));
    /// ```
    pub fn map<CT, F>(&self, f: F) -> MockDisplay<CT>
    where
        CT: PixelColor,
        F: Fn(C) -> CT + Copy,
    {
        let mut target = MockDisplay::new();

        for point in Rectangle::new(Point::zero(), self.size()).points() {
            target.set_pixel(point, self.get_pixel(point).map(f))
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

            display.set_pixel(point, diff_color);
        }

        display
    }
}

impl MockDisplay<BinaryColor> {
    /// Create a mock display from an iterator of [`Point`]s.
    ///
    /// This method can be used to create a mock display from the iterator produced by the
    /// [`Primitive::points`] method.
    ///
    /// The color type used in the returned display is [`BinaryColor`], which can be mapped to
    /// another color type using the [`map`] method.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use embedded_graphics::{prelude::*, primitives::Circle, mock_display::MockDisplay};
    ///
    /// let circle = Circle::new(Point::new(0, 0), 4);
    ///
    /// let mut display = MockDisplay::from_points(circle.points());
    ///
    /// assert_eq!(display, MockDisplay::from_pattern(&[
    ///     " ## ",
    ///     "####",
    ///     "####",
    ///     " ## ",
    /// ]));
    /// ```
    ///
    /// [`Point`]: ../geometry/struct.Point.html
    /// [`Primitive::points`]: ../primitives/trait.Primitive.html#tymethod.points
    /// [`map`]: #method.map
    /// [`BinaryColor`]: ../pixelcolor/enum.BinaryColor.html
    pub fn from_points<I>(points: I) -> Self
    where
        I: IntoIterator<Item = Point>,
    {
        let mut display = MockDisplay::new();

        for point in points.into_iter() {
            display.set_pixel(point, Some(BinaryColor::On));
        }

        display
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
    /// # Panics
    ///
    /// Panics if the displays aren't equal.
    // MSRV: add track_caller attribute to get better error messages for rust >= 1.46.0
    // #[track_caller]
    pub fn assert_eq(&self, other: &MockDisplay<C>) {
        if option_env!("EG_FANCY_PANIC") != Some("1") {
            assert_eq!(self, other);
            return;
        }

        if self != other {
            let fancy_panic = FancyPanic::new(self, other);
            panic!("{}", fancy_panic);
        }
    }

    /// Checks if the display is equal to the given pattern.
    ///
    /// # Panics
    ///
    /// Panics if the display content isn't equal to the pattern.
    // MSRV: add track_caller attribute to get better error messages for rust >= 1.46.0
    // #[track_caller]
    pub fn assert_pattern(&self, pattern: &[&str]) {
        let other = MockDisplay::<C>::from_pattern(pattern);

        self.assert_eq(&other);
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

impl<C> PartialEq for MockDisplay<C>
where
    C: PixelColor,
{
    fn eq(&self, other: &MockDisplay<C>) -> bool {
        self.pixels.iter().eq(other.pixels.iter())
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

struct FancyPanic<'a, C>
where
    C: PixelColor,
{
    display: &'a MockDisplay<C>,
    expected: &'a MockDisplay<C>,
}

impl<'a, C> FancyPanic<'a, C>
where
    C: PixelColor,
{
    fn new(display: &'a MockDisplay<C>, expected: &'a MockDisplay<C>) -> Self {
        Self { display, expected }
    }
}

fn write_display<C>(
    f: &mut fmt::Formatter<'_>,
    display: &MockDisplay<C>,
    bounding_box: &Rectangle,
) -> fmt::Result
where
    C: PixelColor + ColorMapping,
{
    for y in bounding_box.rows().step_by(2) {

        write_row(f, display, bounding_box, y, 0)?;
        f.write_char('\n')?
    }

    Ok(())
}

fn write_row<C>(
    f: &mut fmt::Formatter<'_>,
    display: &MockDisplay<C>,
    bounding_box: &Rectangle,
    y: i32,
    column_width: usize,
) -> fmt::Result
where
    C: PixelColor + ColorMapping,
{
    for x in bounding_box.columns() {
        let point_top = Point::new(x, y);
        let point_bottom = Point::new(x, y + 1);

        // Set foreground color.
        if bounding_box.contains(point_top) {
            let color = display
                .get_pixel(point_top)
                .map(|c| c.into())
                .unwrap_or(C::NONE_COLOR);

            write!(f, "\x1b[38;2;{};{};{}m", color.r(), color.g(), color.b())?;
        } else {
            write!(f, "\x1b[39m")?;
        };

        // Set background color.
        if bounding_box.contains(point_bottom) {
            let color = display
                .get_pixel(point_bottom)
                .map(|c| c.into())
                .unwrap_or(C::NONE_COLOR);

            write!(f, "\x1b[48;2;{};{};{}m", color.r(), color.g(), color.b())?;
        } else {
            write!(f, "\x1b[49m")?;
        };

        // Write "upper half block" character.
        f.write_char('\u{2580}')?;
    }

    // Reset colors.
    f.write_str("\x1b[0;m")?;

    // Pad output with spaces if column width is larger than the width of the bounding box.
    for _ in bounding_box.size.width as usize..column_width {
        f.write_char(' ')?
    }

    Ok(())
}

fn write_vertical_border(f: &mut fmt::Formatter<'_>, column_width: usize) -> fmt::Result {
    write!(
        f,
        "+-{:-<width$}-+-{:-<width$}-+-{:-<width$}-+\n",
        "",
        "",
        "",
        width = column_width
    )
}

fn write_header(f: &mut fmt::Formatter<'_>, column_width: usize) -> fmt::Result {
    write_vertical_border(f, column_width)?;
    write!(
        f,
        "| {:^width$} | {:^width$} | {:^width$} |\n",
        "display",
        "expected",
        "diff",
        width = column_width
    )?;
    write_vertical_border(f, column_width)
}

impl<C> Display for FancyPanic<'_, C>
where
    C: PixelColor + ColorMapping,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let diff = self.display.diff(self.expected);

        let bounding_box_display = self.display.affected_area_origin();
        let bounding_box_expected = self.expected.affected_area_origin();

        let bounding_box = Rectangle::new(
            Point::zero(),
            bounding_box_display
                .size
                .component_max(bounding_box_expected.size),
        );

        f.write_char('\n')?;

        // Output the 3 displays in columns if they are less than 30 pixels wide.
        if bounding_box.size.width <= 30 {
            // Set the width of the output columns to the width of the bounding box,
            // but at least 10 characters to ensure the column labels fit.
            let column_width = (bounding_box.size.width as usize).max(10);

            write_header(f, column_width)?;

            for y in bounding_box.rows().step_by(2) {

                f.write_str("| ")?;
                write_row(f, self.display, &bounding_box, y, column_width)?;
                f.write_str(" | ")?;
                write_row(f, self.expected, &bounding_box, y, column_width)?;
                f.write_str(" | ")?;
                write_row(f, &diff, &bounding_box, y, column_width)?;
                f.write_str(" |\n")?;
            }

            write_vertical_border(f, column_width)?;
        } else {
            let width = bounding_box.size.width as usize;

            write!(f, "display\n{:-<width$}\n", "", width = width)?;
            write_display(f, self.display, &bounding_box)?;

            write!(f, "\nexpected\n{:-<width$}\n", "", width = width)?;
            write_display(f, &self.expected, &bounding_box)?;

            write!(f, "\ndiff\n{:-<width$}\n", "", width = width)?;
            write_display(f, &diff, &bounding_box)?;
        }

        Ok(())
    }
}

/// Mapping between `char`s and colors.
///
/// See the [module-level documentation] for a table of implemented mappings.
///
/// [module-level documentation]: index.html
pub trait ColorMapping: Into<Rgb888> {
    /// Color used to display `None` values when `EG_FANCY_PANIC` is enabled.
    ///
    /// This color must be set to a color that isn't available in normal patterns to make it
    /// distinguishable in the output. For non grayscale colors the default value should be used.
    const NONE_COLOR: Rgb888 = Rgb888::new(128, 128, 128);

    /// Converts a char into a color of type `C`.
    fn char_to_color(c: char) -> Self;

    /// Converts a color of type `C` into a char.
    fn color_to_char(color: Self) -> char;
}

impl ColorMapping for BinaryColor {
    fn char_to_color(c: char) -> Self {
        match c {
            '.' => BinaryColor::Off,
            '#' => BinaryColor::On,
            _ => panic!("Invalid char in pattern: '{}'", c),
        }
    }

    fn color_to_char(color: Self) -> char {
        match color {
            BinaryColor::Off => '.',
            BinaryColor::On => '#',
        }
    }
}

macro_rules! impl_gray_color_mapping {
    ($type:ident, $radix:expr) => {
        impl ColorMapping for $type {
            const NONE_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;

            fn char_to_color(c: char) -> Self {
                if let Some(digit) = c.to_digit($radix) {
                    Self::new(digit as u8)
                } else {
                    panic!("invalid char in pattern: '{}'", c)
                }
            }

            fn color_to_char(color: Self) -> char {
                core::char::from_digit(color.luma() as u32, $radix)
                    .unwrap()
                    .to_ascii_uppercase()
            }
        }
    };
}

impl_gray_color_mapping!(Gray2, 4);
impl_gray_color_mapping!(Gray4, 16);

impl ColorMapping for Gray8 {
    const NONE_COLOR: Rgb888 = Rgb888::CSS_STEEL_BLUE;

    fn char_to_color(c: char) -> Self {
        if let Some(digit) = c.to_digit(16) {
            Self::new(digit as u8 * 0x11)
        } else {
            panic!("invalid char in pattern: '{}'", c);
        }
    }

    fn color_to_char(color: Self) -> char {
        let luma = color.luma();
        let lower = luma & 0xF;
        let upper = luma >> 4;

        if lower != upper {
            '?'
        } else {
            core::char::from_digit(lower as u32, 16)
                .unwrap()
                .to_ascii_uppercase()
        }
    }
}

macro_rules! impl_rgb_color_mapping {
    ($type:ident) => {
        impl ColorMapping for $type {
            fn char_to_color(c: char) -> Self {
                match c {
                    'K' => Self::BLACK,
                    'R' => Self::RED,
                    'G' => Self::GREEN,
                    'B' => Self::BLUE,
                    'Y' => Self::YELLOW,
                    'M' => Self::MAGENTA,
                    'C' => Self::CYAN,
                    'W' => Self::WHITE,
                    _ => panic!("Invalid char in pattern: '{}'", c),
                }
            }

            fn color_to_char(color: Self) -> char {
                match color {
                    Self::BLACK => 'K',
                    Self::RED => 'R',
                    Self::GREEN => 'G',
                    Self::BLUE => 'B',
                    Self::YELLOW => 'Y',
                    Self::MAGENTA => 'M',
                    Self::CYAN => 'C',
                    Self::WHITE => 'W',
                    _ => '?',
                }
            }
        }
    };
}

impl_rgb_color_mapping!(Rgb555);
impl_rgb_color_mapping!(Bgr555);
impl_rgb_color_mapping!(Rgb565);
impl_rgb_color_mapping!(Bgr565);
impl_rgb_color_mapping!(Rgb888);
impl_rgb_color_mapping!(Bgr888);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::drawable::Drawable;

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
    fn gray2_mapping() {
        for luma in 0..4 {
            let color = Gray2::new(luma);

            assert_eq!(color, Gray2::char_to_color(Gray2::color_to_char(color)));
        }
    }

    #[test]
    fn gray4_mapping() {
        for luma in 0..16 {
            let color = Gray4::new(luma);

            assert_eq!(color, Gray4::char_to_color(Gray4::color_to_char(color)));
        }
    }

    #[test]
    fn gray8_mapping() {
        for luma in 0..16 {
            let color = Gray8::new(luma * 0x11);

            assert_eq!(color, Gray8::char_to_color(Gray8::color_to_char(color)));
        }
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: '4'")]
    fn invalid_gray2_char_4() {
        Gray2::char_to_color('4');
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: 'A'")]
    fn invalid_gray2_char_a() {
        Gray2::char_to_color('A');
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: 'G'")]
    fn invalid_gray4_char_g() {
        Gray2::char_to_color('G');
    }

    #[test]
    #[should_panic(expected = "invalid char in pattern: 'G'")]
    fn invalid_gray8_char_g() {
        Gray8::char_to_color('G');
    }

    #[test]
    fn zero_sized_affected_area() {
        let disp: MockDisplay<BinaryColor> = MockDisplay::new();

        assert_eq!(
            disp.affected_area(),
            Rectangle::new(Point::zero(), Size::zero())
        );
    }

    #[test]
    fn diff() {
        let display1 = MockDisplay::<Rgb565>::from_pattern(&[" R RR"]);
        let display2 = MockDisplay::<Rgb565>::from_pattern(&[" RR B"]);
        let expected = MockDisplay::<Rgb888>::from_pattern(&["  RGB"]);

        assert_eq!(display1.diff(&display2), expected);
    }
}
