use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Point, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        ellipse::{compute_threshold, is_point_inside_ellipse, points::Points, Ellipse},
        OffsetOutline, Rectangle,
    },
    style::{PrimitiveStyle, Styled, StyledPrimitiveAreas},
    SaturatingCast,
};

/// Pixel iterator for each pixel in the ellipse border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: Points,
    outer_color: Option<C>,
    inner_size_sq: Size,
    inner_color: Option<C>,
    center: Point,
    threshold: u32,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Ellipse, PrimitiveStyle<C>>) -> Self {
        let iter = if !styled.style.is_transparent() {
            Points::new(&styled.stroke_area())
        } else {
            Points::empty()
        };

        let fill_area = styled.fill_area();
        let (inner_size_sq, threshold) = compute_threshold(fill_area.size);

        Self {
            iter,
            outer_color: styled.style.stroke_color,
            inner_size_sq,
            inner_color: styled.style.fill_color,
            center: styled.primitive.center_2x(),
            threshold,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            let inside_border = is_point_inside_ellipse(
                self.inner_size_sq,
                point * 2 - self.center,
                self.threshold,
            );

            let color = if inside_border {
                self.inner_color
            } else {
                self.outer_color
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
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

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self.into_pixels())
    }
}

impl<C> Dimensions for Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        if !self.style.is_transparent() {
            let offset = self.style.outside_stroke_width().saturating_cast();

            self.primitive.bounding_box().offset(offset)
        } else {
            Rectangle::new(self.primitive.center(), Size::zero())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, Primitive},
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
        SaturatingCast,
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

            assert_eq!(display, expected, "diameter = {}", diameter);
        }
    }

    fn test_ellipse(size: Size, style: PrimitiveStyle<BinaryColor>, pattern: &[&str]) {
        let mut display = MockDisplay::new();

        Ellipse::new(Point::new(0, 0), size)
            .into_styled(style)
            .draw(&mut display)
            .unwrap();

        assert_eq!(display, MockDisplay::from_pattern(pattern));
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

        assert_eq!(display_center, display_inside);
        assert_eq!(display_center, display_outside);
    }

    #[test]
    fn bounding_boxes() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: Size = Size::new(15, 25);

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let center = Ellipse::with_center(CENTER, SIZE).into_styled(style);
        let inside = Ellipse::with_center(CENTER, SIZE + Size::new_equal(2)).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        );
        let outside = Ellipse::with_center(CENTER, SIZE - Size::new_equal(4)).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Outside)
                .build(),
        );

        assert_eq!(center.bounding_box(), inside.bounding_box());
        assert_eq!(outside.bounding_box(), inside.bounding_box());

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
    fn transparent_bounding_box() {
        let ellipse = Ellipse::new(Point::new(5, 5), Size::new(11, 14))
            .into_styled::<BinaryColor>(PrimitiveStyle::new());

        assert_eq!(
            ellipse.bounding_box(),
            Rectangle::new(ellipse.primitive.center(), Size::zero())
        );
    }
}
