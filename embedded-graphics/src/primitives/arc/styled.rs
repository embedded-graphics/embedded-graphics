use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        arc::{plane_sector::PlaneSectorIterator, Arc},
        circle::DistanceIterator,
        OffsetOutline, Rectangle, Styled,
    },
    style::PrimitiveStyle,
    SaturatingCast,
};

/// Pixel iterator for each pixel in the arc border
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: DistanceIterator<PlaneSectorIterator>,

    outer_threshold: u32,
    inner_threshold: u32,

    stroke_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    fn new(styled: &Styled<Arc, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let circle = primitive.to_circle();

        let outside_edge = circle.offset(style.outside_stroke_width().saturating_cast());
        let inside_edge = circle.offset(style.inside_stroke_width().saturating_cast_neg());

        let points = if !styled.style.is_transparent() {
            PlaneSectorIterator::new(
                &outside_edge,
                primitive.center(),
                primitive.angle_start,
                primitive.angle_sweep,
            )
        } else {
            PlaneSectorIterator::empty()
        };

        Self {
            iter: outside_edge.distances(points),
            outer_threshold: outside_edge.threshold(),
            inner_threshold: inside_edge.threshold(),
            stroke_color: styled.style.stroke_color,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let stroke_color = self.stroke_color?;
        let outer_threshold = self.outer_threshold;
        let inner_threshold = self.inner_threshold;

        self.iter
            .find(|(_, distance)| *distance < outer_threshold && *distance >= inner_threshold)
            .map(|(point, _)| Pixel(point, stroke_color))
    }
}

impl<C> Drawable for Styled<Arc, PrimitiveStyle<C>>
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

impl<C> IntoPixels for &Styled<Arc, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Dimensions for Styled<Arc, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    // FIXME: This doesn't take into account start/end angles. This should be fixed to close #405.
    fn bounding_box(&self) -> Rectangle {
        if self.style.is_transparent() {
            Rectangle::new(self.primitive.bounding_box().center(), Size::zero())
        } else {
            let offset = self.style.outside_stroke_width().saturating_cast();

            self.primitive.bounding_box().offset(offset)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{AngleUnit, Point},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::Primitive,
        style::{PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

    // Check the rendering of a simple arc
    #[test]
    fn tiny_arc() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Arc::new(Point::zero(), 7, 30.0.deg(), 120.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)?;

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  ###  ",
                " #   # ",
            ])
        );

        Ok(())
    }

    #[test]
    fn stroke_alignment() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let mut display_center = MockDisplay::new();
        Arc::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg())
            .into_styled(style)
            .draw(&mut display_center)
            .unwrap();

        let mut display_inside = MockDisplay::new();
        Arc::with_center(CENTER, SIZE + 2, 0.0.deg(), 90.0.deg())
            .into_styled(
                PrimitiveStyleBuilder::from(&style)
                    .stroke_alignment(StrokeAlignment::Inside)
                    .build(),
            )
            .draw(&mut display_inside)
            .unwrap();

        let mut display_outside = MockDisplay::new();
        Arc::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg())
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
        const SIZE: u32 = 10;

        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 3);

        let center = Arc::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg()).into_styled(style);
        let inside = Arc::with_center(CENTER, SIZE + 2, 0.0.deg(), 90.0.deg()).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Inside)
                .build(),
        );
        let outside = Arc::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg()).into_styled(
            PrimitiveStyleBuilder::from(&style)
                .stroke_alignment(StrokeAlignment::Outside)
                .build(),
        );

        assert_eq!(center.bounding_box(), inside.bounding_box());
        assert_eq!(outside.bounding_box(), inside.bounding_box());

        // TODO: Uncomment when arc bounding box is fixed in #405
        // let mut display = MockDisplay::new();
        // center.draw(&mut display).unwrap();
        // assert_eq!(display.affected_area().unwrap(), center.bounding_box());
    }

    #[test]
    fn empty_bounding_box() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let empty = Arc::with_center(CENTER, SIZE - 4, 0.0.deg(), 90.0.deg())
            .into_styled::<BinaryColor>(PrimitiveStyle::new());

        assert_eq!(empty.bounding_box(), Rectangle::new(CENTER, Size::zero()));
    }
}
