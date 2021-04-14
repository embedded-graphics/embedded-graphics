use crate::{
    draw_target::DrawTarget,
    geometry::Dimensions,
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        line::{thick_points::ThickPoints, Line, StrokeOffset},
        PrimitiveStyle, Rectangle, Styled,
    },
    Drawable, Pixel, SaturatingCast,
};

/// Styled line iterator.
#[derive(Clone, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    stroke_color: Option<C>,
    line_iter: ThickPoints,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives::line) fn new(styled: &Styled<Line, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        // Note: stroke color will be None if stroke width is 0
        let stroke_color = style.effective_stroke_color();
        let stroke_width = style.stroke_width.saturating_cast();

        Self {
            stroke_color,
            line_iter: ThickPoints::new(&primitive, stroke_width),
        }
    }
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<C> IntoPixels for &Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<Line, PrimitiveStyle<C>>
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

impl<C> Dimensions for Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let (l, r) = self
            .primitive
            .extents(self.style.stroke_width, StrokeOffset::None);

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
        geometry::Point,
        mock_display::MockDisplay,
        pixelcolor::{Rgb888, RgbColor},
        primitives::{Primitive, PrimitiveStyleBuilder},
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

        let transparent_line =
            line.into_styled::<Rgb888>(PrimitiveStyleBuilder::new().stroke_width(10).build());
        let stroked_line = line.into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 10));

        assert_eq!(transparent_line.bounding_box(), stroked_line.bounding_box(),);
    }
}
