//! Geometry module.

mod angle;
mod real;

pub(crate) use angle::angle_consts;
pub(crate) use angle::Trigonometry;
pub use angle::{Angle, AngleUnit};
pub use embedded_graphics_core::geometry::{Dimensions, OriginDimensions, Point, Size};
pub(crate) use real::Real;
