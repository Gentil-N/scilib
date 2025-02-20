//!
//! # Spherical coordinates
//! 
//! Spherical coordinates use the distance to the origin of the point, as well as two angles from reference axis.
//! In this implementation, we use the following convention:
//! - r: distance to origin, `[0, +∞`[
//! - theta: azimuth (longitude) of the point, `[0, 2π[`
//! - phi: elevation (latitude) of the point, `[0, π[`
//! 
//! Support conversion to and from Cartesian and Cylindrical coordinates.

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::f64::consts::{         // Using std lib constants
    PI,                         // Pi
    TAU                         // Tau
};

use std::ops::{                 // Implementing basic operations
    Add,                        // Addition
    Sub,                        // Subtraction
    Mul,                        // Multiplication
    MulAssign,                  // Assigning multiplication
    Div,                        // Division
    DivAssign,                  // Assigning division
    Neg                         // Negation
};

use std::fmt::{                 // Formatter display
    Display,                    // The display itself
    Result as DRes              // The associated result
};

use super::{                    // Using parts from the crate
    cartesian::Cartesian,       // Cartesian coordinates
    cylindrical::Cylindrical    // Cylindrical coordinates
};

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// # Spherical coordinates
/// 
/// Defined for 3D space.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Spherical {
    /// Radial distance
    pub r: f64,
    /// Longitude angle
    pub theta: f64,
    /// Latitude angle
    pub phi: f64
}

/// # Display for Spherical
/// 
/// Simply shows each value.
impl Display for Spherical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> DRes {
        write!(f, "r={} :: theta={}° :: phi={}°", self.r, self.theta.to_degrees(), self.phi.to_degrees())?;
        Ok(())
    }
}

impl Spherical {
    /// # Creates a new entity
    /// 
    /// Returns the same value as `Self::default()`, all elements are equal to zero.
    /// 
    /// ```
    /// # use scilib::coordinate::spherical::Spherical;
    /// let m = Spherical { r: 0.0, theta: 0.0, phi: 0.0 };
    /// let n = Spherical::new();
    /// let d = Spherical::default();
    /// 
    /// assert_eq!(m, n);
    /// assert_eq!(n, d);
    /// ```
    pub const fn new() -> Self {
        Self {
            r: 0.0,
            theta: 0.0,
            phi: 0.0
        }
    }

    /// # From the point
    /// 
    /// Creates a Spherical struct from three given points in space.
    /// 
    /// ```
    /// # use scilib::coordinate::spherical::Spherical;
    /// let m = Spherical { r: 1.0, theta: 0.12, phi: 2.8 };
    /// let f = Spherical::from(1, 0.12, 2.8);
    /// 
    /// assert_eq!(m, f);
    /// ```
    pub fn from<T, U, V>(r: T, theta: U, phi: V) -> Self
    where T: Into<f64>, U: Into<f64>, V: Into<f64> {
        Self {
            r: r.into(),
            theta: theta.into() % TAU,
            phi: phi.into() % PI
        }
    }

    /// # From the point (degrees)
    /// 
    /// Creates a SPherical struct from three given points in space, with the angles in degrees.
    /// 
    /// ```
    /// # use scilib::coordinate::spherical::Spherical;
    /// let m = Spherical { r: 1.0, theta: 45.0_f64.to_radians(), phi: 60.0_f64.to_radians() };
    /// let f = Spherical::from_degree(1, 45, 60);
    /// 
    /// assert_eq!(m, f);
    /// ```
    pub fn from_degree<T, U, V>(r: T, theta: U, phi: V) -> Self
    where T: Into<f64>, U: Into<f64>, V: Into<f64> {
        let td: f64 = theta.into();
        let pd: f64 = phi.into();
        Self {
            r: r.into(),
            theta: td.to_radians() % TAU,
            phi: pd.to_radians() % PI
        }
    }

    /// # From another coordinate system
    /// 
    /// Creates an Spherical struct from another coordinate system. Calls the `Into<Spherical>` method,
    /// which is verified in its implementation.
    /// 
    /// ```
    /// # use scilib::coordinate::cartesian::Cartesian;
    /// # use scilib::coordinate::spherical::Spherical;
    /// # use scilib::coordinate::cylindrical::Cylindrical;
    /// let ca: Cartesian = Cartesian::from(0, 12, 3.2);
    /// let cy: Cylindrical = Cylindrical::from_degree(1.2, 32, -3);
    /// let res1: Spherical = Spherical::from_coord(ca);
    /// let res2: Spherical = Spherical::from_coord(cy);
    /// 
    /// assert_eq!(res1, ca.into());
    /// assert_eq!(res2, cy.into());
    /// ```
    pub fn from_coord<T>(c: T) -> Self
    where T: Into<Self> {
        c.into()
    }

    /// # Distance between two points
    /// 
    /// ```
    /// # use std::f64::consts::SQRT_2;
    /// # use scilib::coordinate::spherical::Spherical;
    /// let s1 = Spherical::from_degree(SQRT_2, 45, 90);
    /// let s2 = Spherical::from_degree(SQRT_2, -45, 90);
    /// 
    /// assert_eq!(s1.distance(s2), 2.0);
    /// ```
    pub fn distance(&self, other: Self) -> f64 {
        let r1: f64 = self.r.powi(2);
        let r2: f64 = other.r.powi(2);
        let a1: f64 = self.phi.sin() * other.phi.sin() * (self.theta - other.theta).cos();
        let a2: f64 = self.phi.cos() * other.phi.cos();

        (r1 + r2 - 2.0 * r1 * r2 * (a1 + a2)).sqrt()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

/// # Conversion to cartesian coordinates
/// 
/// ```
/// # use std::f64::consts::SQRT_2;
/// # use scilib::coordinate::cartesian::Cartesian;
/// # use scilib::coordinate::spherical::Spherical;
/// let s = Spherical::from_degree(SQRT_2, 0, 45);
/// let conv: Cartesian = s.into();
/// let expected = Cartesian::from(1, 0, 1);
/// 
/// assert_eq!(conv.x, expected.x);
/// assert_eq!(conv.y, expected.y);
/// assert!((conv.z - expected.z).abs() < 1.0e-15);
/// 
/// let s = Spherical::from_degree(SQRT_2, 45, 90);
/// let conv: Cartesian = s.into();
/// let expected = Cartesian::from(1, 1, 0);
/// assert!((conv.x - expected.x).abs() < 1.0e-15);
/// assert_eq!(conv.y, expected.y);
/// assert!((conv.z - expected.z).abs() < 1.0e-15);
/// ```
impl Into<Cartesian> for Spherical {
    fn into(self) -> Cartesian {
        Cartesian {
            x: self.r * self.theta.cos() * self.phi.sin(),
            y: self.r * self.theta.sin() * self.phi.sin(),
            z: self.r * self.phi.cos()
        }
    }
}

/// # Conversion to cylindrical coordinates
/// 
/// ```
/// # use scilib::coordinate::cylindrical::Cylindrical;
/// # use scilib::coordinate::spherical::Spherical;
/// let s = Spherical::from_degree(5, 60, 30);
/// let conv: Cylindrical = s.into();
/// let expected = Cylindrical::from_degree(2.5, 60, 4.330127019);
/// 
/// assert!((conv.r - expected.r).abs() < 1.0e-15);
/// assert_eq!(conv.theta, expected.theta);
/// assert!((conv.z - expected.z).abs() < 1.0e-9);
/// 
/// let s = Spherical::from_degree(4, 30, 60);
/// let conv: Cylindrical = s.into();
/// let expected = Cylindrical::from_degree(3.464101615, 30, 2);
/// assert!((conv.r - expected.r).abs() < 1.0e-9);
/// assert_eq!(conv.theta, expected.theta);
/// assert!((conv.z - expected.z).abs() < 1.0e-9);
/// ```
impl Into<Cylindrical> for Spherical {
    fn into(self) -> Cylindrical {
        Cylindrical {
            r: self.r * self.phi.sin(),
            theta: self.theta,
            z: self.r * self.phi.cos()
        }
    }
}

/// # Addition
/// 
/// Converts the coordinate in cartesian for addition, then returns them as spherical.
impl<T: Into<Cartesian>> Add<T> for Spherical {
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        let s: Cartesian = self.into();
        let r: Cartesian = rhs.into();
        (s + r).into()
    }
}

/// # Subtraction
/// 
/// Converts the coordinate in cartesian for subtraction, then returns them as spherical.
impl<T: Into<Cartesian>> Sub<T> for Spherical {
    type Output = Self;
    fn sub(self, rhs: T) -> Self::Output {
        let s: Cartesian = self.into();
        let r: Cartesian = rhs.into();
        (s - r).into()
    }
}

/// # Scalar multiplication
/// 
/// Multiplies the radius by a scalar.
/// 
/// ```
/// # use scilib::coordinate::spherical::Spherical;
/// let s = Spherical::from(1, 0.2, 2.1);
/// let res = s * 2;
/// let expected = Spherical::from(2, 0.2, 2.1);
/// 
/// assert_eq!(res, expected);
/// ```
impl<T: Into<f64>> Mul<T> for Spherical {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let f: f64 = rhs.into();
        let res: Self = Self {
            r: self.r * f.abs(),
            theta: self.theta,
            phi: self.phi
        };

        // If the sign is negative, we need to flip the angles
        if f.is_sign_negative() {
            -res
        } else {
            res
        }
    }
}

/// # Assigning scalar multiplication
/// 
/// Multiplies the radius by a scalar in place.
/// 
/// ```
/// # use scilib::coordinate::spherical::Spherical;
/// let mut s = Spherical::from(1, 0.2, 2.1);
/// s *= 2;
/// let expected = Spherical::from(2, 0.2, 2.1);
/// 
/// assert_eq!(s, expected);
/// ```
impl<T: Into<f64>> MulAssign<T> for Spherical {
    fn mul_assign(&mut self, rhs: T) {
        let f: f64 = rhs.into();
        self.r *= f.abs();

        // If the sign is negative, we need to flip the angles
        if f.is_sign_negative() {
            self.theta = (self.theta + PI) % TAU;
            self.phi = (PI - self.phi) % PI;
        }
    }
}

/// # Scalar division
/// 
/// Divides the radius by a scalar.
/// 
/// ```
/// # use scilib::coordinate::spherical::Spherical;
/// let s = Spherical::from(2, 0.2, 2.1);
/// let res = s / 2;
/// let expected = Spherical::from(1, 0.2, 2.1);
/// 
/// assert_eq!(res, expected);
/// ```
impl<T: Into<f64>> Div<T> for Spherical {
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        let f: f64 = rhs.into();
        let res: Self = Self {
            r: self.r / f.abs(),
            theta: self.theta,
            phi: self.phi
        };

        // If the sign is negative, we need to flip the angles
        if f.is_sign_negative() {
            -res
        } else {
            res
        }
    }
}

/// # Assigning scalar division
/// 
/// Divides the radius by a scalar in place.
/// 
/// ```
/// # use scilib::coordinate::spherical::Spherical;
/// let mut s = Spherical::from(2, 0.2, 2.1);
/// s /= 2;
/// let expected = Spherical::from(1, 0.2, 2.1);
/// 
/// assert_eq!(s, expected);
/// ```
impl<T: Into<f64>> DivAssign<T> for Spherical {
    fn div_assign(&mut self, rhs: T) {
        let f: f64 = rhs.into();
        self.r /= f.abs();

        // If the sign is negative, we need to flip the angles
        if f.is_sign_negative() {
            self.theta = (self.theta + PI) % TAU;
            self.phi = (PI - self.phi) % PI;
        }
    }
}

/// # Negation
/// 
/// Going to the opposite point.
/// 
/// ```
/// # use scilib::coordinate::spherical::Spherical;
/// let c1 = Spherical::from_degree(2, 60, 30);
/// let c2 = -c1;
/// let expected = Spherical::from_degree(2, 240, 150);
/// 
/// assert_eq!(c2, expected);
/// ```
impl Neg for Spherical {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            r: self.r,
            theta: (self.theta + PI) % TAU,
            phi: (PI - self.phi) % PI
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
