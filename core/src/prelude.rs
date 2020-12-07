//! Prelude

#[doc(no_inline)]
pub use crate::{
    draw_target::{DrawTarget, DrawTargetExt},
    drawable::{Drawable, Pixel},
    geometry::{Dimensions, OriginDimensions, Point, Size},
    image::ImageDrawable,
    iterator::{ContiguousIteratorExt, IntoPixels, PixelIteratorExt},
    pixelcolor::{
        raw::{RawData, ToBytes as _},
        GrayColor, IntoStorage, PixelColor, RgbColor, WebColors,
    },
    primitives::PointsIter,
};
