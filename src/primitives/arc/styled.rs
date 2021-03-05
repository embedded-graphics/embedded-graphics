use crate::{
    draw_target::DrawTarget,
    geometry::Dimensions,
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        arc::Arc,
        common::{DistanceIterator, PlaneSector},
        OffsetOutline, PrimitiveStyle, Rectangle, Styled,
    },
    Drawable, Pixel, SaturatingCast,
};

/// Pixel iterator for each pixel in the arc border
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: DistanceIterator,

    plane_sector: PlaneSector,

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

        let iter = if !styled.style.is_transparent() {
            // PERF: The distance iterator should use the smaller arc bounding box
            outside_edge.distances()
        } else {
            DistanceIterator::empty()
        };

        let plane_sector = PlaneSector::new(primitive.angle_start, primitive.angle_sweep);

        Self {
            iter,
            plane_sector,
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
        let plane_sector = self.plane_sector;

        self.iter
            .find(|(_, delta, distance)| {
                *distance < outer_threshold
                    && *distance >= inner_threshold
                    && plane_sector.contains(*delta)
            })
            .map(|(point, ..)| Pixel(point, stroke_color))
    }
}

impl<C> Drawable for Styled<Arc, PrimitiveStyle<C>>
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
        let offset = self.style.outside_stroke_width().saturating_cast();

        self.primitive.bounding_box().offset(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        draw_target::DrawTargetExt,
        geometry::{AnchorPoint, AngleUnit, Point, Size},
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
        primitives::{Circle, Primitive, PrimitiveStyle, PrimitiveStyleBuilder, StrokeAlignment},
    };

    // Check the rendering of a simple arc
    #[test]
    fn tiny_arc() -> Result<(), core::convert::Infallible> {
        let mut display = MockDisplay::new();

        Arc::new(Point::zero(), 7, 30.0.deg(), 120.0.deg())
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)?;

        display.assert_pattern(&[
            "  ###  ", //
            " #   # ", //
        ]);

        Ok(())
    }

    /// Draws arcs with +/-90° sweep angle and compares the result with drawing a quarter circle.
    #[test]
    fn quadrant_arcs() {
        let style = PrimitiveStyle::with_stroke(BinaryColor::On, 2);

        for &diameter in &[11, 12] {
            for &(angle_start, angle_sweep, anchor_point) in &[
                (0.0.deg(), 90.0.deg(), AnchorPoint::TopRight),
                (90.0.deg(), 90.0.deg(), AnchorPoint::TopLeft),
                (180.0.deg(), 90.0.deg(), AnchorPoint::BottomLeft),
                (270.0.deg(), 90.0.deg(), AnchorPoint::BottomRight),
                (0.0.deg(), -90.0.deg(), AnchorPoint::BottomRight),
                (90.0.deg(), -90.0.deg(), AnchorPoint::TopRight),
                (180.0.deg(), -90.0.deg(), AnchorPoint::TopLeft),
                (270.0.deg(), -90.0.deg(), AnchorPoint::BottomLeft),
            ] {
                let circle = Circle::new(Point::new(1, 1), diameter).into_styled(style);

                // Calculate a clip rectangle for the tested quadrant.
                let bounding_box = circle.bounding_box();
                let clip_rect = bounding_box
                    .resized((bounding_box.size + Size::new_equal(1)) / 2, anchor_point);

                // Draw expected display by clipping the circle to the quadrant.
                let mut expected = MockDisplay::new();
                circle.draw(&mut expected.clipped(&clip_rect)).unwrap();

                // Draw the arc.
                let mut display = MockDisplay::new();
                Arc::new(Point::new(1, 1), diameter, angle_start, angle_sweep)
                    .into_styled(style)
                    .draw(&mut display)
                    .unwrap();

                display.assert_eq_with_message(&expected, |f| {
                    write!(
                        f,
                        "diameter: {}, angle_start: {}, angle_sweep: {}",
                        diameter,
                        angle_start.to_degrees(),
                        angle_sweep.to_degrees()
                    )
                });
            }
        }
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

        display_inside.assert_eq(&display_center);
        display_outside.assert_eq(&display_center);
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
        // assert_eq!(display.affected_area(), center.bounding_box());
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        const CENTER: Point = Point::new(15, 15);
        const SIZE: u32 = 10;

        let arc = Arc::with_center(CENTER, SIZE, 0.0.deg(), 90.0.deg());

        let transparent_arc =
            arc.into_styled::<BinaryColor>(PrimitiveStyleBuilder::new().stroke_width(5).build());
        let stroked_arc = arc.into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 5));

        assert_eq!(transparent_arc.bounding_box(), stroked_arc.bounding_box(),);
    }
}
