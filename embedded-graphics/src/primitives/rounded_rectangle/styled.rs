use crate::{
    draw_target::DrawTarget,
    drawable::{Drawable, Pixel},
    geometry::Dimensions,
    iterator::IntoPixels,
    pixelcolor::PixelColor,
    primitives::{
        rounded_rectangle::{Points, RoundedRectangle},
        ContainsPoint, OffsetOutline, Rectangle,
    },
    style::{PrimitiveStyle, Styled, StyledPrimitiveAreas},
    SaturatingCast,
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
        let iter = if !styled.style.is_transparent() {
            Points::new(&styled.stroke_area())
        } else {
            Points::empty()
        };

        Self {
            iter,
            fill_area: styled.fill_area(),
            stroke_color: styled.style.stroke_color,
            fill_color: styled.style.fill_color,
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

impl<C> IntoPixels for &Styled<RoundedRectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    type Color = C;

    type Iter = StyledPixels<Self::Color>;

    fn into_pixels(self) -> Self::Iter {
        StyledPixels::new(self)
    }
}

impl<C> Drawable for Styled<RoundedRectangle, PrimitiveStyle<C>>
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

impl<C> Dimensions for Styled<RoundedRectangle, PrimitiveStyle<C>>
where
    C: PixelColor,
{
    fn bounding_box(&self) -> Rectangle {
        let offset = self.style.outside_stroke_width().saturating_cast();

        self.primitive.bounding_box().offset(offset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        drawable::Drawable,
        geometry::{Dimensions, Point, Size},
        iterator::IntoPixels,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::{rectangle::Rectangle, CornerRadii, OffsetOutline, Primitive},
        style::{PrimitiveStyleBuilder, StrokeAlignment},
    };

    #[test]
    fn transparent_style_no_render() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        )
        .into_styled(PrimitiveStyleBuilder::<BinaryColor>::new().build());

        assert!(rounded_rect.into_pixels().eq(core::iter::empty()));
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

        assert!(rounded_rect.into_pixels().eq(rect.into_pixels()));
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

    #[test]
    fn styled_dimensions() {
        let base = PrimitiveStyleBuilder::new()
            .stroke_width(10)
            .stroke_color(Rgb888::RED);

        let inside = base.stroke_alignment(StrokeAlignment::Inside).build();
        let outside = base.stroke_alignment(StrokeAlignment::Outside).build();
        let center = base.stroke_alignment(StrokeAlignment::Center).build();

        let item = RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(40, 20)),
            CornerRadii {
                top_left: Size::new(20, 20),
                top_right: Size::new(20, 20),
                bottom_right: Size::new(0, 0),
                bottom_left: Size::new(0, 0),
            },
        );

        assert_eq!(item.into_styled(inside).bounding_box(), item.bounding_box());
        assert_eq!(
            item.into_styled(outside).bounding_box(),
            item.bounding_box().offset(10)
        );
        assert_eq!(
            item.into_styled(center).bounding_box(),
            item.bounding_box().offset(5)
        );
    }
}
