use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    pixelcolor::PixelColor,
    primitives::{
        rounded_rectangle::{Points, RoundedRectangle},
        ContainsPoint,
    },
    style::{PrimitiveStyle, Styled},
};

/// Pixel iterator for each pixel in the rect border
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub struct StyledPixels<C>
where
    C: PixelColor,
{
    iter: Points,
    fill_area: RoundedRectangle,
    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C> StyledPixels<C>
where
    C: PixelColor,
{
    pub(in crate::primitives) fn new(styled: &Styled<RoundedRectangle, PrimitiveStyle<C>>) -> Self {
        let Styled { style, primitive } = styled;

        let iter = if !styled.style.is_transparent() {
            let stroke_area = primitive.expand(style.outside_stroke_width());
            Points::new(&stroke_area)
        } else {
            Points::empty()
        };

        let fill_area = primitive.shrink(style.inside_stroke_width());

        Self {
            iter,
            fill_area,
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C> Iterator for StyledPixels<C>
where
    C: PixelColor,
{
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        for point in &mut self.iter {
            let color = if self.fill_area.contains(point) {
                self.fill_color
            } else {
                self.stroke_color
            };

            if let Some(color) = color {
                return Some(Pixel(point, color));
            }
        }

        None
    }
}

impl<C> IntoIterator for &Styled<RoundedRectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Item = Pixel<C>;
    type IntoIter = StyledPixels<C>;

    fn into_iter(self) -> Self::IntoIter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable<C> for Styled<RoundedRectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn draw<D>(&self, display: &mut D) -> Result<(), D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        display.draw_iter(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::{Point, Size},
        mock_display::MockDisplay,
        pixel_iterator::PixelIterator,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::{rectangle::Rectangle, CornerRadii, Primitive},
        style::PrimitiveStyleBuilder,
    };

    #[test]
    fn transparent_style_no_render() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        )
        .into_styled(PrimitiveStyleBuilder::<BinaryColor>::new().build());

        assert!(rounded_rect.into_iter().eq(core::iter::empty()));
    }

    #[test]
    fn thin_line_zero_radius_equals_rectangle() {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .fill_color(Rgb888::RED)
            .build();

        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::zero(),
        )
        .into_styled(style);

        let rect = Rectangle::new(Point::zero(), Size::new(20, 30)).into_styled(style);

        assert!(rounded_rect.into_iter().eq(rect.pixels()));
    }

    #[test]
    fn styled_unequal_corners() {
        let mut display = MockDisplay::new();

        RoundedRectangle::new(
            Rectangle::new(Point::new_equal(2), Size::new(20, 20)),
            CornerRadii {
                top_left: Size::new(3, 4),
                top_right: Size::new(5, 6),
                bottom_right: Size::new(7, 8),
                bottom_left: Size::new(9, 10),
            },
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(5)
                .fill_color(Rgb888::RED)
                .stroke_color(Rgb888::GREEN)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "   GGGGGGGGGGGGGGGG     ",
                "  GGGGGGGGGGGGGGGGGGG   ",
                " GGGGGGGGGGGGGGGGGGGGG  ",
                "GGGGGGGGGGGGGGGGGGGGGGG ",
                "GGGGGGGGGGGGGGGGGGGGGGG ",
                "GGGGGRRRRRRRRRRRRRGGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGRRRRRRRRRRRRRRGGGGG",
                "GGGGGGRRRRRRRRRRRRRGGGGG",
                " GGGGGRRRRRRRRRRRRGGGGGG",
                " GGGGGGRRRRRRRRRRRGGGGG ",
                "  GGGGGGGRRRRRRRRGGGGGG ",
                "  GGGGGGGGGGGGGGGGGGGGG ",
                "   GGGGGGGGGGGGGGGGGGG  ",
                "    GGGGGGGGGGGGGGGGG   ",
                "      GGGGGGGGGGGGGG    ",
                "        GGGGGGGGGG      ",
            ])
        );
    }

    #[test]
    fn styled_unfilled() {
        let mut display = MockDisplay::new();

        RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(20, 20)),
            CornerRadii {
                top_left: Size::new(3, 4),
                top_right: Size::new(5, 6),
                bottom_right: Size::new(7, 8),
                bottom_left: Size::new(9, 10),
            },
        )
        .into_styled(
            PrimitiveStyleBuilder::new()
                .stroke_width(1)
                .stroke_color(Rgb888::BLUE)
                .build(),
        )
        .draw(&mut display)
        .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "  BBBBBBBBBBBBBBB   ",
                " B               B  ",
                "B                 B ",
                "B                 BB",
                "B                  B",
                "B                  B",
                "B                  B",
                "B                  B",
                "B                  B",
                "B                  B",
                "B                  B",
                "B                  B",
                "B                  B",
                " B                 B",
                " B                 B",
                " BB               B ",
                "  B               B ",
                "   BB            B  ",
                "    BB         BB   ",
                "      BBBBBBBBB     ",
            ])
        );
    }

    #[test]
    fn full_height_corners() {
        let mut display = MockDisplay::new();

        RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(40, 20)),
            CornerRadii {
                top_left: Size::new(20, 20),
                top_right: Size::new(20, 20),
                bottom_right: Size::new(0, 0),
                bottom_left: Size::new(0, 0),
            },
        )
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb888::RED).build())
        .draw(&mut display)
        .unwrap();

        assert_eq!(
            display,
            MockDisplay::from_pattern(&[
                "                RRRRRRRR                ",
                "            RRRRRRRRRRRRRRRR            ",
                "          RRRRRRRRRRRRRRRRRRRR          ",
                "         RRRRRRRRRRRRRRRRRRRRRR         ",
                "       RRRRRRRRRRRRRRRRRRRRRRRRRR       ",
                "      RRRRRRRRRRRRRRRRRRRRRRRRRRRR      ",
                "     RRRRRRRRRRRRRRRRRRRRRRRRRRRRRR     ",
                "    RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR    ",
                "    RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR    ",
                "   RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR   ",
                "  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR  ",
                "  RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR  ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                " RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR ",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
                "RRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRRR",
            ])
        );
    }
}
