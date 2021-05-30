use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{
        common::{Scanline, StyledScanline},
        ellipse::{points::Scanlines, Ellipse, EllipseContains},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        PrimitiveStyle, Rectangle,
    },
    Pixel,
};
use az::SaturatingAs;

/// Pixel iterator for each pixel in the ellipse border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixelsIterator<C> {
    styled_scanlines: StyledScanlines,

    stroke_left: Scanline,
    fill: Scanline,
    stroke_right: Scanline,

    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives) fn new(primitive: &Ellipse, style: &PrimitiveStyle<C>) -> Self {
        let stroke_area = style.stroke_area(primitive);
        let fill_area = style.fill_area(primitive);

        Self {
            styled_scanlines: StyledScanlines::new(&stroke_area, &fill_area),
            stroke_left: Scanline::new_empty(0),
            fill: Scanline::new_empty(0),
            stroke_right: Scanline::new_empty(0),
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.stroke_color, self.fill_color) {
            (Some(stroke_color), None) => loop {
                if let Some(pixel) = self
                    .stroke_left
                    .next()
                    .or_else(|| self.stroke_right.next())
                    .map(|p| Pixel(p, stroke_color))
                {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.stroke_left = scanline.stroke_left();
                self.stroke_right = scanline.stroke_right();
            },
            (Some(stroke_color), Some(fill_color)) => loop {
                if let Some(pixel) = self
                    .stroke_left
                    .next()
                    .map(|p| Pixel(p, stroke_color))
                    .or_else(|| self.fill.next().map(|p| Pixel(p, fill_color)))
                    .or_else(|| self.stroke_right.next().map(|p| Pixel(p, stroke_color)))
                {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.stroke_left = scanline.stroke_left();
                self.fill = scanline.fill();
                self.stroke_right = scanline.stroke_right();
            },
            (None, Some(fill_color)) => loop {
                if let Some(pixel) = self.fill.next().map(|p| Pixel(p, fill_color)) {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.fill = scanline.fill();
            },
            (None, None) => None,
        }
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Ellipse {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Ellipse {
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &PrimitiveStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        match (style.effective_stroke_color(), style.fill_color) {
            (Some(stroke_color), None) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke(target, stroke_color)?;
                }
            }
            (Some(stroke_color), Some(fill_color)) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke_and_fill(target, stroke_color, fill_color)?;
                }
            }
            (None, Some(fill_color)) => {
                for scanline in Scanlines::new(&style.fill_area(self)) {
                    scanline.draw(target, fill_color)?;
                }
            }
            (None, None) => {}
        }

        Ok(())
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Ellipse {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let offset = style.outside_stroke_width().saturating_as();

        self.bounding_box().offset(offset)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct StyledScanlines {
    scanlines: Scanlines,
    fill_area: EllipseContains,
}

impl StyledScanlines {
    pub fn new(stroke_area: &Ellipse, fill_area: &Ellipse) -> Self {
        Self {
            scanlines: Scanlines::new(stroke_area),
            fill_area: EllipseContains::new(fill_area.size),
        }
    }
}

impl Iterator for StyledScanlines {
    type Item = StyledScanline;

    fn next(&mut self) -> Option<Self::Item> {
        self.scanlines.next().map(|scanline| {
            let scaled_y = scanline.y * 2 - self.scanlines.center_2x.y;

            let fill_range = scanline
                .x
                .clone()
                .find(|x| {
                    self.fill_area
                        .contains(Point::new(*x * 2 - self.scanlines.center_2x.x, scaled_y))
                })
                .map(|x| x..scanline.x.end - (x - scanline.x.start));

            StyledScanline::new(scanline.y, scanline.x, fill_range)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
        Drawable,
    };

    fn test_circles(style: PrimitiveStyle<BinaryColor>) {
        for diameter in 0..50 {
            let top_left = Point::new_equal(style.stroke_width.saturating_as());

            let mut expected = MockDisplay::new();
            Circle::new(top_left, diameter)
                .into_styled(style)
                .draw(&mut expected)
                .unwrap();

            let mut display = MockDisplay::new();
            Ellipse::new(top_left, Size::new(diameter, diameter))
                .into_styled(style)
                .draw(&mut display)
                .unwrap();

            display.assert_eq_with_message(&expected, |f| write!(f, "diameter = {}", diameter));
        }
    }

    fn test_ellipse(size: Size, style: PrimitiveStyle<BinaryColor>, pattern: &[&str]) {
        let ellipse = Ellipse::new(Point::new(0, 0), size).into_styled(style);

        let mut drawable = MockDisplay::new();
        ellipse.draw(&mut drawable).unwrap();
        drawable.assert_pattern(pattern);

        let mut pixels = MockDisplay::new();
        ellipse.pixels().draw(&mut pixels).unwrap();
        pixels.assert_pattern(pattern);
    }

    #[test]
    fn ellipse_equals_circle_fill() {
        test_circles(PrimitiveStyle::with_fill(BinaryColor::On));
    }

    #[test]
    fn ellipse_equals_circle_stroke_1px() {
        test_circles(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    }

    #[test]
    fn ellipse_equals_circle_stroke_10px() {
        test_circles(PrimitiveStyle::with_stroke(BinaryColor::On, 10));
    }

    #[test]
    fn filled_ellipse() {
        #[rustfmt::skip]
        test_ellipse(Size::new(20, 10), PrimitiveStyle::with_fill(BinaryColor::On), &[
            "      ########      ",
            "   ##############   ",
            " ################## ",
            "####################",
            "####################",
            "####################",
            "####################",
            " ################## ",
            "   ##############   ",
            "      ########      ",
        ],);
    }

    #[test]
    fn thick_stroke_glitch() {
        test_ellipse(
            Size::new(11, 21),
            PrimitiveStyleBuilder::new()
                .stroke_width(10)
                .stroke_color(BinaryColor::On)
                .stroke_alignment(StrokeAlignment::Inside)
                .fill_color(BinaryColor::Off)
                .build(),
            &[
                "    ###    ",
                "   #####   ",
                "  #######  ",
                " ######### ",
                " ######### ",
                " ######### ",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                "###########",
                " ######### ",
                " ######### ",
                " ######### ",
                "  #######  ",
                "   #####   ",
                "    ###    ",
            ],
        );
    }

    #[test]
    fn thin_stroked_ellipse() {
        #[rustfmt::skip]
        test_ellipse(Size::new(20, 10), PrimitiveStyle::with_stroke(BinaryColor::On, 1), &[
            "      ########      ",
            "   ###        ###   ",
            " ##              ## ",
            "##                ##",
            "#                  #",
            "#                  #",
            "##                ##",
            " ##              ## ",
            "   ###        ###   ",
            "      ########      ",
        ],);
    }

    #[test]
    fn fill_and_stroke() {
        test_ellipse(
            Size::new(20, 10),
            PrimitiveStyleBuilder::new()
                .stroke_width(3)
                .stroke_color(BinaryColor::Off)
                .stroke_alignment(StrokeAlignment::Inside)
                .fill_color(BinaryColor::On)
                .build(),
            &[
                "      ........      ",
                "   ..............   ",
                " .................. ",
                ".....##########.....",
                "...##############...",
                "...##############...",
                ".....##########.....",
                " .................. ",
                "   ..............   ",
                "      ........      ",
            ],
        );
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: Size = Size::new(10, 5);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Ellipse::with_center(CENTER, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Ellipse::with_center(CENTER, SIZE + Size::new(2, 2))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Ellipse::with_center(CENTER, SIZE - Size::new(4, 4))
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Outside)
                    .build(),
            )
            .draw(&mut display_outside)
            .unwrap();

        display_inside.assert_eq(&display_center);
        display_outside.assert_eq(&display_center);
    }

    #[test]
    fn bounding_boxes() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: Size = Size::new(15, 25);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let center = Ellipse::with_center(CENTER, SIZE).into_styled(style);
        let inside = Ellipse::with_center(CENTER, SIZE).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        );
        let outside = Ellipse::with_center(CENTER, SIZE).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Outside)
                .build(),
        );

        let mut display = MockDisplay::new();
        center.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), center.bounding_box());

        let mut display = MockDisplay::new();
        outside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), outside.bounding_box());

        let mut display = MockDisplay::new();
        inside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), inside.bounding_box());
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let transparent_ellipse = Ellipse::new(Point::new(5, 5), Size::new(11, 14))
            .into_styled(PrimitiveStyle::<BinaryColor>::new());
        let filled_ellipse = Ellipse::new(Point::new(5, 5), Size::new(11, 14))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert_eq!(
            transparent_ellipse.bounding_box(),
            filled_ellipse.bounding_box(),
        );
    }
}
