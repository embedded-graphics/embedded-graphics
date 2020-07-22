//! Prelude

#[doc(no_inline)]
pub use crate::{
    draw_target::DrawTargetExt,
    fonts::Font,
    geometry::{Angle, AngleUnit, Dimensions, Point, Size},
    image::{ImageDimensions, IntoPixelIter},
    pixel_iterator::{IntoPixels, PixelIteratorExt},
    pixelcolor::{raw::RawData, GrayColor, IntoStorage, PixelColor, RgbColor},
    primitives::{ContainsPoint, Primitive},
    style::StyledPrimitiveAreas,
    transform::Transform,
    DrawTarget, Drawable, Pixel,
};
