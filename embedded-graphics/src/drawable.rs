//! `Drawable` trait and helpers

use super::unsignedcoord::UnsignedCoord;

/// Colour type
#[derive(Debug, Clone, Copy)]
pub struct Color<C>
    where C: Clone + Copy
{
    value: C
}

impl<C> Color<C> 
    where C: Clone + Copy
{
    /// Creates a new color from a number
    pub fn new(self, color: C) -> Self {
        Color {
            value: color 
        }
    }

    /// Raw value of the color type
    pub fn value(self) -> C {
        self.value
    }
}

impl<C> From<C> for Color<C>
    where C: Clone + Copy
{
    fn from(color: C) -> Self {
        Color { value: color }
    }
}

// impl<C> Into<C> for Color<C> 
//     where C: Clone + Copy
// {
//     fn into(self) -> C {
//         self.value
//     }
// }

/// A single pixel
pub type Pixel<C> = (UnsignedCoord, Color<C>);
// #[derive(Debug, Clone, Copy)]
// pub struct Pixel<C>(pub UnsignedCoord, pub Color<C>)
//     where C: Clone + Copy;

/// Marks an object as "drawable". Must be implemented for all graphics objects
pub trait Drawable {}
