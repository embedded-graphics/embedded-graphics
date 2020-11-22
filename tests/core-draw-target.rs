use embedded_graphics::{
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{Dimensions, Point, Size},
    mock_display::MockDisplay,
    pixelcolor::BinaryColor,
    primitives::{Primitive, Rectangle},
    style::PrimitiveStyle,
    Drawable, Pixel,
};

#[cfg(test)]
mod tests {
    use super::*;

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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "    ", //
                "  ##", //
                "  . ", //
                "  . ", //
                "  . ", //
            ])
        );
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

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ", //
                "     ", //
                "   ##", //
                "   ..", //
                "   .#", //
            ])
        );
    }

    #[test]
    fn fill_solid() {
        let mut display = MockDisplay::new();

        let area = Rectangle::new(Point::new(3, 2), Size::new(4, 2));
        let mut clipped = display.clipped(&area);

        let area = Rectangle::new(Point::new(2, 1), Size::new(6, 4));
        clipped.fill_solid(&area, BinaryColor::On).unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "       ", //
                "       ", //
                "   ####", //
                "   ####", //
            ])
        );
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

        assert_eq!(display, expected);
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
