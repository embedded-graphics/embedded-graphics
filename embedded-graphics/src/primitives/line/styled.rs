use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        line::{thick_points::ThickPoints, Line},
        Rectangle,
    },
    style::{PrimitiveStyle, Styled},
    SaturatingCast,
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

    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
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
        if self.style.is_transparent() {
            Rectangle::new(self.primitive.bounding_box().center(), Size::zero())
        } else {
            let (l, r) = self.primitive.extents(self.style.stroke_width);

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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixelcolor::{Rgb888, RgbColor},
        primitives::{Primitive, Rectangle},
    };

    #[test]
    fn bounding_box() {
        let style = PrimitiveStyle::with_stroke(Rgb888::RED, 10);

        let vertical = Line::new(Point::new(10, 20), Point::new(10, 50)).into_styled(style);

        let horizontal = Line::new(Point::new(20, 20), Point::new(50, 20)).into_styled(style);

        let diagonal = Line::new(Point::new(20, 20), Point::new(60, 60)).into_styled(style);

        let thin = Line::new(Point::new(50, 50), Point::new(60, 60))
            .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1));

        assert_eq!(
            vertical.bounding_box(),
            Rectangle::new(Point::new(6, 20), Size::new(10, 31)),
            "vertical line"
        );

        assert_eq!(
            horizontal.bounding_box(),
            Rectangle::new(Point::new(20, 15), Size::new(31, 10)),
            "horizontal line"
        );

        assert_eq!(
            diagonal.bounding_box(),
            Rectangle::new(Point::new(17, 17), Size::new(47, 47)),
            "45deg line"
        );

        assert_eq!(
            thin.bounding_box(),
            Rectangle::with_corners(Point::new(50, 50), Point::new(60, 60)),
            "1px line"
        );

        let lines = [
            (vertical, "vertical"),
            (horizontal, "horizontal"),
            (diagonal, "diagonal"),
            (thin, "thin"),
            (
                Line::new(Point::new(40, 40), Point::new(13, 14))
                    .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1)),
                "random angle 1",
            ),
            (
                Line::new(Point::new(30, 30), Point::new(12, 63))
                    .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1)),
                "random angle 2",
            ),
        ];

        for (line, name) in lines.iter() {
            let mut display = MockDisplay::new();
            line.draw(&mut display).unwrap();
            assert_eq!(display.affected_area(), line.bounding_box(), "{}", name);
        }
    }

    #[test]
    fn transparent_bounding_box() {
        let line = Line::new(Point::new(5, 5), Point::new(11, 14))
            .into_styled::<Rgb888>(PrimitiveStyle::new());

        assert_eq!(
            line.bounding_box(),
            Rectangle::new(line.primitive.bounding_box().center(), Size::zero())
        );
    }
}
