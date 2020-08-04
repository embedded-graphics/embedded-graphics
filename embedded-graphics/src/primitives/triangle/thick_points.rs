use super::{scanline_iterator::PointType, Triangle};
use crate::{
    pixelcolor::PixelColor,
    primitives::{triangle::triangle_iterator::TriangleIterator, triangle::MathematicalPoints},
    style::PrimitiveStyle,
    Pixel,
};
use core::marker::PhantomData;

// TODO: Generalise name, move into more common folder path. Dedupe with polyline struct of the same name
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub(in crate::primitives) struct ThickPoints<C> {
    triangle_iter: TriangleIterator,
    points_iter: MathematicalPoints,
    triangle_type: PointType,
    stroke_color: Option<C>,
    fill_color: Option<C>,
    _c: PhantomData<C>,
}

impl<C> ThickPoints<C>
where
    C: PixelColor,
{
    pub fn new(triangle: &Triangle, style: PrimitiveStyle<C>) -> Self {
        let stroke_color = style.effective_stroke_color();
        let PrimitiveStyle {
            stroke_width,
            stroke_alignment,
            fill_color,
            ..
        } = style;

        let triangle_iter = TriangleIterator::new(
            triangle,
            stroke_width,
            stroke_alignment,
            fill_color.is_some(),
        );

        let mut self_ = Self {
            triangle_iter,
            points_iter: MathematicalPoints::empty(),
            triangle_type: PointType::Border,
            stroke_color,
            fill_color,
            _c: PhantomData,
        };

        self_.update_triangles();

        self_
    }

    fn update_triangles(&mut self) -> Option<()> {
        let (points_iter, triangle_type) = self
            .triangle_iter
            .next()
            .map(|(t, ty)| (t.mathematical_points(), ty))?;

        self.points_iter = points_iter;
        self.triangle_type = triangle_type;

        Some(())
    }
}

impl<C> Iterator for ThickPoints<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(point) = self.points_iter.next() {
            Some(Pixel(
                point,
                match self.triangle_type {
                    PointType::Border => self.stroke_color?,
                    PointType::Inside => self.fill_color?,
                },
            ))
        } else {
            self.update_triangles()?;

            self.next()
        }
    }
}
