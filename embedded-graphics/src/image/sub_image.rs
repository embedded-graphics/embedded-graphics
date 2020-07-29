use crate::{
    draw_target::DrawTarget,
    geometry::{Dimensions, OriginDimensions},
    image::ImageDrawable,
    pixelcolor::PixelColor,
    primitives::Rectangle,
    transform::Transform,
};
use core::marker::PhantomData;

/// Sub image.
///
/// A sub image is rectangular subsection of an [`ImageDrawable`]. It can, for example, be used to
/// draw individual sprites from a larger sprite sheet.
///
/// To create a sub image call the [`sub_image`] method on the parent [`ImageDrawable`].
///
/// # Examples
///
/// TODO: add example
///
/// [`ImageDrawable`]: trait.ImageDrawable.html
/// [`sub_image`]: trait.ImageDrawableExt.html#tymethod_sub_image
#[derive(Debug)]
pub struct SubImage<'a, C, D>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    D: ImageDrawable<C>,
{
    parent: &'a D,
    area: Rectangle,
    color_type: PhantomData<C>,
}

impl<'a, C, D> SubImage<'a, C, D>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    D: ImageDrawable<C>,
{
    pub(super) fn new(parent: &'a D, area: &Rectangle) -> Self {
        let area = parent.bounding_box().intersection(area);

        Self {
            parent,
            area,
            color_type: PhantomData,
        }
    }
}

impl<C, D> OriginDimensions for SubImage<'_, C, D>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    D: ImageDrawable<C>,
{
    fn size(&self) -> crate::prelude::Size {
        self.area.size
    }
}

impl<'a, C, D> ImageDrawable<C> for SubImage<'a, C, D>
where
    C: PixelColor + From<<C as PixelColor>::Raw>,
    D: ImageDrawable<C>,
{
    fn draw<DT>(&self, target: &mut DT) -> Result<(), DT::Error>
    where
        DT: DrawTarget<Color = C>,
    {
        self.parent.draw_sub_image(target, &self.area)
    }

    fn draw_sub_image<DT>(&self, target: &mut DT, area: &Rectangle) -> Result<(), DT::Error>
    where
        DT: DrawTarget<Color = C>,
    {
        let area = area.translate(self.area.top_left);

        self.parent.draw_sub_image(target, &area)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        geometry::{Point, Size},
        image::ImageDrawableExt,
        mock_display::MockDisplay,
        pixelcolor::BinaryColor,
    };

    struct MockImageDrawable {
        expected_area: Rectangle,
    }

    impl ImageDrawable<BinaryColor> for MockImageDrawable {
        fn draw<DT>(&self, _target: &mut DT) -> Result<(), DT::Error>
        where
            DT: DrawTarget<Color = BinaryColor>,
        {
            panic!("draw shouldn't have been called on MockImageDrawable")
        }

        fn draw_sub_image<DT>(&self, _target: &mut DT, area: &Rectangle) -> Result<(), DT::Error>
        where
            DT: DrawTarget<Color = BinaryColor>,
        {
            assert_eq!(area, &self.expected_area);

            Ok(())
        }
    }

    impl OriginDimensions for MockImageDrawable {
        fn size(&self) -> Size {
            Size::new(8, 10)
        }
    }

    #[test]
    fn sub_image() {
        let area = Rectangle::new(Point::new(2, 3), Size::new(3, 4));

        let image = MockImageDrawable {
            expected_area: area,
        };

        let mut display = MockDisplay::new();
        image.sub_image(&area).draw(&mut display).unwrap();
    }

    #[test]
    fn area_larger_than_parent() {
        let area = Rectangle::new(Point::new(-5, -5), Size::new(20, 20));

        let image = MockImageDrawable {
            expected_area: Rectangle::new(Point::zero(), Size::new(8, 10)),
        };

        let mut display = MockDisplay::new();
        image.sub_image(&area).draw(&mut display).unwrap();
    }

    #[test]
    fn sub_image_of_sub_image() {
        let area1 = Rectangle::new(Point::new(2, 3), Size::new(3, 4));
        let area2 = Rectangle::new(Point::new(1, 1), Size::new(2, 2));

        let image = MockImageDrawable {
            expected_area: Rectangle::new(area1.top_left + area2.top_left, area2.size),
        };

        let mut display = MockDisplay::new();
        image
            .sub_image(&area1)
            .sub_image(&area2)
            .draw(&mut display)
            .unwrap();
    }

    #[test]
    fn sub_image_of_sub_image_area_larger_than_parent() {
        let area1 = Rectangle::new(Point::new(2, 3), Size::new(3, 4));
        let area2 = Rectangle::new(Point::new(-10, -10), Size::new(20, 20));

        let image = MockImageDrawable {
            expected_area: area1,
        };

        let mut display = MockDisplay::new();
        image
            .sub_image(&area1)
            .sub_image(&area2)
            .draw(&mut display)
            .unwrap();
    }
}
