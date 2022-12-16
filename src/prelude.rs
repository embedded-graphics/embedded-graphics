//! Prelude

#[doc(no_inline)]
pub use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    geometry::{Angle, AngleUnit, Dimensions, OriginDimensions, Point, Size},
    image::{ImageDrawable, ImageDrawableExt},
    iterator::{ContiguousIteratorExt, PixelIteratorExt},
    pixelcolor::{
        raw::{RawData, ToBytes as _},
        GrayColor, PixelColor, RgbColor, StorablePixelColor, WebColors,
    },
    primitives::{ContainsPoint, OffsetOutline, PointsIter, Primitive},
    transform::Transform,
    Drawable, Pixel,
};
