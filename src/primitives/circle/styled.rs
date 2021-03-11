use crate::{
    draw_target::DrawTarget,
    geometry::Dimensions,
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        circle::Circle, common::DistanceIterator, rectangle::Rectangle, PrimitiveStyle,
        StyledPrimitiveAreas,
    },
    Drawable, Pixel, SaturatingCast, Styled,
};

/// Pixel iterator for each pixel in the circle border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: DistanceIterator,

    outer_threshold: u32,
    outer_color: Option<C>,

    inner_threshold: u32,
    inner_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Circle, PrimitiveStyle<C>>) -> Self {
        let stroke_area = styled.stroke_area();
        let fill_area = styled.fill_area();

        let iter = if !styled.style.is_transparent() {
            stroke_area.distances()
        } else {
            DistanceIterator::empty()
        };

        Self {
            iter,
            outer_threshold: stroke_area.threshold(),
            outer_color: styled.style.stroke_color,
            inner_threshold: fill_area.threshold(),
            inner_color: styled.style.fill_color,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for (point, _, distance) in &mut self.iter {
            let color = if distance < self.inner_threshold {
                self.inner_color
            } else if distance < self.outer_threshold {
                self.outer_color
            } else {
                None
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
    }
}

impl<C> IntoPixels for &Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, display: &mut D) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self.into_pixels())
    }
}

impl<C> Dimensions for Styled<Circle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let offset = self.style.outside_stroke_width().saturating_cast();

        self.primitive.bounding_box().offset(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{PointsIter, Primitive, PrimitiveStyleBuilder, StrokeAlignment},
        Drawable,
    };

    #[test]
    fn filled_styled_matches_points() {
        let circle = Circle::with_center(Point::new(10, 10), 5);

        let styled_points = circle
            .clone()
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .into_pixels()
            .map(|Pixel(p, _)| p);

        assert!(circle.points().eq(styled_points));
    }

    // Check that tiny circles render as a "+" shape with a hole in the center
    #[test]
    fn tiny_circle() {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), 3)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " # ", //
            "# #", //
            " # ", //
        ]);
    }

    // Check that tiny filled circle render as a "+" shape with NO hole in the center
    #[test]
    fn tiny_circle_filled() {
        let mut display = MockDisplay::new();

        Circle::new(Point::new(0, 0), 3)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        display.assert_pattern(&[
            " # ", //
            "###", //
            " # ", //
        ]);
    }

    #[test]
    fn transparent_border() {
        let circle: Styled<Circle, PrimitiveStyle<BinaryColor>> =
            Circle::new(Point::new(-5, -5), 21)
                .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert!(circle.into_pixels().count() > 0);
    }

    #[test]
    fn it_handles_negative_coordinates() {
        let positive = Circle::new(Point::new(10, 10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_pixels();

        let negative = Circle::new(Point::new(-10, -10), 5)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .into_pixels();

        assert!(negative.eq(positive.map(|Pixel(p, c)| Pixel(p - Point::new(20, 20), c))));
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Circle::with_center(CENTER, SIZE)
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Circle::with_center(CENTER, SIZE + 2)
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Circle::with_center(CENTER, SIZE - 4)
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

    /// Test for issue #143
    #[test]
    fn issue_143_stroke_and_fill() {
        for size in 0..10 {
            let circle_no_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size)
                    .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

            let style = PrimitiveStyleBuilder::new()
                .fill_color(BinaryColor::On)
                .stroke_color(BinaryColor::On)
                .stroke_width(1)
                .build();
            let circle_stroke: Styled<Circle, PrimitiveStyle<BinaryColor>> =
                Circle::new(Point::new(10, 16), size).into_styled(style);

            assert_eq!(
                circle_stroke.bounding_box(),
                circle_no_stroke.bounding_box(),
                "Filled and unfilled circle bounding boxes are unequal for radius {}",
                size
            );
            assert!(
                circle_no_stroke
                    .into_pixels()
                    .eq(circle_stroke.into_pixels()),
                "Filled and unfilled circle iters are unequal for radius {}",
                size
            );
        }
    }

    #[test]
    fn bounding_boxes() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let center = Circle::with_center(CENTER, SIZE).into_styled(style);

        let inside = Circle::with_center(CENTER, SIZE).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        );

        let outside = Circle::with_center(CENTER, SIZE).into_styled(
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
        let transparent_circle =
            Circle::new(Point::new(5, 5), 11).into_styled::<BinaryColor>(PrimitiveStyle::new());
        let filled_circle = Circle::new(Point::new(5, 5), 11)
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));

        assert_eq!(
            transparent_circle.bounding_box(),
            filled_circle.bounding_box(),
        );
    }
}
