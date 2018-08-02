//! # Embedded graphics
//!
//! This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
//! following:
//!
//! * 1 bit-per-pixel images
//! * 8 bit-per-pixel images (downsampled to 1BPP currently)
//! * Primitives
//!     * Lines
//!     * Rectangles (and squares)
//!     * Circles
//! * Text with a 6x8 pixel font
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
//! You can also add your own objects by implementing `IntoIterator<Item = Pixel>` to create an
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

pub mod coord;
pub mod drawable;
pub mod fonts;
// pub mod image;
pub mod prelude;
// pub mod primitives;
pub mod transform;
pub mod unsignedcoord;

use drawable::Color;

/// The main trait of this crate. All graphics objects must implement it.
pub trait Drawing {
    /// Data type to store color
    type C : Clone + Copy;
    /// Draw an object from an iterator over its pixels
    fn draw<T>(&mut self, item_pixels: T)
    where
        T: Iterator<Item = drawable::Pixel<Color<Self::C>>>;
}
