use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        common::{Scanline, StyledScanline},
        ellipse::{points::Scanlines, Ellipse, EllipseContains},
        PrimitiveStyle, Rectangle, Styled, StyledPrimitiveAreas,
    },
    Drawable, Pixel, SaturatingCast,
};

/// Pixel iterator for each pixel in the ellipse border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    styled_scanlines: StyledScanlines,

    stroke_left: Scanline,
    fill: Scanline,
    stroke_right: Scanline,

    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Ellipse, PrimitiveStyle<C>>) -> Self {
        let stroke_area = styled.stroke_area();
        let fill_area = styled.fill_area();

        Self {
            styled_scanlines: StyledScanlines::new(&stroke_area, &fill_area),
            stroke_left: Scanline::new_empty(0),
            fill: Scanline::new_empty(0),
            stroke_right: Scanline::new_empty(0),
            stroke_color: styled.style.stroke_color,
            fill_color: styled.style.fill_color,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
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

impl<C> IntoPixels for &Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        match (self.style.effective_stroke_color(), self.style.fill_color) {
            (Some(stroke_color), None) => {
                for scanline in StyledScanlines::new(&self.stroke_area(), &self.fill_area()) {
                    scanline.draw_stroke(target, stroke_color)?;
                }
            }
            (Some(stroke_color), Some(fill_color)) => {
                for scanline in StyledScanlines::new(&self.stroke_area(), &self.fill_area()) {
                    scanline.draw_stroke_and_fill(target, stroke_color, fill_color)?;
                }
            }
            (None, Some(fill_color)) => {
                for scanline in Scanlines::new(&self.fill_area()) {
                    scanline.draw(target, fill_color)?;
                }
            }
            (None, None) => {}
        }

        Ok(())
    }
}

impl<C> Dimensions for Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let offset = self.style.outside_stroke_width().saturating_cast();

        self.primitive.bounding_box().offset(offset)
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
        Drawable, SaturatingCast,
    };

    fn test_circles(style: PrimitiveStyle<BinaryColor>) {
        for diameter in 0..50 {
            let top_left = Point::new_equal(style.stroke_width.saturating_cast());

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

        let mut into_pixels = MockDisplay::new();
        ellipse.into_pixels().draw(&mut into_pixels).unwrap();
        into_pixels.assert_pattern(pattern);
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
            .into_styled::<BinaryColor>(PrimitiveStyle::new());
        let filled_ellipse = Ellipse::new(Point::new(5, 5), Size::new(11, 14))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert_eq!(
            transparent_ellipse.bounding_box(),
            filled_ellipse.bounding_box(),
        );
    }
}
