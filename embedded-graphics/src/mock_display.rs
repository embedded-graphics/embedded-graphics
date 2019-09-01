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

use crate::drawable::Pixel;
use crate::pixelcolor::BinaryColor;
use crate::prelude::*;
use crate::Drawing;
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
    pub fn get_pixel(&self, p: UnsignedCoord) -> Option<C> {
        let x = p[0] as usize;
        let y = p[1] as usize;

        self.0[x + y * SIZE]
    }

    /// Changes the color of a pixel.
    pub fn set_pixel(&mut self, p: UnsignedCoord, color: Option<C>) {
        let x = p[0] as usize;
        let y = p[1] as usize;

        self.0[x + y * SIZE] = color;
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
            .into_iter()
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
            .take_while(|row| row.into_iter().all(Option::is_none))
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
        self.0.into_iter().eq(other.0.into_iter())
    }
}

impl<C> Drawing<C> for MockDisplay<C>
where
    C: PixelColor,
{
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: IntoIterator<Item = Pixel<C>>,
    {
        for Pixel(c, color) in item_pixels {
            let x = c[0] as usize;
            let y = c[1] as usize;

            if x >= SIZE || y >= SIZE {
                continue;
            }

            let i = x + y * SIZE;
            self.0[i] = Some(color);
        }
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
