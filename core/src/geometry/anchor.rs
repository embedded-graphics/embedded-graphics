/// Anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorPoint {
    /// Top left.
    TopLeft,
    /// Top center.
    TopCenter,
    /// Top right.
    TopRight,
    /// Center left.
    CenterLeft,
    /// Center.
    Center,
    /// Center right.
    CenterRight,
    /// Bottom left.
    BottomLeft,
    /// Bottom center.
    BottomCenter,
    /// Bottom right.
    BottomRight,
}

impl AnchorPoint {
    /// Creates an anchor point from an X and Y component.
    pub fn from_xy(x: AnchorX, y: AnchorY) -> Self {
        match (y, x) {
            (AnchorY::Top, AnchorX::Left) => AnchorPoint::TopLeft,
            (AnchorY::Top, AnchorX::Center) => AnchorPoint::TopCenter,
            (AnchorY::Top, AnchorX::Right) => AnchorPoint::TopRight,
            (AnchorY::Center, AnchorX::Left) => AnchorPoint::CenterLeft,
            (AnchorY::Center, AnchorX::Center) => AnchorPoint::Center,
            (AnchorY::Center, AnchorX::Right) => AnchorPoint::CenterRight,
            (AnchorY::Bottom, AnchorX::Left) => AnchorPoint::BottomLeft,
            (AnchorY::Bottom, AnchorX::Center) => AnchorPoint::BottomCenter,
            (AnchorY::Bottom, AnchorX::Right) => AnchorPoint::BottomRight,
        }
    }

    /// Returns the X axis component.
    pub fn x(self) -> AnchorX {
        match self {
            AnchorPoint::TopLeft | AnchorPoint::CenterLeft | AnchorPoint::BottomLeft => {
                AnchorX::Left
            }
            AnchorPoint::TopCenter | AnchorPoint::Center | AnchorPoint::BottomCenter => {
                AnchorX::Center
            }
            AnchorPoint::TopRight | AnchorPoint::CenterRight | AnchorPoint::BottomRight => {
                AnchorX::Right
            }
        }
    }

    /// Returns the Y axis component.
    pub fn y(self) -> AnchorY {
        match self {
            AnchorPoint::TopLeft | AnchorPoint::TopCenter | AnchorPoint::TopRight => AnchorY::Top,
            AnchorPoint::CenterLeft | AnchorPoint::Center | AnchorPoint::CenterRight => {
                AnchorY::Center
            }
            AnchorPoint::BottomLeft | AnchorPoint::BottomCenter | AnchorPoint::BottomRight => {
                AnchorY::Bottom
            }
        }
    }
}

/// X axis anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorX {
    /// Left.
    Left,
    /// Center.
    Center,
    /// Right.
    Right,
}

/// Y axis anchor point.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub enum AnchorY {
    /// Top.
    Top,
    /// Center.
    Center,
    /// Bottom.
    Bottom,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    const ANCHOR_TESTS: &[((AnchorY, AnchorX), AnchorPoint)] = &[
        ((AnchorY::Top, AnchorX::Left), AnchorPoint::TopLeft),
        ((AnchorY::Top, AnchorX::Center), AnchorPoint::TopCenter),
        ((AnchorY::Top, AnchorX::Right), AnchorPoint::TopRight),
        ((AnchorY::Center, AnchorX::Left), AnchorPoint::CenterLeft),
        ((AnchorY::Center, AnchorX::Center), AnchorPoint::Center),
        ((AnchorY::Center, AnchorX::Right), AnchorPoint::CenterRight),
        ((AnchorY::Bottom, AnchorX::Left), AnchorPoint::BottomLeft),
        ((AnchorY::Bottom, AnchorX::Center), AnchorPoint::BottomCenter),
        ((AnchorY::Bottom, AnchorX::Right), AnchorPoint::BottomRight),
    ];

    #[test]
    fn anchor_conversion() {
        for ((y, x), p) in ANCHOR_TESTS.iter().copied() {
            assert_eq!(p.x(), x);
            assert_eq!(p.y(), y);

            assert_eq!(AnchorPoint::from_xy(x, y), p);
        }
    }
}
