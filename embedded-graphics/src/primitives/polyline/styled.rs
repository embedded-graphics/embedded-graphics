use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    pixelcolor::PixelColor,
    primitives::{polyline, polyline::Polyline, Primitive},
    style::{PrimitiveStyle, Styled},
};

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPolylineIterator<'a, C>
where
    C: PixelColor,
{
    stroke_color: Option<C>,
    line_iter: polyline::Points<'a>,
}

impl<'a, C> StyledPolylineIterator<'a, C>
where
    C: PixelColor,
{
    pub(crate) fn new(styled: &Styled<Polyline<'a>, PrimitiveStyle<C>>) -> Self {
        StyledPolylineIterator {
            stroke_color: styled.style.effective_stroke_color(),
            line_iter: styled.primitive.points(),
        }
    }
}

impl<'a, C> Iterator for StyledPolylineIterator<'a, C>
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

impl<'a, C: 'a> Drawable<C> for &Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self.into_iter())
    }
}
