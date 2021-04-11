//! A target for embedded-graphics drawing operations.

mod clipped;
mod color_converted;
mod cropped;
mod translated;

use crate::{geometry::Point, pixelcolor::PixelColor, primitives::Rectangle};

pub use clipped::Clipped;
pub use color_converted::ColorConverted;
pub use cropped::Cropped;
pub use translated::Translated;

pub use embedded_graphics_core::draw_target::DrawTarget;

/// Extension trait for `DrawTarget`s.
pub trait DrawTargetExt: DrawTarget + Sized {
    /// Creates a translated draw target based on this draw target.
    ///
    /// All drawing operations are translated by `offset` pixels, before being passed to the parent
    /// draw target.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     mock_display::MockDisplay,
    ///     mono_font::{ascii::FONT_6X9, MonoTextStyle},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    ///     text::Text,
    /// };
    ///
    /// let mut display = MockDisplay::new();
    /// let mut translated_display = display.translated(Point::new(5, 10));
    ///
    /// let style = MonoTextStyle::new(&FONT_6X9, BinaryColor::On);
    ///
    /// // Draws text at position (5, 10) in the display coordinate system
    /// Text::new("Text", Point::zero(), style).draw(&mut translated_display)?;
    /// #
    /// # let mut expected = MockDisplay::new();
    /// #
    /// # Text::new("Text", Point::new(5, 10), style).draw(&mut expected)?;
    /// #
    /// # display.assert_eq(&expected);
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn translated(&mut self, offset: Point) -> Translated<'_, Self>;

    /// Creates a cropped draw target based on this draw target.
    ///
    /// A cropped draw target is a draw target for a rectangular subregion of the parent draw target.
    /// Its coordinate system is shifted so that the origin coincides with `area.top_left` in the
    /// parent draw target's coordinate system.
    ///
    /// The bounding box of the returned target will always be contained inside the bounding box
    /// of the parent target. If any of the requested `area` lies outside the parent target's bounding
    /// box the intersection of the parent target's bounding box and `area` will be used.
    ///
    /// Drawing operations outside the bounding box will not be clipped.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     mock_display::MockDisplay,
    ///     mono_font::{ascii::FONT_6X9, MonoTextStyle},
    ///     pixelcolor::Rgb565,
    ///     prelude::*,
    ///     primitives::Rectangle,
    ///     text::{Text, Alignment, Baseline, TextStyleBuilder},
    /// };
    ///
    /// /// Fills a draw target with a blue background and prints centered yellow text.
    /// fn draw_text<T>(target: &mut T, text: &str) -> Result<(), T::Error>
    /// where
    ///     T: DrawTarget<Color = Rgb565>,
    /// {
    ///     target.clear(Rgb565::BLUE)?;
    ///
    ///     let text_position = target.bounding_box().center();
    ///
    ///     let character_style = MonoTextStyle::new(&FONT_6X9, Rgb565::YELLOW);
    ///     let text_style = TextStyleBuilder::new()
    ///         .alignment(Alignment::Center)
    ///         .baseline(Baseline::Middle)
    ///         .build();
    ///
    ///     Text::with_text_style(text, text_position, character_style, text_style).draw(target)?;
    ///
    ///     Ok(())
    /// }
    ///
    /// let mut display = MockDisplay::new();
    /// display.set_allow_overdraw(true);
    ///
    /// let area = Rectangle::new(Point::new(5, 10), Size::new(40, 15));
    /// let mut cropped_display = display.cropped(&area);
    ///
    /// draw_text(&mut cropped_display, "Text")?;
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn cropped(&mut self, area: &Rectangle) -> Cropped<'_, Self>;

    /// Creates a clipped draw target based on this draw target.
    ///
    /// A clipped draw target is a draw target for a rectangular subregion of the parent draw target.
    /// The coordinate system of the created draw target is equal to the parent target's coordinate
    /// system. All drawing operations outside the bounding box will be clipped.
    ///
    /// The bounding box of the returned target will always be contained inside the bounding box
    /// of the parent target. If any of the requested `area` lies outside the parent target's bounding
    /// box the intersection of the parent target's bounding box and `area` will be used.
    ///
    /// # Examples
    ///
    /// ```
    /// use embedded_graphics::{
    ///     mock_display::MockDisplay,
    ///     mono_font::{ascii::FONT_10X20, MonoTextStyle},
    ///     pixelcolor::BinaryColor,
    ///     prelude::*,
    ///     primitives::Rectangle,
    ///     text::Text,
    /// };
    ///
    /// let mut display = MockDisplay::new();
    ///
    /// let area = Rectangle::new(Point::zero(), Size::new(4 * 10, 20));
    /// let mut clipped_display = display.clipped(&area);
    ///
    /// let style = MonoTextStyle::new(&FONT_10X20, BinaryColor::On);
    ///
    /// // Only the first 4 characters will be drawn, because the others are outside
    /// // the clipping area
    /// Text::new("Clipped", Point::new(0, 15), style).draw(&mut clipped_display)?;
    /// #
    /// # let mut expected = MockDisplay::new();
    /// #
    /// # Text::new("Clip", Point::new(0, 15), style).draw(&mut expected)?;
    /// #
    /// # display.assert_eq(&expected);
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn clipped(&mut self, area: &Rectangle) -> Clipped<'_, Self>;

    /// Creates a color conversion draw target.
    ///
    /// A color conversion draw target is used to draw drawables with a different color type to a
    /// draw target. The drawable color type must implement `Into<C>`, where `C` is the draw
    /// target color type.
    ///
    /// # Performance
    ///
    /// Color conversion can be expensive on embedded hardware and should be avoided if possible.
    /// Using the same color type for drawables and the draw target makes sure that no unnecessary
    /// color conversion is used. But in some cases color conversion will be required, for example,
    /// to draw images with a color format only known at runtime.
    ///
    /// # Examples
    ///
    /// This example draws a `BinaryColor` image to an `Rgb888` display.
    ///
    /// ```
    /// use embedded_graphics::{
    ///     image::{Image, ImageRaw},
    ///     mock_display::MockDisplay,
    ///     pixelcolor::{BinaryColor, Rgb888},
    ///     prelude::*,
    /// };
    ///
    /// /// The image data.
    /// const DATA: &[u8] = &[
    ///     0b11110000, //
    ///     0b10010000, //
    ///     0b10010000, //
    ///     0b11110000, //
    /// ];
    ///
    /// // Create a `BinaryColor` image from the image data.
    /// let raw_image = ImageRaw::<BinaryColor>::new(DATA, 4);
    /// let image = Image::new(&raw_image, Point::zero());
    ///
    /// // Create a `Rgb888` display.
    /// let mut display = MockDisplay::<Rgb888>::new();
    ///
    /// // The image can't directly be drawn to the draw target because they use different
    /// // color type. This will fail to compile:
    /// // image.draw(&mut display)?;
    ///
    /// // To fix this `color_converted` is added to enable color conversion.
    /// image.draw(&mut display.color_converted())?;
    /// #
    /// # let mut expected = MockDisplay::from_pattern(&[
    /// #     "WWWW", //
    /// #     "WKKW", //
    /// #     "WKKW", //
    /// #     "WWWW", //
    /// # ]);
    /// #
    /// # display.assert_eq(&expected);
    /// #
    /// # Ok::<(), core::convert::Infallible>(())
    /// ```
    fn color_converted<C>(&mut self) -> ColorConverted<'_, Self, C>
    where
        C: PixelColor + Into<Self::Color>;
}

impl<T> DrawTargetExt for T
where
    T: DrawTarget,
{
    fn translated(&mut self, offset: Point) -> Translated<'_, Self> {
        Translated::new(self, offset)
    }

    fn cropped(&mut self, area: &Rectangle) -> Cropped<'_, Self> {
        Cropped::new(self, area)
    }

    fn clipped(&mut self, area: &Rectangle) -> Clipped<'_, Self> {
        Clipped::new(self, area)
    }

    fn color_converted<C>(&mut self) -> ColorConverted<'_, Self, C>
    where
        C: PixelColor + Into<Self::Color>,
    {
        ColorConverted::new(self)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        draw_target::{DrawTarget, DrawTargetExt},
        geometry::{Dimensions, Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Primitive, PrimitiveStyle, Rectangle},
        Drawable, Pixel,
    };

    #[test]
    fn draw_iter() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(2, 1), Size::new(2, 4));
        let mut clipped = display.clipped(&area);

        let pixels = [
            Pixel(Point::new(0, 1), BinaryColor::On),
            Pixel(Point::new(1, 1), BinaryColor::On),
            Pixel(Point::new(2, 1), BinaryColor::On),
            Pixel(Point::new(3, 1), BinaryColor::On),
            Pixel(Point::new(4, 1), BinaryColor::On),
            Pixel(Point::new(2, 0), BinaryColor::Off),
            Pixel(Point::new(2, 2), BinaryColor::Off),
            Pixel(Point::new(2, 3), BinaryColor::Off),
            Pixel(Point::new(2, 4), BinaryColor::Off),
            Pixel(Point::new(2, 5), BinaryColor::Off),
        ];
        clipped.draw_iter(pixels.iter().copied()).unwrap();

        display.assert_pattern(&[
            "    ", //
            "  ##", //
            "  . ", //
            "  . ", //
            "  . ", //
        ]);
    }

    #[test]
    fn fill_contiguous() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(3, 2), Size::new(2, 3));
        let mut clipped = display.clipped(&area);

        let colors = [
            1, 1, 1, 1, 1, //
            0, 0, 0, 0, 1, //
            0, 1, 0, 1, 1, //
            1, 0, 1, 0, 1, //
        ];
        let area = Rectangle::new(Point::new(1, 2), Size::new(5, 4));
        clipped
            .fill_contiguous(&area, colors.iter().map(|c| BinaryColor::from(*c != 0)))
            .unwrap();

        display.assert_pattern(&[
            "     ", //
            "     ", //
            "   ##", //
            "   ..", //
            "   .#", //
        ]);
    }

    #[test]
    fn fill_solid() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(3, 2), Size::new(4, 2));
        let mut clipped = display.clipped(&area);

        let area = Rectangle::new(Point::new(2, 1), Size::new(6, 4));
        clipped.fill_solid(&area, BinaryColor::On).unwrap();

        display.assert_pattern(&[
            "       ", //
            "       ", //
            "   ####", //
            "   ####", //
        ]);
    }

    #[test]
    fn clear() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(1, 3), Size::new(3, 4));
        let mut clipped = display.clipped(&area);
        clipped.clear(BinaryColor::On).unwrap();

        let mut expected = MockDisplay::new();
        area.into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut expected)
            .unwrap();

        display.assert_eq(&expected);
    }

    #[test]
    fn bounding_box() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        let area = Rectangle::new(Point::new(1, 3), Size::new(2, 4));
        let clipped = display.clipped(&area);

        assert_eq!(clipped.bounding_box(), area);
    }

    #[test]
    fn bounding_box_is_clipped() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        let display_bb = display.bounding_box();

        let top_left = Point::new(10, 20);
        let size = Size::new(1000, 1000);
        let area = Rectangle::new(top_left, size);
        let clipped = display.clipped(&area);

        let expected_size = display_bb.size - Size::new(top_left.x as u32, top_left.y as u32);

        assert_eq!(
            clipped.bounding_box(),
            Rectangle::new(top_left, expected_size),
        );
    }
}
