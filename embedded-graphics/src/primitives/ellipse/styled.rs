use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Point, Size},
    pixelcolor::PixelColor,
    primitives::ellipse::{
        compute_threshold, is_point_inside_ellipse, points_iterator::Points, Ellipse,
    },
    style::{PrimitiveStyle, Styled},
};

/// Pixel iterator for each pixel in the ellipse border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledEllipseIterator<C>
where
    C: PixelColor,
{
    iter: Points,
    outer_color: Option<C>,
    inner_size_sq: Size,
    inner_color: Option<C>,
    center: Point,
    threshold: u32,
}

impl<C> StyledEllipseIterator<C>
where
    C: PixelColor,
{
    pub(crate) fn new(styled: &Styled<Ellipse, PrimitiveStyle<C>>) -> Self {
        let Styled { primitive, style } = styled;

        let iter = if !styled.style.is_transparent() {
            let stroke_area = primitive.expand(style.outside_stroke_width());
            Points::new(&stroke_area)
        } else {
            Points::empty()
        };

        let fill_area = primitive.shrink(style.inside_stroke_width());
        let (inner_size_sq, threshold) = compute_threshold(fill_area.size);

        Self {
            iter,
            outer_color: styled.style.stroke_color,
            inner_size_sq,
            inner_color: styled.style.fill_color,
            center: styled.primitive.center_2x(),
            threshold,
        }
    }
}

impl<C> Iterator for StyledEllipseIterator<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            let inside_border = is_point_inside_ellipse(
                self.inner_size_sq,
                point * 2 - self.center,
                self.threshold,
            );

            let color = if inside_border {
                self.inner_color
            } else {
                self.outer_color
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
    }
}

impl<'a, C> IntoIterator for &'a Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledEllipseIterator<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledEllipseIterator::new(self)
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Ellipse, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}
