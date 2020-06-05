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
//! ![TGA test image, scaled 1000%]( data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAFoAAAAyBAMAAAAuIdEGAAAAGFBMVEUAAAD/AAAA/wD//wAAAP//AP8A//////8V3DX3AAAACXBIWXMAAC4jAAAuIwF4pT92AAAAB3RJTUUH5AEODTI3wavPmgAAAEtJREFUSMdjKAcBBhAQBAElEHABAWMQCAWBNBCAqBtVPZhVI8Co6qGuejR9D8d8OQqGOkDEKJgFjmVwfINjHpwGwKkBnC5GVQ9m1QAE0rmq9y8gWgAAAABJRU5ErkJggg==)
//!
//! ```rust
//! use embedded_graphics::{
//!     image::Image,
//!     mock_display::MockDisplay,
//!     pixelcolor::{Rgb888, RgbColor},
//!     prelude::*,
//! };
//! use tinytga::Tga;
//!
//! let data = include_bytes!("../../tinytga/tests/type1_tl.tga");
//!
//! let tga = Tga::from_slice(data).unwrap();
//!
//! let image: Image<Tga, Rgb888> = Image::new(&tga, Point::zero());
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
    drawable::Pixel,
    geometry::{Point, Size},
    pixelcolor::{
        Bgr555, Bgr565, Bgr888, BinaryColor, Gray8, GrayColor, PixelColor, Rgb555, Rgb565, Rgb888,
        RgbColor,
    },
    prelude::Primitive,
    primitives::{ContainsPoint, Rectangle},
    DrawTarget,
};
use core::{
    cmp::PartialEq,
    fmt::{self, Write},
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

    /// Changes the color of a pixel.
    pub fn set_pixel(&mut self, point: Point, color: Option<C>) {
        let i = point.x + point.y * SIZE as i32;
        self.pixels[i as usize] = color;
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
            "Test pattern must not be wider than {}",
            SIZE
        );
        assert!(
            pattern_height <= SIZE,
            "Test pattern must not be taller than {}",
            SIZE
        );
        for row in pattern {
            assert_eq!(
                row.len(),
                pattern_width,
                "Every row in the pattern must be {} characters wide",
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

            if !DISPLAY_AREA.contains(point) {
                if self.allow_out_of_bounds_drawing {
                    continue;
                } else {
                    panic!(
                        "tried to draw pixel outside the display area (x: {}, y: {})",
                        point.x, point.y
                    );
                }
            }

            if !self.allow_overdraw && self.get_pixel(point).is_some() {
                panic!("tried to draw pixel twice (x: {}, y: {})", point.x, point.y);
            }

            self.set_pixel(point, Some(color));
        }

        Ok(())
    }

    fn size(&self) -> Size {
        DISPLAY_AREA.size
    }
}

/// Mapping between `char`s and colors.
///
/// See the [module-level documentation] for a table of implemented mappings.
///
/// [module-level documentation]: index.html
pub trait ColorMapping {
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

impl ColorMapping for Gray8 {
    fn char_to_color(c: char) -> Self {
        let digit = match c {
            '0'..='9' | 'A'..='F' => c.to_digit(16).unwrap(),
            _ => panic!("Invalid char in pattern: '{}'", c),
        };

        Gray8::new(digit as u8 * 0x11)
    }

    fn color_to_char(color: Gray8) -> char {
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
}
