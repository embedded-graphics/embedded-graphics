//! # Embedded graphics
//!
//! This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
//! following:
//!
//! * [1 bit-per-pixel images](./image/struct.Image1BPP.html)
//! * [8 bits-per-pixel images](./image/struct.Image8BPP.html)
//! * [16 bits-per-pixel images](./image/struct.Image16BPP.html)
//! * [Primitives](./primitives/index.html)
//!     * Lines
//!     * Rectangles (and squares)
//!     * Circles
//! * [Text with multiple fonts](./fonts/index.html#types)
//!
//! A core goal is to do the above without using any buffers; the crate should work without a
//! dynamic memory allocator and without pre-allocating large chunks of memory. To achieve this, it
//! takes an `Iterator` based approach, where pixel values and positions are calculated on the fly,
//! with the minimum of saved state. This allows the consuming application to use far less RAM at
//! little to no performance penalty.
//!
//! To use this crate in a driver, you only need to implement the `Drawing` trait to start drawing
//! things.
//!
//! You can also add your own objects by implementing `IntoIterator<Item = Pixel<C>>` to create an
//! iterator that `Drawable#draw()` can consume.
//!
//! ## Crate features
//!
//! * `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
//! support to use as the `Coord` type. This should allow you to use most Nalgebra methods on
//! objects rendered by embedded_graphics.

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

#[cfg(feature = "nalgebra_support")]
extern crate nalgebra;
extern crate tinybmp;

pub mod coord;
pub mod dev;
pub mod drawable;
pub mod fonts;
pub mod image;
#[cfg(test)]
pub(crate) mod mock_display;
pub mod pixelcolor;
pub mod prelude;
pub mod primitives;
pub mod style;
pub mod transform;
pub mod unsignedcoord;

use crate::pixelcolor::PixelColor;

/// The main trait of this crate. All graphics objects must implement it.
pub trait Drawing<C>
where
    C: PixelColor + Clone,
{
    /// Draw an object from an iterator over its pixels
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel<C>>;
}
