//! Embedded Graphics Core
//!
//! DOC TODO

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
pub mod drawable;
pub mod geometry;
pub mod image_drawable;
pub mod iterator;
pub mod pixelcolor;
pub mod prelude;
pub mod rectangle;

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

pub use drawable::{Drawable, Pixel};
pub use rectangle::Rectangle;

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
