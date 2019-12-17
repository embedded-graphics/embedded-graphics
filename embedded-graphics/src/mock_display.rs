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
//! [`MockDisplay`]: struct.MockDisplay.html
//! [`from_pattern`]: struct.MockDisplay.html#method.from_pattern
//!
//! # Characters used in `BinaryColor` patterns
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'.'`     | `Some(BinaryColor::Off)` | Pixel was changed to `BinaryColor::Off` |
//! | `'#'`     | `Some(BinaryColor::On)`  | Pixel was changed to `BinaryColor::On`  |
//!
//! # Characters used in `Gray8` patterns
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'0'`     | `Some(Gray8::new(0x00))` | Pixel was changed to `Gray8::new(0x00)` |
//! | `'1'`     | `Some(Gray8::new(0x11))` | Pixel was changed to `Gray8::new(0x11)` |
//! | `...`     |                          |                                         |
//! | `'E'`     | `Some(Gray8::new(0xEE))` | Pixel was changed to `Gray8::new(0xEE)` |
//! | `'F'`     | `Some(Gray8::new(0xFF))` | Pixel was changed to `Gray8::new(0xFF)` |
//!
//! # Characters used in `Rgb888` patterns
//!
//! | Character | Color                    | Description                             |
//! |-----------|--------------------------|-----------------------------------------|
//! | `' '`     | `None`                   | No drawing operation changed the pixel  |
//! | `'K'`     | `Some(Rgb888::BLACK)`    | Pixel was changed to `Rgb888::BLACK`    |
//! | `'R'`     | `Some(Rgb888::RED)`      | Pixel was changed to `Rgb888::RED`      |
//! | `'G'`     | `Some(Rgb888::GREEN)`    | Pixel was changed to `Rgb888::GREEN`    |
//! | `'B'`     | `Some(Rgb888::BLUE)`     | Pixel was changed to `Rgb888::BLUE`     |
//! | `'Y'`     | `Some(Rgb888::YELLOW)`   | Pixel was changed to `Rgb888::YELLOW`   |
//! | `'M'`     | `Some(Rgb888::MAGENTA)`  | Pixel was changed to `Rgb888::MAGENTA`  |
//! | `'C'`     | `Some(Rgb888::CYAN)`     | Pixel was changed to `Rgb888::CYAN`     |
//! | `'W'`     | `Some(Rgb888::WHITE)`    | Pixel was changed to `Rgb888::WHITE`    |

use crate::drawable::Pixel;
use crate::geometry::{Point, Size};
use crate::pixelcolor::{BinaryColor, Gray8, GrayColor, PixelColor, Rgb888, RgbColor};
use crate::DrawTarget;
use core::{
    cmp::PartialEq,
    fmt::{self, Write},
    iter,
};

const SIZE: usize = 64;

/// Mock display for use in tests and some doc examples. Do not use directly!
#[derive(Clone)]
pub struct MockDisplay<C>([Option<C>; SIZE * SIZE])
where
    C: PixelColor;

impl<C> MockDisplay<C>
where
    C: PixelColor,
{
    /// Creates a new empty mock display.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the width of the display.
    pub fn width(&self) -> usize {
        SIZE
    }

    /// Returns the height of the display.
    pub fn height(&self) -> usize {
        SIZE
    }

    /// Returns the color of a pixel.
    pub fn get_pixel(&self, p: Point) -> Option<C> {
        let Point { x, y } = p;

        self.0[x as usize + y as usize * SIZE]
    }

    /// Changes the color of a pixel.
    pub fn set_pixel(&mut self, p: Point, color: Option<C>) {
        let Point { x, y } = p;

        self.0[x as usize + y as usize * SIZE] = color;
    }
}

impl<C> MockDisplay<C>
where
    C: PixelColor + ColorMapping<C>,
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
        assert!(pattern_width <= SIZE);
        assert!(pattern_height <= SIZE);
        for row in pattern {
            assert_eq!(row.len(), pattern_width);
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
            display.0[i] = color;
        }

        display
    }
}

impl<C> Default for MockDisplay<C>
where
    C: PixelColor,
{
    fn default() -> Self {
        Self([None; SIZE * SIZE])
    }
}

impl<C> fmt::Debug for MockDisplay<C>
where
    C: PixelColor + ColorMapping<C>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let empty_rows = self
            .0
            .rchunks(SIZE)
            .take_while(|row| row.iter().all(Option::is_none))
            .count();

        writeln!(f, "MockDisplay[")?;
        for row in self.0.chunks(SIZE).take(SIZE - empty_rows) {
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
        self.0.iter().eq(other.0.iter())
    }
}

impl<C> DrawTarget<C> for MockDisplay<C>
where
    C: PixelColor,
{
    fn draw_pixel(&mut self, pixel: Pixel<C>) {
        let Pixel(Point { x, y }, color) = pixel;
        if !(0..SIZE).contains(&(x as usize)) || !(0..SIZE).contains(&(y as usize)) {
            return;
        }

        let i = x + y * SIZE as i32;
        self.0[i as usize] = Some(color);
    }

    fn size(&self) -> Size {
        Size::new(self.width() as u32, self.height() as u32)
    }
}

/// Mapping between `char`s and colors.
///
/// See the [module-level documentation] for a table of implemented mappings.
///
/// [module-level documentation]: index.html
pub trait ColorMapping<C> {
    /// Converts a char into a color of type `C`.
    fn char_to_color(c: char) -> C;

    /// Converts a color of type `C` into a char.
    fn color_to_char(color: C) -> char;
}

impl ColorMapping<BinaryColor> for BinaryColor {
    fn char_to_color(c: char) -> Self {
        match c {
            '.' => BinaryColor::Off,
            '#' => BinaryColor::On,
            _ => panic!("Invalid char in pattern: '{}'", c),
        }
    }

    fn color_to_char(color: BinaryColor) -> char {
        match color {
            BinaryColor::Off => '.',
            BinaryColor::On => '#',
        }
    }
}

impl ColorMapping<Gray8> for Gray8 {
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

impl ColorMapping<Rgb888> for Rgb888 {
    fn char_to_color(c: char) -> Self {
        match c {
            'K' => Rgb888::BLACK,
            'R' => Rgb888::RED,
            'G' => Rgb888::GREEN,
            'B' => Rgb888::BLUE,
            'Y' => Rgb888::YELLOW,
            'M' => Rgb888::MAGENTA,
            'C' => Rgb888::CYAN,
            'W' => Rgb888::WHITE,
            _ => panic!("Invalid char in pattern: '{}'", c),
        }
    }

    fn color_to_char(color: Rgb888) -> char {
        match color {
            Rgb888::BLACK => 'K',
            Rgb888::RED => 'R',
            Rgb888::GREEN => 'G',
            Rgb888::BLUE => 'B',
            Rgb888::YELLOW => 'Y',
            Rgb888::MAGENTA => 'M',
            Rgb888::CYAN => 'C',
            Rgb888::WHITE => 'W',
            _ => '?',
        }
    }
}
