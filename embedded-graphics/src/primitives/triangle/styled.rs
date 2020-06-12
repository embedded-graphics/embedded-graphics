use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    pixelcolor::PixelColor,
    primitives::triangle::{
        scanline_iterator::{PointType, ScanlineIterator},
        Triangle,
    },
    style::{PrimitiveStyle, Styled},
};

/// Pixel iterator for each pixel in the triangle border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPixels<C: PixelColor>
where
    C: PixelColor,
{
    iter: ScanlineIterator,
    fill_color: Option<C>,
    stroke_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<Triangle, PrimitiveStyle<C>>) -> Self {
        let iter = if !styled.style.is_transparent() {
            ScanlineIterator::new(&styled.primitive)
        } else {
            ScanlineIterator::empty()
        };

        Self {
            iter,
            fill_color: styled.style.fill_color,
            stroke_color: styled.style.effective_stroke_color(),
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        let Self {
            ref stroke_color,
            ref fill_color,
            ..
        } = self;

        self.iter.find_map(|(point_type, point)| {
            match point_type {
                PointType::Border => stroke_color.or(*fill_color),
                PointType::Inside => *fill_color,
            }
            .map(|c| Pixel(point, c))
        })
    }
}

impl<C> IntoIterator for &Styled<Triangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledPixels<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledPixels::new(self)
    }
}

impl<'a, C: 'a> Drawable<C> for &Styled<Triangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D: DrawTarget<Color = C>>(self, display: &mut D) -> Result<(), D::Error> {
        display.draw_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::Point,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::Primitive,
        style::PrimitiveStyleBuilder,
        transform::Transform,
    };

    #[test]
    fn unfilled_no_stroke_width_no_triangle() {
        let mut tri = Triangle::new(Point::new(2, 2), Point::new(4, 2), Point::new(2, 4))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 0))
            .into_iter();

        assert_eq!(tri.next(), None);
    }

    #[test]
    fn issue_308_infinite() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);

        Triangle::new(Point::new(10, 10), Point::new(20, 30), Point::new(30, -10))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    fn it_draws_filled_strokeless_tri() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Triangle::new(Point::new(2, 2), Point::new(2, 4), Point::new(4, 2))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        #[rustfmt::skip]
        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "     ",
                "     ",
                "  ###",
                "  ## ",
                "  #  ",
            ])
        );
    }

    #[test]
    fn stroke_fill_colors() {
        let mut display: MockDisplay<Rgb888> = MockDisplay::new();
        display.set_allow_overdraw(true);

        Triangle::new(Point::new(2, 2), Point::new(8, 2), Point::new(2, 8))
            .into_styled(
                PrimitiveStyleBuilder::new()
                    .stroke_width(1)
                    .stroke_color(Rgb888::RED)
                    .fill_color(Rgb888::GREEN)
                    .build(),
            )
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          ",
                "          ",
                "  RRRRRRR ",
                "  RGGGGR  ",
                "  RGGGR   ",
                "  RGGR    ",
                "  RGR     ",
                "  RR      ",
                "  R       ",
            ])
        );
    }

    #[test]
    fn off_screen() {
        let mut display: MockDisplay<BinaryColor> = MockDisplay::new();
        display.set_allow_overdraw(true);
        display.set_allow_out_of_bounds_drawing(true);

        Triangle::new(Point::new(5, 5), Point::new(10, 15), Point::new(15, -5))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display)
            .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "          #####",
                "         ######",
                "        ###### ",
                "       ####### ",
                "      ######## ",
                "     ######### ",
                "     ########  ",
                "      #######  ",
                "      #######  ",
                "       ######  ",
                "       #####   ",
                "        ####   ",
                "        ####   ",
                "         ###   ",
                "         ##    ",
                "          #    ",
            ])
        );
    }

    #[test]
    fn styled_off_screen_still_draws_points() {
        let off_screen = Triangle::new(Point::new(10, 10), Point::new(20, 20), Point::new(30, -30))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On));
        let on_screen = off_screen.translate(Point::new(0, 35));

        assert!(off_screen.into_iter().eq(on_screen
            .into_iter()
            .map(|Pixel(p, col)| Pixel(p - Point::new(0, 35), col))));
    }
}
