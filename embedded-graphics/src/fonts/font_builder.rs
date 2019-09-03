//! Common code used to define available monospace pixel fonts.
//!
//! See the [module level type definitions](../index.html#types) for a list of usable fonts.

use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::fonts::Font;
use crate::geometry::{Dimensions, Point, Size};
use crate::pixelcolor::{BinaryColor, PixelColor};
use crate::style::Style;
use crate::style::WithStyle;
use crate::transform::Transform;
use core::marker::PhantomData;

/// The configuration of the font
pub trait FontBuilderConf {
    /// Raw image containing the font
    const FONT_IMAGE: &'static [u8];
    /// `char` height of the font
    const CHAR_HEIGHT: u32;

    /// `char` width of the font
    const CHAR_WIDTH: u32;
    /// Font image width, must be divisible by `8` and `CHAR_WIDTH`.
    const FONT_IMAGE_WIDTH: u32 = 240;
    /// Returns the index in the font of the correponding `char`
    fn char_offset(_: char) -> u32;
}

/// The font builder
///
/// This is a helper struct to reduce code duplication when implementing fonts. View the [module
/// level type definitions](../index.html#types) for a list of usable fonts.
#[derive(Debug)]
pub struct FontBuilder<'a, C: PixelColor, Conf> {
    /// Top left corner of the text
    pub pos: Point,

    /// Text to draw
    text: &'a str,

    /// Style of the font
    style: Style<C>,

    _conf: PhantomData<Conf>,
}

impl<'a, C: PixelColor + Copy, Conf> Copy for FontBuilder<'a, C, Conf> {}
impl<'a, C: PixelColor + Clone, Conf> Clone for FontBuilder<'a, C, Conf> {
    fn clone(&self) -> Self {
        Self {
            pos: self.pos,
            text: self.text,
            style: self.style,
            _conf: Default::default(),
        }
    }
}

impl<'a, C, Conf> Dimensions for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
    Conf: FontBuilderConf,
{
    fn top_left(&self) -> Point {
        self.pos
    }

    fn bottom_right(&self) -> Point {
        self.top_left() + self.size()
    }

    /// Get the bounding box of a piece of text
    ///
    /// Currently does not handle newlines (but neither does the rasteriser). It will give `(0, 0)`
    /// if the string to render is empty.
    fn size(&self) -> Size {
        // TODO: Handle height of text with newlines in it
        let width = Conf::CHAR_WIDTH * self.text.len() as u32;
        let height = if width > 0 { Conf::CHAR_HEIGHT } else { 0 };

        Size::new(width, height)
    }
}

impl<'a, C, Conf> Font<'a, C> for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
    Conf: FontBuilderConf,
{
    fn render_str(text: &'a str) -> Self {
        Self {
            pos: Point::zero(),
            text,
            style: Style::default(),
            _conf: Default::default(),
        }
    }
}

impl<'a, C, Conf> WithStyle<C> for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
{
    fn style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn stroke(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn stroke_width(self, _width: u8) -> Self {
        // Noop

        self
    }

    fn fill(mut self, color: Option<C>) -> Self {
        self.style.fill_color = color;

        self
    }
}

/// Pixel iterator for the `FontBuilder` object
#[derive(Debug, Clone, Copy)]
pub struct FontBuilderIterator<'a, C, Conf>
where
    C: PixelColor,
{
    char_walk_x: u32,
    char_walk_y: u32,
    current_char: Option<char>,
    idx: usize,
    pos: Point,
    text: &'a str,
    style: Style<C>,
    _conf: PhantomData<Conf>,
}

impl<'a, C: 'a, Conf: 'a> IntoIterator for FontBuilder<'a, C, Conf>
where
    C: PixelColor + From<BinaryColor>,
    Conf: FontBuilderConf,
{
    type Item = Pixel<C>;
    type IntoIter = FontBuilderIterator<'a, C, Conf>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current_char: self.text.chars().next(),
            idx: 0,
            text: self.text,
            char_walk_x: 0,
            char_walk_y: 0,
            pos: self.pos,
            style: self.style,
            _conf: Default::default(),
        }
    }
}

impl<'a, C, Conf> IntoIterator for &'a FontBuilder<'a, C, Conf>
where
    C: PixelColor + From<BinaryColor>,
    Conf: FontBuilderConf,
{
    type IntoIter = FontBuilderIterator<'a, C, Conf>;
    type Item = Pixel<C>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            current_char: self.text.chars().next(),
            idx: 0,
            text: self.text,
            char_walk_x: 0,
            char_walk_y: 0,
            pos: self.pos,
            style: self.style,
            _conf: Default::default(),
        }
    }
}

impl<'a, C, Conf> Iterator for FontBuilderIterator<'a, C, Conf>
where
    C: PixelColor + From<BinaryColor>,
    Conf: FontBuilderConf,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let char_per_row = Conf::FONT_IMAGE_WIDTH / Conf::CHAR_WIDTH;

        loop {
            if let Some(current_char) = self.current_char {
                // Char _code_ offset from first char, most often a space
                // E.g. first char = ' ' (32), target char = '!' (33), offset = 33 - 32 = 1
                let char_offset = Conf::char_offset(current_char);
                let row = char_offset / char_per_row;

                // Top left corner of character, in pixels
                let char_x = (char_offset - (row * char_per_row)) * Conf::CHAR_WIDTH;
                let char_y = row * Conf::CHAR_HEIGHT;

                // Bit index
                // = X pixel offset for char
                // + Character row offset (row 0 = 0, row 1 = (192 * 8) = 1536)
                // + X offset for the pixel block that comprises this char
                // + Y offset for pixel block
                let bitmap_bit_index = char_x
                    + (Conf::FONT_IMAGE_WIDTH * char_y)
                    + self.char_walk_x
                    + (self.char_walk_y * Conf::FONT_IMAGE_WIDTH);

                let bitmap_byte = bitmap_bit_index / 8;
                let bitmap_bit = 7 - (bitmap_bit_index % 8);

                let color = if Conf::FONT_IMAGE[bitmap_byte as usize] & (1 << bitmap_bit) != 0 {
                    Some(
                        self.style
                            .stroke_color
                            .unwrap_or_else(|| BinaryColor::On.into()),
                    )
                } else {
                    self.style.fill_color
                };

                let x = self.pos.x
                    + (Conf::CHAR_WIDTH * self.idx as u32) as i32
                    + self.char_walk_x as i32;
                let y = self.pos.y + self.char_walk_y as i32;

                self.char_walk_x += 1;

                if self.char_walk_x >= Conf::CHAR_WIDTH {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= Conf::CHAR_HEIGHT {
                        self.char_walk_y = 0;
                        self.idx += 1;
                        self.current_char = self.text.chars().nth(self.idx);
                    }
                }

                // Skip to next point if pixel is transparent
                if let Some(color) = color {
                    break Some(Pixel(Point::new(x, y), color));
                }
            } else {
                break None;
            }
        }
    }
}

impl<'a, C: 'a, Conf: 'a> Drawable for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
    Conf: FontBuilderConf,
{
}

impl<'a, C, Conf> Transform for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Font8x16`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font8x16 };
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// #
    /// # let style = Style::stroke(BinaryColor::On);
    /// #
    /// // 8px x 1px test image
    /// let text = Font8x16::render_str("Hello world")
    /// #    .style(style);
    /// let moved = text.translate(Point::new(25, 30));
    ///
    /// assert_eq!(text.pos, Point::new(0, 0));
    /// assert_eq!(moved.pos, Point::new(25, 30));
    /// ```
    fn translate(&self, by: Point) -> Self {
        Self {
            pos: self.pos + by,
            ..*self
        }
    }

    /// Translate the font origin from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font8x16 };
    /// # use embedded_graphics::prelude::*;
    /// # use embedded_graphics::pixelcolor::BinaryColor;
    /// #
    /// # let style = Style::stroke(BinaryColor::On);
    /// #
    /// // 8px x 1px test image
    /// let mut text = Font8x16::render_str("Hello world")
    /// #    .style(style);
    /// text.translate_mut(Point::new(25, 30));
    ///
    /// assert_eq!(text.pos, Point::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Point) -> &mut Self {
        self.pos += by;

        self
    }
}
