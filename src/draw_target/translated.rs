use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    iterator::PixelIteratorExt,
    primitives::Rectangle,
    transform::Transform,
    Pixel,
};

/// Translated draw target.
///
/// Created by calling [`translated`] on any [`DrawTarget`].
/// See the [`translated`] method documentation for more.
///
/// [`DrawTarget`]: trait.DrawTarget.html
/// [`translated`]: trait.DrawTargetExt.html#tymethod.translated
#[derive(Debug)]
pub struct Translated<'a, T>
where
    T: DrawTarget,
{
    parent: &'a mut T,
    offset: Point,
}

impl<'a, T> Translated<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(parent: &'a mut T, offset: Point) -> Self {
        Self { parent, offset }
    }
}

impl<T> DrawTarget for Translated<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.parent
            .draw_iter(pixels.into_iter().translate(self.offset))
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        let area = area.translate(self.offset);
        self.parent.fill_contiguous(&area, colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        let area = area.translate(self.offset);
        self.parent.fill_solid(&area, color)
    }

    fn clear(&mut self, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.clear(color)
    }
}

impl<T> Dimensions for Translated<'_, T>
where
    T: DrawTarget,
{
    fn bounding_box(&self) -> Rectangle {
        self.parent.bounding_box().translate(-self.offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        draw_target::DrawTargetExt, geometry::Size, mock_display::MockDisplay,
        pixelcolor::BinaryColor,
    };

    #[test]
    fn draw_iter() {
        let mut display = MockDisplay::new();

        let mut translated = display.translated(Point::new(2, 3));

        let pixels = [
            Pixel(Point::new(0, 0), BinaryColor::On),
            Pixel(Point::new(1, 2), BinaryColor::Off),
        ];
        translated.draw_iter(pixels.iter().copied()).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "    ", //
                "    ", //
                "    ", //
                "  # ", //
                "    ", //
                "   .", //
            ])
        );
    }

    #[test]
    fn fill_contiguous() {
        let mut display = MockDisplay::new();

        let mut translated = display.translated(Point::new(3, 2));

        let colors = [
            1, 1, 1, 1, 1, //
            0, 0, 0, 0, 1, //
            0, 1, 0, 1, 1, //
            1, 0, 1, 0, 1, //
        ];
        let area = Rectangle::new(Point::new(1, 2), Size::new(5, 4));
        translated
            .fill_contiguous(&area, colors.iter().map(|c| BinaryColor::from(*c != 0)))
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "         ", //
                "         ", //
                "         ", //
                "         ", //
                "    #####", //
                "    ....#", //
                "    .#.##", //
                "    #.#.#", //
            ])
        );
    }

    #[test]
    fn fill_solid() {
        let mut display = MockDisplay::new();

        let mut translated = display.translated(Point::new(1, 3));

        let area = Rectangle::new(Point::new(2, 1), Size::new(3, 4));
        translated.fill_solid(&area, BinaryColor::On).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "      ", //
                "      ", //
                "      ", //
                "      ", //
                "   ###", //
                "   ###", //
                "   ###", //
                "   ###", //
            ])
        );
    }

    #[test]
    fn clear() {
        let mut display = MockDisplay::new();
        let mut translated = display.translated(Point::new(1, 3));
        translated.clear(BinaryColor::On).unwrap();

        let mut expected = MockDisplay::new();
        expected.clear(BinaryColor::On).unwrap();

        assert_eq!(display, expected);
    }

    #[test]
    fn bounding_box() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        let display_bb = display.bounding_box();

        let translated = display.translated(Point::new(1, 3));

        assert_eq!(
            display_bb.translate(-Point::new(1, 3)),
            translated.bounding_box()
        );
    }
}
