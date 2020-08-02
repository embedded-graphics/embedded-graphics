use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, Size},
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        polyline,
        polyline::{thick_points::ThickPoints, Polyline},
        Primitive, Rectangle,
    },
    style::{PrimitiveStyle, Styled},
};

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
enum StyledIter<'a> {
    Thin(polyline::Points<'a>),
    Thick(ThickPoints<'a>),
}

/// Pixel iterator for each pixel in the line
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPixels<'a, C>
where
    C: PixelColor,
{
    stroke_color: Option<C>,
    line_iter: StyledIter<'a>,
}

impl<'a, C> StyledPixels<'a, C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Polyline<'a>, PrimitiveStyle<C>>) -> Self {
        let line_iter = if styled.style.stroke_width <= 1 {
            StyledIter::Thin(styled.primitive.points())
        } else {
            StyledIter::Thick(ThickPoints::new(
                styled.primitive.vertices,
                styled.style.stroke_width,
                styled.style.stroke_alignment,
            ))
        };

        StyledPixels {
            stroke_color: styled.style.effective_stroke_color(),
            line_iter,
        }
    }
}

impl<'a, C> Iterator for StyledPixels<'a, C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return none if stroke color is none
        let stroke_color = self.stroke_color?;

        match self.line_iter {
            StyledIter::Thin(ref mut it) => it.next(),
            StyledIter::Thick(ref mut it) => it.next(),
        }
        .map(|point| Pixel(point, stroke_color))
    }
}

impl<'a, C> IntoPixels for &Styled<Polyline<'a>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<'a, C>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<'a, C> Drawable for Styled<Polyline<'a>, PrimitiveStyle<C>>
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

impl<C> Dimensions for Styled<Polyline<'_>, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    // NOTE: Polyline currently ignores stroke width, so this delegates to the un-styled bounding
    // box impl.
    fn bounding_box(&self) -> Rectangle {
        if self.style.effective_stroke_color().is_some() {
            self.primitive.bounding_box()
        } else {
            Rectangle::new(self.primitive.bounding_box().center(), Size::zero())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::primitives::polyline::tests::SMALL;
    use crate::{
        drawable::Drawable,
        geometry::Point,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb565, RgbColor},
        primitives::Primitive,
        style::{PrimitiveStyle, PrimitiveStyleBuilder},
    };

    #[test]
    fn mock_display() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();

        Polyline::new(&SMALL)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                ",
                "                ",
                "     #         #",
                "    # ##     ## ",
                "   #    ## ##   ",
                "  #       #     ",
            ])
        );
    }

    #[test]
    fn empty_styled_iterators() {
        let points: [Point; 3] = [Point::new(2, 5), Point::new(3, 4), Point::new(4, 3)];

        // No stroke width = no pixels
        assert!(Polyline::new(&points)
            .into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 0))
            .into_pixels()
            .eq(core::iter::empty()));

        // No stroke color = no pixels
        assert!(Polyline::new(&points)
            .into_styled::<Rgb565>(PrimitiveStyleBuilder::new().stroke_width(1).build())
            .into_pixels()
            .eq(core::iter::empty()));
    }

    #[test]
    fn bounding_box() {
        let points: [Point; 3] = [Point::new(2, 5), Point::new(3, 4), Point::new(4, 3)];

        let pl = Polyline::new(&points);

        let styled = pl.into_styled(PrimitiveStyle::with_stroke(Rgb565::BLUE, 10));

        assert_eq!(styled.bounding_box(), pl.bounding_box());

        let mut display = MockDisplay::new();
        styled.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), styled.bounding_box());

        assert_eq!(
            pl.into_styled::<Rgb565>(PrimitiveStyle::new())
                .bounding_box(),
            Rectangle::new(pl.bounding_box().center(), Size::zero()),
            "transparent"
        );

        assert_eq!(
            pl.into_styled::<Rgb565>(PrimitiveStyle::with_fill(Rgb565::RED))
                .bounding_box(),
            Rectangle::new(pl.bounding_box().center(), Size::zero()),
            "filled"
        );
    }
}
