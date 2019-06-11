//! # Embedded graphics
//!
//! This crate aims to make drawing 2D graphics primitives super easy. It currently supports the
//! following:
//!
//! * [1 bit-per-pixel images](./image/type.Image1BPP.html)
//! * [8 bits-per-pixel images](./image/type.Image8BPP.html)
//! * [16 bits-per-pixel images](./image/type.Image16BPP.html)
//! * [BMP-format images](./image/struct.ImageBmp.html) (with `bmp` feature enabled)
//! * [TGA-format images](./image/struct.ImageTga.html) (with `tga` feature enabled)
//! * [Primitives](./primitives/index.html)
//!     * Lines
//!     * Rectangles (and squares)
//!     * Circles
//!     * Triangles
//! * [Text with multiple fonts](./fonts/index.html#types)
//!
//! A core goal is to do the above without using any buffers; the crate should work without a
//! dynamic memory allocator and without pre-allocating large chunks of memory. To achieve this, it
//! takes an `Iterator` based approach, where pixel values and positions are calculated on the fly,
//! with the minimum of saved state. This allows the consuming application to use far less RAM at
//! little to no performance penalty.
//!
//! To use this crate in a driver, you only need to implement the [`Drawing`](./trait.Drawing.html)
//! trait to start drawing things.
//!
//! If the device used supports partial updates where only a given range of pixels is updated, you
//! should also implement the [`SizedDrawing`](./trait.SizedDrawing.html) trait. This is similar to
//! `Drawing`, but has a bound on [`Dimensions`](./drawable/trait.Dimensions.html) which provides
//! methods for getting the bounding rectangle of the passed item to draw.
//!
//! You can also add your own objects by implementing `IntoIterator<Item = Pixel<C>>` to create an
//! iterator that `Drawable#draw()` can consume.
//!
//! ## Crate features
//!
//! * `nalgebra_support` - use the [Nalgebra](https://crates.io/crates/nalgebra) crate with `no_std`
//! support to use as the `Coord` type. This should allow you to use most Nalgebra methods on
//! objects rendered by embedded_graphics.
//! * `bmp` - use the [TinyBMP](https://crates.io/crates/tinybmp) crate for BMP image support.
//! * `tga` - use the [TinyTGA](https://crates.io/crates/tinytga) crate for TGA image support.

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

use crate::drawable::Dimensions;
use crate::pixelcolor::PixelColor;

/// The main trait of this crate. All graphics objects must implement it.
pub trait Drawing<C>
where
    C: PixelColor + Clone,
{
    /// Draw an object from an iterator over its pixels
    fn draw<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = drawable::Pixel<C>>;
}

/// Very similar to the `Drawing` trait, but accepts drawable objects which have a known size
///
/// This is useful for displays that implement some kind of partial update functionality, as only a
/// small square of pixels need to be sent as opposed to an entire framebuffer.
///
/// Library authors **should** implement `Drawing` along with `SizedDrawing` for maximum
/// compatibility, however some devices may only support one or the other.
///
/// Below is a contrived example that sets the draw area on an imaginary display and writes pixels to
/// it.
///
/// ```rust
/// # struct Display;
/// # impl Display {
/// #     pub fn set_pixel(&self, coord: UnsignedCoord, color: u8) {}
/// # }
/// use embedded_graphics::drawable::{Dimensions, Pixel};
/// use embedded_graphics::prelude::*;
/// use embedded_graphics::{Drawing, SizedDrawing};
///
/// impl SizedDrawing<u8> for Display where {
///     fn draw_sized<T>(&mut self, item: T)
///     where
///         T: IntoIterator<Item = Pixel<u8>> + Dimensions,
///     {
///         // Use `top_left()`, `size()`, etc methods defined on Dimensions to set draw area here
///
///         let offs = item.top_left().to_unsigned();
///
///         for Pixel(coord, color) in item {
///             // Undo any translations applied to this object
///             let coord = coord - offs;
///
///             self.set_pixel(coord, color)
///         }
///     }
/// }
/// ```
pub trait SizedDrawing<C>
where
    C: PixelColor + Clone,
{
    /// Draw an object from an iterator over its pixels
    fn draw_sized<T>(&mut self, item: T)
    where
        T: IntoIterator<Item = drawable::Pixel<C>> + Dimensions;
}
