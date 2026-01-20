//! Transparent colors
//!
//! # Usage example
//!
//! TODO we need some implementation for a working usage.
//!
//! # Implementing transparent color types
//!
//! Transparent color types can be added by implementing the [`ColorBlend<C>`] trait with
//! an existing [`PixelColor`] as the underling color.
//! Please note that the transparent color may have the same [`PixelColor::Raw`] type as
//! its underlying color.
//!
//! A transparent color SHOULD implement [`From<C>`] so that a color converted
//! target can easily be created. This makes sure it is always possible to draw with an opaque color
//! over a `DrawTarget` that supports transparency.
//!
//! If a transparent color is implemented with an alpha channel (most of the time), it should also
//! implement `AlphaColor` to expose its alpha channel.
//!
//! If a transparent color has a natural underlying color, it should implement `HasAlphaColor` on
//! its underlying color to make it easy to create a blendable version of the color.
//!
//! # Implementation example
//! ```
//! use embedded_graphics::pixelcolor::{Gray8, ColorBlend};
//! use embedded_graphics::image::{Image, ImageRaw};
//! use embedded_graphics::prelude::*;
//! use embedded_graphics::pixelcolor::raw::RawU16;
//!
//! // A transparent color where alpha is handled as a u8
//! #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
//! pub struct TransparentGray {
//!     alpha: u8,
//!     gray: Gray8,
//! }
//!
//! // PixelColor is needed for TransparentColor
//! impl PixelColor for TransparentGray {
//!     // this implementation doesn't expose raw data
//!     type Raw = RawU16;
//! }
//!
//! // From<RawU16> is needed for PixelColor
//! impl From<RawU16> for TransparentGray {
//!     fn from(value: RawU16) -> Self {
//!         let inner = value.into_inner();
//!         let alpha = ((inner & 0xFF00) >> 8) as u8;
//!         TransparentGray { alpha, gray: Gray8::new((inner & 0xFF) as u8)}
//!     }
//! }
//!
//! // From<TransparentGray> is needed for PixelColor
//! impl From<TransparentGray> for RawU16 {
//!     fn from(value: TransparentGray) -> Self {
//!         let TransparentGray { alpha, gray } = value;
//!         RawU16::new((alpha as u16) << 8 + gray.luma())
//!     }
//! }
//!
//! // Implement required methods
//! impl ColorBlend<Gray8> for TransparentGray {
//!     fn blend_over(self, other: Gray8) -> Gray8 {
//!         let luma = (self.alpha as u16 * self.gray.luma() as u16) +
//!                    (255 - self.alpha as u16) * other.luma() as u16;
//!         Gray8::new((luma / 255) as u8)
//!     }
//! }
//!
//! // Make conversion available for `DrawTargetExt::color_converted`
//! impl From<Gray8> for TransparentGray {
//!     fn from(value: Gray8) -> Self {
//!         TransparentGray { alpha: 255, gray: value }
//!     }
//! }
//!
//! // Mock transparent handling display
//! pub struct Display {}
//! impl DrawTarget for Display {
//!     type Color = TransparentGray;
//!     type Error = core::convert::Infallible;
//!
//!     fn draw_iter<I>(&mut self, pixels: I) -> Result<(), core::convert::Infallible>
//!     where
//!         I: IntoIterator<Item = Pixel<Self::Color>>,
//!     {
//!         for Pixel(point, color) in pixels.into_iter() {
//!             // mock existing pixel as middle gray
//!             let current_color = Gray8::new(127);
//!             // blend the color with existing color
//!             let final_color = color.blend_over(current_color);
//!             // ... draw final color on display
//!         }
//!         Ok(())
//!     }
//! }
//! impl OriginDimensions for Display {
//!     fn size(&self) -> Size {
//!         Size::new(300, 300)
//!     }
//! }
//!
//! // The image data.
//! const DATA: &[u8] = &[
//!     0x80, 0xFF, 0xFF, 0x80, //
//!     0xFF, 0x00, 0x00, 0xFF, //
//!     0xFF, 0x00, 0x00, 0xFF, //
//!     0x80, 0xFF, 0xFF, 0x80, //
//! ];
//!
//! // Create a 4x4 non-transparent image
//! let raw_image = ImageRaw::<Gray8>::new(DATA, Size::new(4,4)).unwrap();
//! let image = Image::new(&raw_image, Point::zero());
//!
//! // Create a transparent handling display
//! let mut display = Display{};
//!
//! // Draw a non-transparent object on a transparent handling display
//! // Conversion is automatically handled by the `From` trait above and the `DrawTargetExt` trait
//! image.draw(&mut display.color_converted()).unwrap();
//! ```

use crate::pixelcolor::{Bgr666, Bgr888, Rgb444, Rgb666, Rgb888, PixelColor, RgbColor, IntoStorage};
use crate::pixelcolor::raw::{RawU16, RawU24, RawU32, RawData};
use crate::pixelcolor::{impl_rgb_color_common,const_rgb};
use core::fmt;

/// Transparent color trait.
///
/// This trait is used to represent color that handle transparency level.
/// They can blend with other colors, transparent or nor.
///
/// Transparent colors are a special kind of [`PixelColor`].
pub trait ColorBlend<C: PixelColor = Self>: PixelColor {
    /// Blend this color over another underlying opaque color
    fn blend_over(self, other: C) -> C;
}

/// Trait for transparent colors that expose their alpha channel
pub trait AlphaColor: PixelColor {
    ///Returns the alpha channel value.
    fn alpha(&self) -> u8;

    /// The maximum value of the alpha channel.
    const MAX_A: u8;
}

/// Trait used to create a transparent color that hold a pixelcolor with a compatible alpha channel
/// There can be only one matching AlphaColor for a given color.
///
/// Example:
/// ```ignore
/// # TODO we need some Rgba implementation for this test to pass
/// use embedded_graphics_core::pixelcolor::{Rgb888,HasAlphaColor,AlphaColor};
/// let color = Rgb888::new(0x0,0x80,0xFF);
/// let transparent = color.with_alpha(0);
/// ```
pub trait HasAlphaColor: PixelColor {
    /// Associated AlphaColor
    type AlphaColor: AlphaColor + ColorBlend<Self>;

    /// Create transparent color from opaque color
    fn with_alpha(self, alpha: u8) -> Self::AlphaColor;
}

macro_rules! argb_color {
    (
        $type:ident,
        $base_type:ty,
        $data_type:ty,
        $storage_type:ty,Argb =
        ($a_bits:expr, $r_bits:expr, $g_bits:expr, $b_bits:expr)
    ) => {
        impl_argb_color!(
            $type,
            $base_type,
            $data_type,
            $storage_type,
            ($a_bits, $r_bits, $g_bits, $b_bits),
            ($r_bits + $g_bits + $b_bits, $g_bits + $b_bits, $b_bits, 0),
            stringify!($type)
        );
    };

    (
        $type:ident,
        $base_type:ty,
        $data_type:ty,
        $storage_type:ty,Bgra =
        ($a_bits:expr, $r_bits:expr, $g_bits:expr, $b_bits:expr)
    ) => {
        impl_argb_color!(
            $type,
            $base_type,
            $data_type,
            $storage_type,
            ($a_bits, $r_bits, $g_bits, $b_bits),
            (0, $r_bits, $r_bits + $g_bits, $r_bits + $g_bits + $a_bits),
            stringify!($type)
        );
    };
}

/// Divides by Self::MAX_A (rounded to nearest bit)
/// input `a` is u16 that must be < 255*255, Result is u8 (rounded to the nearest result)
#[inline(always)]
fn r_div(a: u16, b: u8) -> u8 {
    // Round by pre-adding half denominator to keep value inside u16
    // this works because b <= 255, and a <= 255*255
    // thus  a + b/2 < 65536
    let r = (a + b as u16 / 2) / b as u16;
    r as u8
}

macro_rules! impl_argb_color {
    (
        $type:ident,
        $base_type:ty,
        $data_type:ty,
        $storage_type:ty,
        ($a_bits:expr, $r_bits:expr, $g_bits:expr, $b_bits:expr),
        ($a_pos:expr, $r_pos:expr, $g_pos:expr, $b_pos:expr),
        $type_str:expr
    ) => {
        impl_rgb_color_common!(
            $type, $data_type, $storage_type,
            ($r_bits, $g_bits, $b_bits),
            ($r_pos, $g_pos, $b_pos),
            MAX_A, $type_str
        );

        impl fmt::Debug for $type {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(
                    f,
                    "{}(r: {}, g: {}, b: {}, a: {})",
                    stringify!($type),
                    self.r(),
                    self.g(),
                    self.b(),
                    self.alpha()
                )
            }
        }

        #[cfg(feature = "defmt")]
        impl ::defmt::Format for $type {
            fn format(&self, f: ::defmt::Formatter) {
                ::defmt::write!(
                    f,
                    "{}(r: {=u8}, g: {=u8}, b: {=u8}, a: {=u8})",
                    stringify!($type),
                    self.r(),
                    self.g(),
                    self.b(),
                    self.alpha()
                )
            }
        }

        impl $type {
            const A_MASK: $storage_type = ($type::MAX_A as $storage_type) << $a_pos;
            const ARGB_MASK: $storage_type = Self::A_MASK | Self::R_MASK | Self::B_MASK | Self::G_MASK;

            /// create from channels
            pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
                // into_storage is not const, it would allow removing some code here
                let a_shifted = (a & Self::MAX_A) as $storage_type << $a_pos;
                let r_shifted = (r & Self::MAX_R) as $storage_type << $r_pos;
                let g_shifted = (g & Self::MAX_G) as $storage_type << $g_pos;
                let b_shifted = (b & Self::MAX_B) as $storage_type << $b_pos;
                Self(a_shifted | r_shifted | g_shifted | b_shifted)
            }

        }

        impl ColorBlend<$base_type> for $type {
            fn blend_over(self, other: $base_type) -> $base_type {
                let a1 = self.alpha() as u16;
                let a2 = Self::MAX_A as u16 - a1;

                let r0 = self.r() as u16 * a1 + other.r() as u16 * a2;
                let g0 = self.g() as u16 * a1 + other.g() as u16 * a2;
                let b0 = self.b() as u16 * a1 + other.b() as u16 * a2;

                let r = r_div(r0, Self::MAX_A);
                let g = r_div(g0, Self::MAX_A);
                let b = r_div(b0, Self::MAX_A);
                <$base_type>::new(r, g, b)
            }
        }

        impl ColorBlend<$type> for $type {
            fn blend_over(self, other: $type) -> $type {
                let a1 = self.alpha() as u16 ;
                let a2 = r_div(a1 * (Self::MAX_A as u16 - a1), Self::MAX_A) as u16;

                let r0 = self.r() as u16 * a1 + other.r() as u16 * a2;
                let g0 = self.g() as u16 * a1 + other.g() as u16 * a2;
                let b0 = self.b() as u16 * a1 + other.b() as u16 * a2;
                let a0 = self.alpha() + r_div(a1 * a2, Self::MAX_A);

                let r = r_div(r0, a0);
                let g = r_div(g0, a0);
                let b = r_div(b0, a0);
                <$type>::new(r, g, b, a0)
            }
        }

        impl AlphaColor for $type {
            fn alpha(&self) -> u8 {
                (self.0 >> $a_pos) as u8 & Self::MAX_A
            }

            const MAX_A: u8 = ((1usize << $a_bits) - 1) as u8;
        }

        impl HasAlphaColor for $base_type {
            type AlphaColor = $type;

            fn with_alpha(self, alpha: u8) -> Self::AlphaColor {
                $type(self.into_storage() | (alpha as $storage_type) << $a_pos)
            }
        }

        impl From<$base_type> for $type {
            fn from(value: $base_type) -> Self {
                // set alpha to max value
                $type(value.into_storage() | Self::A_MASK)
            }
        }
    }
}

argb_color!(Argb4444, Rgb444, RawU16, u16, Argb = (4, 4, 4, 4));
argb_color!(Argb6666, Rgb666, RawU24, u32, Argb = (6, 6, 6, 6));
argb_color!(Bgra6666, Bgr666, RawU24, u32, Bgra = (6, 6, 6, 6));
argb_color!(Argb8888, Rgb888, RawU32, u32, Argb = (8, 8, 8, 8));
argb_color!(Bgra8888, Bgr888, RawU32, u32, Bgra = (8, 8, 8, 8));

// No obvious impl
//argb_color!(???, Rgb332, RawU8, u8, Rgb = (3, 3, 2));
//argb_color!(Argb5555, Rgb555, RawU24, u32, Argb = (5, 5, 5, 5));
//argb_color!(Bgra5555, Bgr555, RawU24, u32, Bgra = (5, 5, 5, 5));
//argb_color!(Rgb565, Rgb565, RawU16, u16, Rgb = (5, 6, 5));
//argb_color!(Bgr565, Bgr565, RawU16, u16, Bgr = (5, 6, 5));

