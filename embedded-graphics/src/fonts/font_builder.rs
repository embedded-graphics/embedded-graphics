//! Common code used to define available monospace pixel fonts

use crate::coord::Coord;
use crate::coord::ToUnsigned;
use crate::drawable::Dimensions;
use crate::drawable::Drawable;
use crate::drawable::Pixel;
use crate::fonts::Font;
use crate::pixelcolor::PixelColor;
use crate::style::Style;
use crate::style::WithStyle;
use crate::transform::Transform;
use crate::unsignedcoord::{ToSigned, UnsignedCoord};
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
#[derive(Debug)]
pub struct FontBuilder<'a, C: PixelColor, Conf> {
    /// Top left corner of the text
    pub pos: Coord,

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
            style: self.style.clone(),
            _conf: Default::default(),
        }
    }
}

impl<'a, C, Conf> Dimensions for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
    Conf: FontBuilderConf,
{
    fn top_left(&self) -> Coord {
        self.pos
    }

    fn bottom_right(&self) -> Coord {
        self.top_left() + self.size().to_signed()
    }

    /// Get the bounding box of a piece of text
    ///
    /// Currently does not handle newlines (but neither does the rasteriser). It will give `(0, 0)`
    /// if the string to render is empty.
    fn size(&self) -> UnsignedCoord {
        // TODO: Handle height of text with newlines in it
        let width = Conf::CHAR_WIDTH * self.text.len() as u32;
        let height = if width > 0 { Conf::CHAR_HEIGHT } else { 0 };

        UnsignedCoord::new(width, height)
    }
}

impl<'a, C, Conf> Font<'a, C> for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
    Conf: FontBuilderConf,
{
    fn render_str(text: &'a str) -> Self {
        Self {
            pos: Coord::new(0, 0),
            text,
            style: Style::default(),
            _conf: Default::default(),
        }
    }

    fn dimensions(&self) -> UnsignedCoord {
        UnsignedCoord::new(
            Conf::CHAR_WIDTH * self.text.len() as u32,
            if self.text.len() > 0 {
                Conf::CHAR_HEIGHT
            } else {
                0
            },
        )
    }
}

impl<'a, C, Conf> WithStyle<C> for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
{
    fn with_style(mut self, style: Style<C>) -> Self {
        self.style = style;

        self
    }

    fn with_stroke(mut self, color: Option<C>) -> Self {
        self.style.stroke_color = color;

        self
    }

    fn with_stroke_width(self, _width: u8) -> Self {
        // Noop

        self
    }

    fn with_fill(mut self, color: Option<C>) -> Self {
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
    pos: Coord,
    text: &'a str,
    style: Style<C>,
    _conf: PhantomData<Conf>,
}

impl<'a, C, Conf> IntoIterator for &'a FontBuilder<'a, C, Conf>
where
    C: PixelColor,
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
    C: PixelColor,
    Conf: FontBuilderConf,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos[0] + (self.text.len() as i32 * Conf::CHAR_WIDTH as i32) < 0
            || self.pos[1] + (Conf::CHAR_HEIGHT as i32) < 0
        {
            return None;
        }

        let char_per_row = Conf::FONT_IMAGE_WIDTH / Conf::CHAR_WIDTH;
        if let Some(current_char) = self.current_char {
            let pixel = loop {
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
                    self.style.stroke_color.unwrap_or(1.into()) // white
                } else {
                    self.style.fill_color.unwrap_or(0.into()) // black
                };

                let x = self.pos[0]
                    + (Conf::CHAR_WIDTH * self.idx as u32) as i32
                    + self.char_walk_x as i32;
                let y = self.pos[1] + self.char_walk_y as i32;

                self.char_walk_x += 1;

                if self.char_walk_x >= Conf::CHAR_WIDTH {
                    self.char_walk_x = 0;
                    self.char_walk_y += 1;

                    // Done with this char, move on to the next one
                    if self.char_walk_y >= Conf::CHAR_HEIGHT {
                        self.char_walk_y = 0;
                        self.idx += 1;
                        self.current_char = self.text.chars().skip(self.idx).next();
                    }
                }

                if x >= 0 && y >= 0 {
                    break Some(Pixel(Coord::new(x, y).to_unsigned(), color));
                }
            };

            pixel
        } else {
            None
        }
    }
}

impl<'a, C, Conf> Drawable for FontBuilder<'a, C, Conf> where C: PixelColor {}

impl<'a, C, Conf> Transform for FontBuilder<'a, C, Conf>
where
    C: PixelColor,
{
    /// Translate the image from its current position to a new position by (x, y) pixels, returning
    /// a new `Font8x16`. For a mutating transform, see `translate_mut`.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font8x16 };
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// // 8px x 1px test image
    /// let text = Font8x16::render_str("Hello world")
    /// #    .with_style(style);
    /// let moved = text.translate(Coord::new(25, 30));
    ///
    /// assert_eq!(text.pos, Coord::new(0, 0));
    /// assert_eq!(moved.pos, Coord::new(25, 30));
    /// ```
    fn translate(&self, by: Coord) -> Self {
        Self {
            pos: self.pos + by,
            ..self.clone()
        }
    }

    /// Translate the font origin from its current position to a new position by (x, y) pixels.
    ///
    /// ```
    /// # use embedded_graphics::fonts::{ Font, Font8x16 };
    /// # use embedded_graphics::dev::TestPixelColor;
    /// # use embedded_graphics::prelude::*;
    /// #
    /// # let style: Style<TestPixelColor> = Style::with_stroke(TestPixelColor(1));
    /// #
    /// // 8px x 1px test image
    /// let mut text = Font8x16::render_str("Hello world")
    /// #    .with_style(style);
    /// text.translate_mut(Coord::new(25, 30));
    ///
    /// assert_eq!(text.pos, Coord::new(25, 30));
    /// ```
    fn translate_mut(&mut self, by: Coord) -> &mut Self {
        self.pos += by;

        self
    }
}
