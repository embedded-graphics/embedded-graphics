/// Horizontal alignment variants.
///
/// A `AlignH` can be applied to a [`TextStyle`] object to define how the text is aligned.
///
/// [`TextStyle`]: ./struct.TextStyle.html
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum AlignH {
    /// Left alignment.
    LEFT,
    /// Centered.
    CENTER,
    /// Right alignment.
    RIGHT,
}

impl Default for AlignH {
    fn default() -> Self {
        AlignH::LEFT
    }
}
