use crate::{
    drawable::Pixel,
    pixelcolor::PixelColor,
    primitives::circle::{diameter_to_threshold, distance_iterator::DistanceIterator, Circle},
    style::{PrimitiveStyle, Styled},
};

/// Pixel iterator for each pixel in the circle border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledCircleIterator<C>
where
    C: PixelColor,
{
    iter: DistanceIterator,

    outer_threshold: u32,
    outer_color: Option<C>,

    inner_threshold: u32,
    inner_color: Option<C>,
}

impl<C> StyledCircleIterator<C>
where
    C: PixelColor,
{
    pub(crate) fn new(styled: &Styled<Circle, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let stroke_area = primitive.expand(style.outside_stroke_width());
        let fill_area = primitive.shrink(style.inside_stroke_width());

        let inner_threshold = diameter_to_threshold(fill_area.diameter);
        let outer_threshold = diameter_to_threshold(stroke_area.diameter);

        let iter = if !styled.style.is_transparent() {
            DistanceIterator::new(&stroke_area)
        } else {
            DistanceIterator::empty()
        };

        Self {
            iter,
            outer_threshold,
            outer_color: styled.style.stroke_color,
            inner_threshold,
            inner_color: styled.style.fill_color,
        }
    }
}

impl<C> Iterator for StyledCircleIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for (point, distance) in &mut self.iter {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::Point,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        prelude::*,
        style::{PrimitiveStyleBuilder, StrokeAlignment},
    };

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

        assert_eq!(display_center, display_inside);
        assert_eq!(display_center, display_outside);
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
                circle_no_stroke.into_iter().eq(circle_stroke.into_iter()),
                "Filled and unfilled circle iters are unequal for radius {}",
                size
            );
        }
    }
}
