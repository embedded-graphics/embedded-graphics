use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use float_cmp::{ApproxEq, F32Margin};
#[cfg(not(feature = "fixed_point"))]
#[allow(unused_imports)]
use micromath::F32Ext;

/// Real.
///
/// `Real` is used to store real numbers as either floating point or 32 bit fixed point if the
/// `fixed_point` cargo feature is enabled.
/// ```
pub(crate) use real_impl::{Real, FRAC_PI_2, PI, TAU};

#[cfg(not(feature = "fixed_point"))]
mod real_impl {
    use core::f32;

    #[allow(dead_code)]
    pub(crate) const FRAC_PI_2: Real = Real(f32::consts::FRAC_PI_2);
    pub(crate) const PI: Real = Real(f32::consts::PI);
    pub(crate) const TAU: Real = Real(2.0 * f32::consts::PI);

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub(crate) struct Real(pub(super) f32);

    impl From<f32> for Real {
        fn from(src: f32) -> Self {
            Self(src)
        }
    }

    impl From<i32> for Real {
        fn from(src: i32) -> Self {
            Self(src as f32)
        }
    }

    impl From<u32> for Real {
        fn from(src: u32) -> Self {
            Self(src as f32)
        }
    }

    impl From<Real> for f32 {
        fn from(src: Real) -> Self {
            src.0
        }
    }

    impl From<Real> for i32 {
        fn from(src: Real) -> Self {
            src.0 as i32
        }
    }

    impl From<Real> for u32 {
        fn from(src: Real) -> Self {
            src.0 as u32
        }
    }
}

#[cfg(feature = "fixed_point")]
mod real_impl {
    use fixed::types::I16F16;

    #[allow(dead_code)]
    pub(crate) const FRAC_PI_2: Real = Real(I16F16::from_bits(102944));
    pub(crate) const PI: Real = Real(I16F16::from_bits(205887));
    pub(crate) const TAU: Real = Real(I16F16::from_bits(411775));

    #[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
    pub(crate) struct Real(pub(super) I16F16);

    impl Real {
        pub(crate) const fn zero() -> Self {
            Self(I16F16::from_bits(0))
        }
    }

    impl From<I16F16> for Real {
        fn from(src: I16F16) -> Self {
            Self(src)
        }
    }

    impl From<f32> for Real {
        fn from(src: f32) -> Self {
            Self(I16F16::from_num(src))
        }
    }

    impl From<i32> for Real {
        fn from(src: i32) -> Self {
            Self(I16F16::from_num(src))
        }
    }

    impl From<u32> for Real {
        fn from(src: u32) -> Self {
            Self(I16F16::from_num(src))
        }
    }

    impl From<Real> for f32 {
        fn from(src: Real) -> Self {
            src.0.to_num::<f32>()
        }
    }

    impl From<Real> for i32 {
        fn from(src: Real) -> Self {
            src.0.round_to_zero().to_num::<i32>()
        }
    }

    impl From<Real> for u32 {
        fn from(src: Real) -> Self {
            src.0.to_num::<u32>()
        }
    }
}

impl Add for Real {
    type Output = Real;

    fn add(self, other: Real) -> Real {
        Self(self.0 + other.0)
    }
}

impl AddAssign for Real {
    fn add_assign(&mut self, other: Real) {
        self.0 += other.0;
    }
}

impl Sub for Real {
    type Output = Real;

    fn sub(self, other: Real) -> Real {
        Self(self.0 - other.0)
    }
}

impl SubAssign for Real {
    fn sub_assign(&mut self, other: Real) {
        self.0 -= other.0;
    }
}

impl Neg for Real {
    type Output = Real;

    fn neg(self) -> Real {
        Self(-self.0)
    }
}

impl Mul for Real {
    type Output = Real;

    fn mul(self, other: Real) -> Real {
        Self(self.0 * other.0)
    }
}

impl MulAssign for Real {
    fn mul_assign(&mut self, other: Real) {
        self.0 *= other.0
    }
}

impl Div for Real {
    type Output = Real;

    fn div(self, other: Real) -> Real {
        Self(self.0 / other.0)
    }
}

impl DivAssign for Real {
    fn div_assign(&mut self, other: Real) {
        self.0 /= other.0
    }
}

impl ApproxEq for Real {
    type Margin = F32Margin;

    fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
        let a: f32 = self.into();
        let b: f32 = other.into();
        a.approx_eq(b, margin.into())
    }
}

impl Real {
    pub(crate) fn abs(self) -> Self {
        Self(self.0.abs())
    }

    pub(crate) fn rem_euclid(self, rhs: Real) -> Self {
        let r = self.0 % rhs.0;
        if r < 0.0 {
            Real(r) + rhs.abs()
        } else {
            Real(r)
        }
    }

    #[allow(unused)]
    pub(crate) fn round(self) -> Self {
        Self(self.0.round())
    }
}
