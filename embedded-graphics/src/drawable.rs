//! `Drawable` trait and helpers

use super::unsignedcoord::UnsignedCoord;

/// Colour type
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color<C>
    where C: Clone + Copy + PartialEq
{
    value: C
}

impl<C> Color<C> 
    where C: Clone + Copy + PartialEq
{
    /// Creates a new color from a number
    pub fn new(color: C) -> Self {
        Self {
            value: color 
        }
    }

    /// Raw value of the color type
    pub fn value(self) -> C {
        self.value
    }
}

/// A single pixel
pub type Pixel<C> = (UnsignedCoord, Color<C>);

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
