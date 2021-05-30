//! embedded-graphics-core contains the core components of [embedded-graphics] that are required to
//! add embedded-graphics support to display drivers, image libraries, text renderers and other
//! third party crates.
//!
//! This crate should only be used by crates that extend embedded-graphics.
//! Applications should instead depend on [embedded-graphics] itself.
//!
//! Like any other crate, embedded-graphics-core will change over time, however it will change at a
//! much slower rate than embedded-graphics itself, and will likely release fewer breaking changes.
//! This will provide more stability and compatability for the weider embedded-graphics ecosystem,
//! whilst allowing non-core features of embedded-graphics to evolve at a faster pace. The same
//! version of embedded-graphics-core may be used for multiple major versions of embedded-graphics.
//!
//! ## Core functionality
//!
//! * [`DrawTarget`] - By implementing a draw target for a display driver, all embedded-graphics drawables can be drawn to that display.
//! * [`Drawable`] - This trait can be implemented to make an object drawable to any [`DrawTarget`]. Examples include shapes, text, UI elements, etc.
//! * [`ImageDrawable`]
//! * Color types - see below.
//! * Geometry - [`Point`], [`Size`] and [`Rectangle`] provide ways of defining positions, dimensions and rectangular areas respectively.
//!
//! # Colors
//!
//! The [`pixelcolor`] module provides various standard color types, from [`BinaryColor`] to
//! [`Rgb888`]. See the [`pixelcolor`] module documentation for the complete list of color depths
//! and formats available.
//!
//! # Display drivers
//!
//! See the [`DrawTarget`] documentation for examples on how to integrate embedded-graphics with a
//! display driver using the [`DrawTarget`] trait.
//!
//! # Images
//!
//! The [`ImageDrawable`] trait should be implemented for any image or image-like item, for example
//! a spritemap.
//!
//! [embedded-graphics]: https://docs.rs/embedded-graphics
//! [`Pixel`]: ./drawable/struct.Pixel.html
//! [`Point`]: ./geometry/struct.Point.html
//! [`Size`]: ./geometry/struct.Size.html
//! [`Drawable`]: ./drawable/trait.Drawable.html
//! [`DrawTarget`]: ./draw_target/trait.DrawTarget.html
//! [`Rectangle`]: ./primitives/rectangle/struct.Rectangle.html
//! [`Dimensions`]: ./geometry/trait.Dimensions.html
//! [`OriginDimensions`]: ./geometry/trait.OriginDimensions.html
//! [`prelude`]: ./prelude/index.html
//! [`pixelcolor`]: ./pixelcolor/index.html
//! [`BinaryColor`]: ./pixelcolor/enum.BinaryColor.html
//! [`Rgb888`]: ./pixelcolor/struct.Rgb888.html
//! [`ImageDrawable`]: ./image/image_drawable/trait.ImageDrawable.html

#![doc(
    html_logo_url = "https://raw.githubusercontent.com/embedded-graphics/embedded-graphics/b225511f390c0ed9bc065eb67d05125845312148/assets/logo_core.svg?sanitize=true"
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
pub mod pixelcolor;
pub mod prelude;
pub mod primitives;

pub use drawable::{Drawable, Pixel};
