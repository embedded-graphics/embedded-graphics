use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, Point},
    pixelcolor::PixelColor,
    primitives::{
        common::{Scanline, StyledScanline},
        rounded_rectangle::{points::Scanlines, RoundedRectangle},
        styled::{StyledDimensions, StyledDrawable, StyledPixels},
        PrimitiveStyle, Rectangle,
    },
    Pixel,
};
use az::SaturatingAs;

use super::RoundedRectangleContains;

/// Pixel iterator for each pixel in the rect border
#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub struct StyledPixelsIterator<C> {
    styled_scanlines: StyledScanlines,

    stroke_left: Scanline,
    fill: Scanline,
    stroke_right: Scanline,

    stroke_color: Option<C>,
    fill_color: Option<C>,
}

impl<C: PixelColor> StyledPixelsIterator<C> {
    pub(in crate::primitives) fn new(
        primitive: &RoundedRectangle,
        style: &PrimitiveStyle<C>,
    ) -> Self {
        let stroke_area = style.stroke_area(primitive);
        let fill_area = style.fill_area(primitive);

        Self {
            styled_scanlines: StyledScanlines::new(&stroke_area, &fill_area),
            stroke_left: Scanline::new_empty(0),
            fill: Scanline::new_empty(0),
            stroke_right: Scanline::new_empty(0),
            stroke_color: style.stroke_color,
            fill_color: style.fill_color,
        }
    }
}

impl<C: PixelColor> Iterator for StyledPixelsIterator<C> {
    type Item = Pixel<C>;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.stroke_color, self.fill_color) {
            (Some(stroke_color), None) => loop {
                if let Some(pixel) = self
                    .stroke_left
                    .next()
                    .or_else(|| self.stroke_right.next())
                    .map(|p| Pixel(p, stroke_color))
                {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.stroke_left = scanline.stroke_left();
                self.stroke_right = scanline.stroke_right();
            },
            (Some(stroke_color), Some(fill_color)) => loop {
                if let Some(pixel) = self
                    .stroke_left
                    .next()
                    .map(|p| Pixel(p, stroke_color))
                    .or_else(|| self.fill.next().map(|p| Pixel(p, fill_color)))
                    .or_else(|| self.stroke_right.next().map(|p| Pixel(p, stroke_color)))
                {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.stroke_left = scanline.stroke_left();
                self.fill = scanline.fill();
                self.stroke_right = scanline.stroke_right();
            },
            (None, Some(fill_color)) => loop {
                if let Some(pixel) = self.fill.next().map(|p| Pixel(p, fill_color)) {
                    return Some(pixel);
                }

                let scanline = self.styled_scanlines.next()?;
                self.fill = scanline.fill();
            },
            (None, None) => None,
        }
    }
}

impl<C: PixelColor> StyledPixels<PrimitiveStyle<C>> for RoundedRectangle {
    type Iter = StyledPixelsIterator<C>;

    fn pixels(&self, style: &PrimitiveStyle<C>) -> Self::Iter {
        StyledPixelsIterator::new(self, style)
    }
}

impl<C: PixelColor> StyledDrawable<PrimitiveStyle<C>> for RoundedRectangle {
    type Color = C;
    type Output = ();

    fn draw_styled<D>(
        &self,
        style: &PrimitiveStyle<C>,
        target: &mut D,
    ) -> Result<Self::Output, D::Error>
    where
        D: DrawTarget<Color = C>,
    {
        match (style.effective_stroke_color(), style.fill_color) {
            (Some(stroke_color), None) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke(target, stroke_color)?;
                }
            }
            (Some(stroke_color), Some(fill_color)) => {
                for scanline in
                    StyledScanlines::new(&style.stroke_area(self), &style.fill_area(self))
                {
                    scanline.draw_stroke_and_fill(target, stroke_color, fill_color)?;
                }
            }
            (None, Some(fill_color)) => {
                for scanline in Scanlines::new(&style.fill_area(self)) {
                    scanline.draw(target, fill_color)?;
                }
            }
            (None, None) => {}
        }

        Ok(())
    }
}

impl<C: PixelColor> StyledDimensions<PrimitiveStyle<C>> for RoundedRectangle {
    fn styled_bounding_box(&self, style: &PrimitiveStyle<C>) -> Rectangle {
        let offset = style.outside_stroke_width().saturating_as();

        self.bounding_box().offset(offset)
    }
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct StyledScanlines {
    scanlines: Scanlines,
    fill_area: RoundedRectangleContains,
}

impl StyledScanlines {
    pub fn new(stroke_area: &RoundedRectangle, fill_area: &RoundedRectangle) -> Self {
        Self {
            scanlines: Scanlines::new(stroke_area),
            fill_area: RoundedRectangleContains::new(fill_area),
        }
    }
}

impl Iterator for StyledScanlines {
    type Item = StyledScanline;

    fn next(&mut self) -> Option<Self::Item> {
        self.scanlines.next().map(|scanline| {
            if self.fill_area.rows.contains(&scanline.y) {
                let fill_start = scanline
                    .x
                    .clone()
                    .find(|x| self.fill_area.contains(Point::new(*x, scanline.y)))
                    .unwrap_or(scanline.x.start);

                let fill_end = scanline
                    .x
                    .clone()
                    .rfind(|x| self.fill_area.contains(Point::new(*x, scanline.y)))
                    .map(|x| x + 1)
                    .unwrap_or(scanline.x.end);

                StyledScanline::new(scanline.y, scanline.x, Some(fill_start..fill_end))
            } else {
                StyledScanline::new(scanline.y, scanline.x, None)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Dimensions, Point, Size},
        iterator::PixelIteratorExt,
        mock_display::MockDisplay,
        pixelcolor::{BinaryColor, Rgb888, RgbColor},
        primitives::{
            rectangle::Rectangle, CornerRadii, Primitive, PrimitiveStyleBuilder, StrokeAlignment,
        },
        Drawable,
    };

    #[test]
    fn transparent_style_no_render() {
        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(10, 20)),
            Size::new(4, 8),
        )
        .into_styled(PrimitiveStyleBuilder::<BinaryColor>::new().build());

        assert!(rounded_rect.pixels().eq(core::iter::empty()));
    }

    #[test]
    fn thin_line_zero_radius_equals_rectangle() {
        let style = PrimitiveStyleBuilder::new()
            .stroke_color(Rgb888::RED)
            .stroke_width(1)
            .fill_color(Rgb888::RED)
            .build();

        let mut expected = MockDisplay::new();
        Rectangle::new(Point::zero(), Size::new(20, 30))
            .into_styled(style)
            .draw(&mut expected)
            .unwrap();

        let rounded_rect = RoundedRectangle::with_equal_corners(
            Rectangle::new(Point::zero(), Size::new(20, 30)),
            Size::zero(),
        )
        .into_styled(style);

        let mut drawable = MockDisplay::new();
        rounded_rect.draw(&mut drawable).unwrap();
        drawable.assert_eq(&expected);

        let mut pixels = MockDisplay::new();
        rounded_rect.pixels().draw(&mut pixels).unwrap();
        pixels.assert_eq(&expected);
    }

    #[test]
    fn styled_unequal_corners() {
        let expected_pattern = &[
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
        ];

        let rounded_rect = RoundedRectangle::new(
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
        );

        let mut drawable = MockDisplay::new();
        rounded_rect.draw(&mut drawable).unwrap();
        drawable.assert_pattern(expected_pattern);

        let mut pixels = MockDisplay::new();
        rounded_rect.pixels().draw(&mut pixels).unwrap();
        pixels.assert_pattern(expected_pattern);
    }

    #[test]
    fn styled_unfilled() {
        let expected_pattern = &[
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
        ];

        let rounded_rect = RoundedRectangle::new(
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
        );

        let mut drawable = MockDisplay::new();
        rounded_rect.draw(&mut drawable).unwrap();
        drawable.assert_pattern(expected_pattern);

        let mut pixels = MockDisplay::new();
        rounded_rect.pixels().draw(&mut pixels).unwrap();
        pixels.assert_pattern(expected_pattern);
    }

    #[test]
    fn full_height_corners() {
        let expected_pattern = &[
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
        ];

        let rounded_rect = RoundedRectangle::new(
            Rectangle::new(Point::zero(), Size::new(40, 20)),
            CornerRadii {
                top_left: Size::new(20, 20),
                top_right: Size::new(20, 20),
                bottom_right: Size::new(0, 0),
                bottom_left: Size::new(0, 0),
            },
        )
        .into_styled(PrimitiveStyleBuilder::new().fill_color(Rgb888::RED).build());

        let mut drawable = MockDisplay::new();
        rounded_rect.draw(&mut drawable).unwrap();
        drawable.assert_pattern(expected_pattern);

        let mut pixels = MockDisplay::new();
        rounded_rect.pixels().draw(&mut pixels).unwrap();
        pixels.assert_pattern(expected_pattern);
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
            Rectangle::new(Point::new(10, 10), Size::new(40, 20)),
            CornerRadii {
                top_left: Size::new(20, 20),
                top_right: Size::new(20, 20),
                bottom_right: Size::new(0, 0),
                bottom_left: Size::new(0, 0),
            },
        );

        let center = item.into_styled(center);
        let inside = item.into_styled(inside);
        let outside = item.into_styled(outside);

        assert_eq!(center.bounding_box(), item.bounding_box().offset(5));
        assert_eq!(inside.bounding_box(), item.bounding_box());
        assert_eq!(outside.bounding_box(), item.bounding_box().offset(10));

        let mut display = MockDisplay::new();
        center.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), center.bounding_box());
        let mut display = MockDisplay::new();
        inside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), inside.bounding_box());
        let mut display = MockDisplay::new();
        outside.draw(&mut display).unwrap();
        assert_eq!(display.affected_area(), outside.bounding_box());
    }

    #[test]
    fn bounding_box_is_independent_of_colors() {
        let rect = RoundedRectangle::new(
            Rectangle::new(Point::new(5, 5), Size::new(11, 14)),
            CornerRadii {
                top_left: Size::new(20, 20),
                top_right: Size::new(20, 20),
                bottom_right: Size::new(0, 0),
                bottom_left: Size::new(0, 0),
            },
        );

        let transparent_rect = rect.into_styled(PrimitiveStyle::<Rgb888>::new());
        let filled_rect = rect.into_styled(PrimitiveStyle::with_fill(Rgb888::RED));

        assert_eq!(transparent_rect.bounding_box(), filled_rect.bounding_box(),);
    }
}
