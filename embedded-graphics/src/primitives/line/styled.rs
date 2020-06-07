use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    pixelcolor::PixelColor,
    primitives::line::{thick_points::ThickPoints, Line},
    style::{PrimitiveStyle, Styled},
};

/// Styled line iterator.
#[derive(Clone, Debug)]
pub struct StyledIterator<C>
where
    C: PixelColor,
{
    stroke_color: Option<C>,
    line_iter: ThickPoints,
}

impl<C: PixelColor> StyledIterator<C> {
    pub(in crate::primitives::line) fn new(styled: &Styled<Line, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        // Note: stroke color will be None if stroke width is 0
        let stroke_color = style.effective_stroke_color();

        Self {
            stroke_color,
            line_iter: ThickPoints::new(&primitive, style.stroke_width_i32()),
        }
    }
}

// [Bresenham's line algorithm](https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm)
impl<C: PixelColor> Iterator for StyledIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        self.line_iter
            .next()
            .map(|point| Pixel(point, stroke_color))
    }
}

impl<'a, C> IntoIterator for &'a Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledIterator::new(self)
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}
