use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    pixel_iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::line::{thick_points::ThickPoints, Line},
    style::{PrimitiveStyle, Styled},
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

        Self {
            stroke_color,
            line_iter: ThickPoints::new(&primitive, style.stroke_width_i32()),
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

impl<'a, C> IntoPixels for &'a Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable<C> for Styled<Line, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self.into_pixels())
    }
}
