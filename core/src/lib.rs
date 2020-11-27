//! embedded-graphics-core contains the common components of [embedded-graphics] that can be used to
//! integrate embedded-graphics into display drivers, image libraries and other third party crates.
//!
//! Display driver crates, image decoding crates and other crates that wish to integrate
//! embedded-graphics should use embedded-graphics-core to provid embedded-graphics integration for
//! their crate. Conversely, consumers of embedded-graphics and these crates should use
//! [embedded-graphics] itself.
//!
//! Like any other crate, embedded-graphics-core will change over time, however it will change at a
//! much slower rate than embedded-graphics itself, and will likely release fewer breaking changes.
//! This will provide more stability and compatability for the weider embedded-graphics ecosystem,
//! whilst allowing non-core features of embedded-graphics to evolve at a faster pace.
//!
//! It is strongly recommended to use as much of embedded-graphics-core as possible when
//! implementing an integration.
//!
//! ## Common functionality
//!
//! * [`Pixel`] - A simple struct containing a [`Point`] and color defining a single pixel.
//! * [`Drawable`] - A trait that should be implemented for anything that is drawable to a
//!   [`DrawTarget`]. Examples include shapes, text, UI elements, etc.
//! * Geometry - [`Point`], [`Size`] and [`Rectangle`] provide ways of defining positions,
//!   dimensions and areas respectively.
//! * Dimensions - the [`Dimensions`] and [`OriginDimensions`] traits allow sizing of objects.
//! * The [`prelude`] reexports useful items to reduce boilerplate.
//!
//! # Colors
//!
//! The [`pixelcolor`] module provides various standard colors, from [`BinaryColor`] to [`Rgb888`].
//! See the [`pixelcolor`] module documentation for the complete list of color depths and formats
//! available.
//!
//! # Display driver authors
//!
//! See the [`DrawTarget`] documentation for examples on how to integrate embedded-graphics with a
//! display driver using the [`DrawTarget`] trait.
//!
//! # Images
//!
//! The [`ImageDrawable`] trait should be implemented for any image or image-like item, like a font
//! bitmap or spritemap.
//!
//! [embedded-graphics]: https://docs.rs/embedded-graphics
//! [`Pixel`]: ./drawable/struct.Pixel.html
//! [`Point`]:  ./geometry/struct.Point.html
//! [`Size`]:  ./geometry/struct.Size.html
//! [`Drawable`]: ./drawable/trait.Drawable.html
//! [`DrawTarget`]: ./draw_target/trait.DrawTarget.html
//! [`Rectangle`]: ./primitives/rectangle/struct.Rectangle.html
//! [`Dimensions`]:  ./geometry/trait.Dimensions.html
//! [`OriginDimensions`]: ./geometry/trait.OriginDimensions.html
//! [`prelude`]: ./prelude/index.html
//! [`pixelcolor`]: ./pixelcolor/index.html
//! [`BinaryColor`]: ./pixelcolor/enum.BinaryColor.html
//! [`Rgb888`]: ./pixelcolor/struct.Rgb888.html
//! [`ImageDrawable`]: ./image/image_drawable/trait.ImageDrawable.html

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/191fe7f8a0fedc713f9722b9dc59208dacadee7e/assets/logo.svg?sanitize=true"
)]
#![no_std]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(missing_copy_implementations)]
#![deny(trivial_casts)]
#![deny(trivial_numeric_casts)]
#![deny(unsafe_code)]
#![deny(unstable_features)]
#![deny(unused_import_braces)]
#![deny(unused_qualifications)]

pub mod draw_target;
mod drawable;
pub mod geometry;
pub mod image;
pub mod iterator;
pub mod pixelcolor;
pub mod prelude;
pub mod primitives;

pub use drawable::{Drawable, Pixel};

/// Trait to convert unsigned into signed integer.
#[doc(hidden)]
pub trait SaturatingCast<T> {
    /// Casts a unsigned integer into a positive value.
    ///
    /// If the value is too large the maximum positive value is returned instead.
    fn saturating_cast(self) -> T;

    /// Casts a unsigned integer into a negative value.
    ///
    /// If the value is too large the minimum negative value is returned instead.
    fn saturating_cast_neg(self) -> T;
}

impl SaturatingCast<i32> for u32 {
    fn saturating_cast(self) -> i32 {
        if self < 0x8000_0000 {
            self as i32
        } else {
            i32::max_value()
        }
    }

    fn saturating_cast_neg(self) -> i32 {
        if self < 0x8000_0000 {
            -(self as i32)
        } else {
            i32::min_value()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn saturating_cast() {
        assert_eq!(0u32.saturating_cast(), 0i32);
        assert_eq!(0u32.saturating_cast_neg(), 0i32);

        assert_eq!(1u32.saturating_cast(), 1i32);
        assert_eq!(1u32.saturating_cast_neg(), -1i32);

        assert_eq!(0x7FFF_FFFFu32.saturating_cast(), 0x7FFF_FFFFi32);
        assert_eq!(0x7FFF_FFFFu32.saturating_cast_neg(), -0x7FFF_FFFFi32);

        assert_eq!(0x8000_0000u32.saturating_cast(), 0x7FFF_FFFFi32);
        assert_eq!(0x8000_0000u32.saturating_cast_neg(), -0x8000_0000i32);

        assert_eq!(u32::max_value().saturating_cast(), 0x7FFF_FFFFi32);
        assert_eq!(u32::max_value().saturating_cast_neg(), -0x8000_0000i32);
    }
}
