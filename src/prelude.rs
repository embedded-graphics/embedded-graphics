//! Prelude

#[doc(no_inline)]
pub use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    fonts::MonospacedFont,
    geometry::{Angle, AngleUnit, Dimensions, OriginDimensions, Point, Size},
    image::{ImageDrawable, ImageDrawableExt},
    iterator::{ContiguousIteratorExt, IntoPixels, PixelIteratorExt},
    pixelcolor::{
        raw::{RawData, ToBytes as _},
        GrayColor, IntoStorage, PixelColor, RgbColor, WebColors,
    },
    primitives::{ContainsPoint, OffsetOutline, Primitive},
    style::StyledPrimitiveAreas,
    transform::Transform,
    Drawable, Pixel,
};
