use super::real;
use crate::geometry::Real;
use core::f32::consts::PI;
use core::ops::{Add, AddAssign, Neg, Sub, SubAssign};
#[cfg(not(feature = "fixed_point"))]
#[allow(unused_imports)]
use micromath::F32Ext;

pub(crate) mod angle_consts {
    use super::{real, Angle};

    pub(crate) const ANGLE_90DEG: Angle = Angle(real::FRAC_PI_2);
    pub(crate) const ANGLE_180DEG: Angle = Angle(real::PI);
    pub(crate) const ANGLE_360DEG: Angle = Angle(real::TAU);
}

/// Angle.
///
/// `Angle` is used to define the value of an angle.
///
/// # Examples
///
/// ## Create an `Angle` from a value
///
/// ```rust
/// use embedded_graphics::geometry::{Angle, AngleUnit};
/// use core::f32::consts::PI;
///
/// // Create an angle using the `from_degrees` constructor method
/// let angle_a = Angle::from_degrees(10.0);
/// let angle_b = Angle::from_radians(PI);
///
/// // Angles can also be created using the [AngleUnit] trait
/// let angle_c = 30.0.deg();
/// let angle_d = PI.rad();
/// ```
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
#[cfg_attr(feature = "defmt", derive(::defmt::Format))]
pub struct Angle(Real);

impl Angle {
    /// Creates an angle defined in degrees.
    pub fn from_degrees(angle: f32) -> Self {
        Angle((angle * PI / 180.0).into())
    }

    /// Creates an angle defined in radians.
    pub fn from_radians(angle: f32) -> Self {
        Angle(angle.into())
    }

    /// Creates a zero degree angle.
    pub fn zero() -> Self {
        Angle(0.into())
    }

    /// Compute the absolute value of the angle.
    pub fn abs(self) -> Self {
        Angle(self.0.abs())
    }

    /// Normalize the angle to less than one full rotation (ie. in the range 0..360).
    pub fn normalize(self) -> Self {
        Angle(self.0.rem_euclid((2.0 * PI).into()))
    }

    /// Return numerical value of the angle in degree
    pub fn to_degrees(self) -> f32 {
        let angle: f32 = self.0.into();
        180.0 * angle / PI
    }

    /// Return numerical value of the angle in radian
    pub fn to_radians(self) -> f32 {
        self.0.into()
    }
}

/// AngleUnit trait.
///
/// `AngleUnit` is a trait to convert numbers into angle by appending .deg() or .rad()
/// to the number as if it was a unit.
///
/// # Examples
///
/// ## Create an `Angle` from a value using `AngleUnit`
///
/// ```rust
/// use embedded_graphics::geometry::AngleUnit;
/// use core::f32::consts::PI;
///
/// // Create an angle using the `AngleUnit` methods
/// let angle_a = 30.0.deg();
/// let angle_b = PI.rad();
/// ```
pub trait AngleUnit {
    /// Convert a number (interpreted as degrees) to an `Angle`.
    fn deg(self) -> Angle;

    /// Convert a number (interpreted as radians) to an `Angle`.
    fn rad(self) -> Angle;
}

impl AngleUnit for f32 {
    fn deg(self) -> Angle {
        Angle::from_degrees(self)
    }

    fn rad(self) -> Angle {
        Angle::from_radians(self)
    }
}

pub(crate) trait Trigonometry {
    /// Get the sine of the angle.
    fn sin(self) -> Real;

    /// Get the cosine of the angle.
    fn cos(self) -> Real;

    /// Get the tangent of the angle.
    fn tan(self) -> Option<Real>;
}

#[cfg(not(feature = "fixed_point"))]
impl Trigonometry for Angle {
    fn sin(self) -> Real {
        let angle: f32 = self.0.into();
        angle.sin().into()
    }

    fn cos(self) -> Real {
        let angle: f32 = self.0.into();
        angle.cos().into()
    }

    fn tan(self) -> Option<Real> {
        let angle: f32 = self.0.into();
        let tan = angle.tan();
        // FRAC_PI_2.tan() has no value, but the approximate method used by micromath actually return a huge
        // value which is > 20000000.0, so we check for this to decide that the angle was approximately
        // FRAC_PI_2 and that tan() has actually no value.
        if tan.is_nan() || tan.abs() > 20000000.0 {
            None
        } else {
            Some(tan.into())
        }
    }
}

#[cfg(feature = "fixed_point")]
impl Trigonometry for Angle {
    fn sin(self) -> Real {
        use fixed::types::I16F16;
        const SIN: [I16F16; 91] = [
            // Ideally we could make the compiler generate those values, but for now sin() is not a const fn,
            // so it can't be used here. Here is how it would look like:
            //   I16F16::from_bits((0f64.sin() * (1 << 16) as f64).round()))),
            I16F16::from_bits(0),
            I16F16::from_bits(1144),
            I16F16::from_bits(2287),
            I16F16::from_bits(3430),
            I16F16::from_bits(4572),
            I16F16::from_bits(5712),
            I16F16::from_bits(6850),
            I16F16::from_bits(7987),
            I16F16::from_bits(9121),
            I16F16::from_bits(10252),
            I16F16::from_bits(11380),
            I16F16::from_bits(12505),
            I16F16::from_bits(13626),
            I16F16::from_bits(14742),
            I16F16::from_bits(15855),
            I16F16::from_bits(16962),
            I16F16::from_bits(18064),
            I16F16::from_bits(19161),
            I16F16::from_bits(20252),
            I16F16::from_bits(21336),
            I16F16::from_bits(22415),
            I16F16::from_bits(23486),
            I16F16::from_bits(24550),
            I16F16::from_bits(25607),
            I16F16::from_bits(26656),
            I16F16::from_bits(27697),
            I16F16::from_bits(28729),
            I16F16::from_bits(29753),
            I16F16::from_bits(30767),
            I16F16::from_bits(31772),
            I16F16::from_bits(32768),
            I16F16::from_bits(33754),
            I16F16::from_bits(34729),
            I16F16::from_bits(35693),
            I16F16::from_bits(36647),
            I16F16::from_bits(37590),
            I16F16::from_bits(38521),
            I16F16::from_bits(39441),
            I16F16::from_bits(40348),
            I16F16::from_bits(41243),
            I16F16::from_bits(42126),
            I16F16::from_bits(42995),
            I16F16::from_bits(43852),
            I16F16::from_bits(44695),
            I16F16::from_bits(45525),
            I16F16::from_bits(46341),
            I16F16::from_bits(47143),
            I16F16::from_bits(47930),
            I16F16::from_bits(48703),
            I16F16::from_bits(49461),
            I16F16::from_bits(50203),
            I16F16::from_bits(50931),
            I16F16::from_bits(51643),
            I16F16::from_bits(52339),
            I16F16::from_bits(53020),
            I16F16::from_bits(53684),
            I16F16::from_bits(54332),
            I16F16::from_bits(54963),
            I16F16::from_bits(55578),
            I16F16::from_bits(56175),
            I16F16::from_bits(56756),
            I16F16::from_bits(57319),
            I16F16::from_bits(57865),
            I16F16::from_bits(58393),
            I16F16::from_bits(58903),
            I16F16::from_bits(59396),
            I16F16::from_bits(59870),
            I16F16::from_bits(60326),
            I16F16::from_bits(60764),
            I16F16::from_bits(61183),
            I16F16::from_bits(61584),
            I16F16::from_bits(61966),
            I16F16::from_bits(62328),
            I16F16::from_bits(62672),
            I16F16::from_bits(62997),
            I16F16::from_bits(63303),
            I16F16::from_bits(63589),
            I16F16::from_bits(63856),
            I16F16::from_bits(64104),
            I16F16::from_bits(64332),
            I16F16::from_bits(64540),
            I16F16::from_bits(64729),
            I16F16::from_bits(64898),
            I16F16::from_bits(65048),
            I16F16::from_bits(65177),
            I16F16::from_bits(65287),
            I16F16::from_bits(65376),
            I16F16::from_bits(65446),
            I16F16::from_bits(65496),
            I16F16::from_bits(65526),
            I16F16::from_bits(65536),
        ];
        let degree: i32 = (Real::from(180) * self.0 / real::PI).round().into();
        let degree = degree.rem_euclid(360) as usize;
        let sin = if degree <= 90 {
            SIN[degree]
        } else if degree <= 180 {
            SIN[180 - degree]
        } else if degree <= 270 {
            -SIN[degree - 180]
        } else {
            -SIN[360 - degree]
        };
        sin.into()
    }

    fn cos(self) -> Real {
        (self + angle_consts::ANGLE_90DEG).sin()
    }

    fn tan(self) -> Option<Real> {
        let cos = self.cos();
        if cos != Real::zero() {
            Some(self.sin() / cos)
        } else {
            None
        }
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle(self.0 + other.0)
    }
}

impl AddAssign for Angle {
    fn add_assign(&mut self, other: Angle) {
        self.0 += other.0;
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        Angle(self.0 - other.0)
    }
}

impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Angle) {
        self.0 -= other.0;
    }
}

impl Neg for Angle {
    type Output = Angle;

    fn neg(self) -> Angle {
        Angle(-self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::{approx_eq, ApproxEq, F32Margin};

    impl ApproxEq for Angle {
        type Margin = F32Margin;

        fn approx_eq<M: Into<Self::Margin>>(self, other: Self, margin: M) -> bool {
            self.0.approx_eq(other.0, margin.into())
        }
    }

    #[test]
    fn angles_can_be_added() {
        let left = Angle::from_degrees(10.0);
        let right = Angle::from_degrees(30.0);

        assert!(approx_eq!(
            Angle,
            left + right,
            Angle::from_degrees(40.0),
            epsilon = 0.0001
        ));
    }

    #[test]
    fn angles_can_be_subtracted() {
        let left = Angle::from_degrees(30.0);
        let right = Angle::from_degrees(10.0);

        assert!(approx_eq!(
            Angle,
            left - right,
            Angle::from_degrees(20.0),
            epsilon = 0.0001
        ));
    }

    #[test]
    fn angles_can_be_absoluted() {
        let angle = Angle::from_degrees(30.0).abs();
        assert_eq!(angle, Angle::from_degrees(30.0));

        let angle = Angle::from_degrees(-30.0).abs();
        assert_eq!(angle, Angle::from_degrees(30.0));
    }

    #[test]
    fn angle_unit() {
        assert_eq!(180.0.deg(), Angle::from_degrees(180.0));
        assert_eq!(PI.rad(), Angle::from_radians(PI));
    }

    #[test]
    fn from_radians() {
        assert_eq!(Angle(PI.into()), Angle::from_radians(PI));
    }

    #[test]
    fn to_radians() {
        let angle = Angle(PI.into()).to_radians();
        assert!(approx_eq!(f32, angle, PI, epsilon = 0.0001));
    }

    #[test]
    fn from_degrees() {
        let angle = Angle::from_degrees(180.0);
        assert!(approx_eq!(f32, angle.0.into(), PI, epsilon = 0.0001));
    }

    #[test]
    fn to_degrees() {
        let angle = Angle(PI.into()).to_degrees();
        assert!(approx_eq!(f32, angle, 180.0, epsilon = 0.001));
    }

    #[test]
    fn sin_correct() {
        let degree_sin_pairs = [
            (-90.0, -1.0),
            (-60.0, -0.86602540),
            (-45.0, -0.70710678),
            (-30.0, -0.5),
            (0.0, 0.0),
            (30.0, 0.5),
            (45.0, 0.70710678),
            (60.0, 0.86602540),
            (90.0, 1.0),
            (120.0, 0.86602540),
            (135.0, 0.70710678),
            (150.0, 0.5),
            (180.0, 0.0),
            (210.0, -0.5),
            (225.0, -0.70710678),
            (240.0, -0.86602540),
            (270.0, -1.0),
        ];

        for (angle, sin) in &degree_sin_pairs {
            assert!(approx_eq!(
                Real,
                angle.deg().sin(),
                (*sin).into(),
                epsilon = 0.0001
            ));
        }
    }

    #[test]
    fn cos_correct() {
        let degree_cos_pairs = [
            (-90.0, 0.0),
            (-60.0, 0.5),
            (-45.0, 0.70710678),
            (-30.0, 0.86602540),
            (0.0, 1.0),
            (30.0, 0.86602540),
            (45.0, 0.70710678),
            (60.0, 0.5),
            (90.0, 0.0),
            (120.0, -0.5),
            (135.0, -0.70710678),
            (150.0, -0.86602540),
            (180.0, -1.0),
            (210.0, -0.86602540),
            (225.0, -0.70710678),
            (240.0, -0.5),
            (270.0, -0.0),
        ];

        for (angle, cos) in &degree_cos_pairs {
            assert!(approx_eq!(
                Real,
                angle.deg().cos(),
                (*cos).into(),
                epsilon = 0.0001
            ));
        }
    }

    #[test]
    fn tan_correct() {
        let degree_tan_pairs = [
            (-60.0, -1.73205080),
            (-45.0, -1.0),
            (-30.0, -0.57735026),
            (0.0, 0.0),
            (30.0, 0.57735026),
            (45.0, 1.0),
            (60.0, 1.73205080),
        ];

        for (angle, tan) in &degree_tan_pairs {
            assert!(approx_eq!(
                Real,
                angle.deg().tan().unwrap(),
                (*tan).into(),
                epsilon = 0.0001
            ));
        }

        assert_eq!((-90.0.deg()).tan(), None);
        assert_eq!(90.0.deg().tan(), None);
    }
}
