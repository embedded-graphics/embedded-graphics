use crate::{
    draw_target::DrawTarget,
    pixelcolor::PixelColor,
    primitives::{
        line::{thick_points::ThickPoints, Line, StrokeOffset},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        PrimitiveStyle, Rectangle,
    },
    Pixel,
};
use az::SaturatingAs;

#[cfg(feature = "async_draw")]
use crate::draw_target::AsyncDrawTarget;
#[cfg(feature = "async_draw")]
use crate::primitives::AsyncStyledDrawable;

/// Styled line iterator.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct StyledPixelsIterator<C> {
    stroke_color: Option<C>,
    line_iter: ThickPoints,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives::line) fn new(primitive: &Line, style: &PrimitiveStyle<C>) -> Self {
        // Note: stroke color will be None if stroke width is 0
        let stroke_color = style.effective_stroke_color();
        let stroke_width = style.stroke_width.saturating_as();

        Self {
            stroke_color,
            line_iter: ThickPoints::new(primitive, stroke_width),
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for Line {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for Line {
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
        target.draw_iter(StyledPixelsIterator::new(self, style))
    }
}

#[cfg(feature = "async_draw")]
impl<C: PixelColor> AsyncStyledDrawable<PrimitiveStyle<C>> for Line {
    type Color = C;
    type Output = ();

    async fn draw_styled_async<D>(
        &self,
        style: &PrimitiveStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: AsyncDrawTarget<Color = C>,
    {
        target
            .draw_iter_async(StyledPixelsIterator::new(self, style))
            .await
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for Line {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let (l, r) = self.extents(style.stroke_width, StrokeOffset::None);

        let min = l
            .start
            .component_min(l.end)
            .component_min(r.start)
            .component_min(r.end);
        let max = l
            .start
            .component_max(l.end)
            .component_max(r.start)
            .component_max(r.end);

        Rectangle::with_corners(min, max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point},
        mock_display::MockDisplay,
        pixelcolor::{Rgb888, RgbColor},
        primitives::{Primitive, PrimitiveStyleBuilder},
        Drawable,
    };

    #[test]
    fn bounding_box() {
        let lines = [
            (
                Line::new(Point::new(10, 20), Point::new(10, 50)),
                "vertical",
            ),
            (
                Line::new(Point::new(20, 20), Point::new(50, 20)),
                "horizontal",
            ),
            (
                Line::new(Point::new(20, 20), Point::new(55, 55)),
                "diagonal",
            ),
            (Line::new(Point::new(20, 20), Point::new(55, 55)), "thin"),
            (
                Line::new(Point::new(40, 40), Point::new(13, 14)),
                "random angle 1",
            ),
            (
                Line::new(Point::new(30, 30), Point::new(12, 53)),
                "random angle 2",
            ),
        ];

        for (line, name) in lines.iter() {
            for thickness in 1..15 {
                let style = PrimitiveStyle::with_stroke(Rgb888::RED, thickness);
                let styled = line.into_styled(style);

                let mut display = MockDisplay::new();
                styled.draw(&mut display).unwrap();
                assert_eq!(
                    display.affected_area(),
                    styled.bounding_box(),
                    "{}, {} px",
                    name,
                    thickness
                );
            }
        }
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let line = Line::new(Point::new(5, 5), Point::new(11, 14));

        let transparent_line = line.into_styled(
            PrimitiveStyleBuilder::<Rgb888>::new()
                .stroke_width(10)
                .build(),
        );
        let stroked_line = line.into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 10));

        assert_eq!(transparent_line.bounding_box(), stroked_line.bounding_box(),);
    }
}
