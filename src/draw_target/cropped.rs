use crate::{
    draw_target::{DrawTarget, DrawTargetExt, Translated},
    geometry::{OriginDimensions, Size},
    primitives::Rectangle,
    Pixel,
};

/// Cropped draw target.
///
/// Created by calling [`cropped`] on any [`DrawTarget`].
/// See the [`cropped`] method documentation for more.
///
/// [`cropped`]: DrawTargetExt::cropped
#[derive(Debug)]
pub struct Cropped<'a, T>
where
    T: DrawTarget,
{
    parent: Translated<'a, T>,
    size: Size,
}

impl<'a, T> Cropped<'a, T>
where
    T: DrawTarget,
{
    pub(super) fn new(parent: &'a mut T, area: &Rectangle) -> Self {
        let area = area.intersection(&parent.bounding_box());

        Self {
            parent: parent.translated(area.top_left),
            size: area.size,
        }
    }
}

impl<T> DrawTarget for Cropped<'_, T>
where
    T: DrawTarget,
{
    type Color = T::Color;
    type Error = T::Error;

    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.parent.draw_iter(pixels)
    }

    fn fill_contiguous<I>(&mut self, area: &Rectangle, colors: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Self::Color>,
    {
        self.parent.fill_contiguous(area, colors)
    }

    fn fill_solid(&mut self, area: &Rectangle, color: Self::Color) -> Result<(), Self::Error> {
        self.parent.fill_solid(area, color)
    }
}

impl<T> OriginDimensions for Cropped<'_, T>
where
    T: DrawTarget,
{
    fn size(&self) -> Size {
        self.size
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        draw_target::{DrawTarget, DrawTargetExt},
        geometry::Dimensions,
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Primitive, PrimitiveStyle, Rectangle},
        Drawable, Pixel,
    };

    #[test]
    fn draw_iter() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(2, 3), Size::new(10, 10));
        let mut cropped = display.cropped(&area);

        let pixels = [
            Pixel(Point::new(0, 0), BinaryColor::On),
            Pixel(Point::new(1, 2), BinaryColor::Off),
        ];
        cropped.draw_iter(pixels.iter().copied()).unwrap();

        display.assert_pattern(&[
            "    ", //
            "    ", //
            "    ", //
            "  # ", //
            "    ", //
            "   .", //
        ]);
    }

    #[test]
    fn fill_contiguous() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(3, 2), Size::new(10, 10));
        let mut cropped = display.cropped(&area);

        let colors = [
            1, 1, 1, 1, 1, //
            0, 0, 0, 0, 1, //
            0, 1, 0, 1, 1, //
            1, 0, 1, 0, 1, //
        ];
        let area = Rectangle::new(Point::new(1, 2), Size::new(5, 4));
        cropped
            .fill_contiguous(&area, colors.iter().map(|c| BinaryColor::from(*c != 0)))
            .unwrap();

        display.assert_pattern(&[
            "         ", //
            "         ", //
            "         ", //
            "         ", //
            "    #####", //
            "    ....#", //
            "    .#.##", //
            "    #.#.#", //
        ]);
    }

    #[test]
    fn fill_solid() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(1, 3), Size::new(10, 10));
        let mut cropped = display.cropped(&area);

        let area = Rectangle::new(Point::new(2, 1), Size::new(3, 4));
        cropped.fill_solid(&area, BinaryColor::On).unwrap();

        display.assert_pattern(&[
            "      ", //
            "      ", //
            "      ", //
            "      ", //
            "   ###", //
            "   ###", //
            "   ###", //
            "   ###", //
        ]);
    }

    #[test]
    fn clear() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(1, 3), Size::new(3, 4));
        let mut cropped = display.cropped(&area);
        cropped.clear(BinaryColor::On).unwrap();

        let mut expected = MockDisplay::new();
        area.into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut expected)
            .unwrap();

        display.assert_eq(&expected);
    }

    #[test]
    fn bounding_box() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        let size = Size::new(3, 4);
        let area = Rectangle::new(Point::new(1, 3), size);
        let cropped = display.cropped(&area);

        assert_eq!(cropped.bounding_box(), Rectangle::new(Point::zero(), size));
    }

    #[test]
    fn bounding_box_is_clipped() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        let display_bb = display.bounding_box();

        let top_left = Point::new(10, 20);
        let size = Size::new(1000, 1000);
        let area = Rectangle::new(top_left, size);
        let cropped = display.cropped(&area);

        let expected_size = display_bb.size - Size::new(top_left.x as u32, top_left.y as u32);

        assert_eq!(
            cropped.bounding_box(),
            Rectangle::new(Point::zero(), expected_size),
        );
    }
}
