//! Generic Color

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