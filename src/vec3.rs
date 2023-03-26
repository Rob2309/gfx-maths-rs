use std::{fmt::Display, ops::Neg};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::Vec4;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y, z } = self;
        write!(f, "({x}, {y}, {z})")
    }
}

impl Vec3 {
    /// The zero vector (0, 0, 0)
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0);
    /// The one vector (1, 1, 1)
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns the square of the vector's length.
    ///
    /// Faster to compute than [`magnitude()`](Self::magnitude())
    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the vector's length
    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    /// Normalizes `self` in place
    pub fn normalize(&mut self) -> &mut Self {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
        self.z /= m;
        self
    }
    /// Returns a normalized copy of `self`
    #[must_use]
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    /// Returns the dot product of `self` and `b`
    pub fn dot(&self, b: Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    /// Returns the cross product of `self` and `b`
    #[must_use]
    pub fn cross(&self, b: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }

    pub fn extend(&self, w: f32) -> Vec4 {
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.z,
            w,
        }
    }
}

/// Vec3 swizzles
impl Vec3 {
    swizzle!(x, x);
    swizzle!(x, y);
    swizzle!(x, z);
    swizzle!(y, x);
    swizzle!(y, y);
    swizzle!(y, z);
    swizzle!(z, x);
    swizzle!(z, y);
    swizzle!(z, z);

    swizzle!(x, x, x);
    swizzle!(x, x, y);
    swizzle!(x, x, z);
    swizzle!(x, y, x);
    swizzle!(x, y, y);
    swizzle!(x, y, z);
    swizzle!(x, z, x);
    swizzle!(x, z, y);
    swizzle!(x, z, z);
    swizzle!(y, x, x);
    swizzle!(y, x, y);
    swizzle!(y, x, z);
    swizzle!(y, y, x);
    swizzle!(y, y, y);
    swizzle!(y, y, z);
    swizzle!(y, z, x);
    swizzle!(y, z, y);
    swizzle!(y, z, z);
    swizzle!(z, x, x);
    swizzle!(z, x, y);
    swizzle!(z, x, z);
    swizzle!(z, y, x);
    swizzle!(z, y, y);
    swizzle!(z, y, z);
    swizzle!(z, z, x);
    swizzle!(z, z, y);
    swizzle!(z, z, z);

    swizzle!(x, x, x, x);
    swizzle!(x, x, x, y);
    swizzle!(x, x, x, z);
    swizzle!(x, x, y, x);
    swizzle!(x, x, y, y);
    swizzle!(x, x, y, z);
    swizzle!(x, x, z, x);
    swizzle!(x, x, z, y);
    swizzle!(x, x, z, z);
    swizzle!(x, y, x, x);
    swizzle!(x, y, x, y);
    swizzle!(x, y, x, z);
    swizzle!(x, y, y, x);
    swizzle!(x, y, y, y);
    swizzle!(x, y, y, z);
    swizzle!(x, y, z, x);
    swizzle!(x, y, z, y);
    swizzle!(x, y, z, z);
    swizzle!(x, z, x, x);
    swizzle!(x, z, x, y);
    swizzle!(x, z, x, z);
    swizzle!(x, z, y, x);
    swizzle!(x, z, y, y);
    swizzle!(x, z, y, z);
    swizzle!(x, z, z, x);
    swizzle!(x, z, z, y);
    swizzle!(x, z, z, z);
    swizzle!(y, x, x, x);
    swizzle!(y, x, x, y);
    swizzle!(y, x, x, z);
    swizzle!(y, x, y, x);
    swizzle!(y, x, y, y);
    swizzle!(y, x, y, z);
    swizzle!(y, x, z, x);
    swizzle!(y, x, z, y);
    swizzle!(y, x, z, z);
    swizzle!(y, y, x, x);
    swizzle!(y, y, x, y);
    swizzle!(y, y, x, z);
    swizzle!(y, y, y, x);
    swizzle!(y, y, y, y);
    swizzle!(y, y, y, z);
    swizzle!(y, y, z, x);
    swizzle!(y, y, z, y);
    swizzle!(y, y, z, z);
    swizzle!(y, z, x, x);
    swizzle!(y, z, x, y);
    swizzle!(y, z, x, z);
    swizzle!(y, z, y, x);
    swizzle!(y, z, y, y);
    swizzle!(y, z, y, z);
    swizzle!(y, z, z, x);
    swizzle!(y, z, z, y);
    swizzle!(y, z, z, z);
    swizzle!(z, x, x, x);
    swizzle!(z, x, x, y);
    swizzle!(z, x, x, z);
    swizzle!(z, x, y, x);
    swizzle!(z, x, y, y);
    swizzle!(z, x, y, z);
    swizzle!(z, x, z, x);
    swizzle!(z, x, z, y);
    swizzle!(z, x, z, z);
    swizzle!(z, y, x, x);
    swizzle!(z, y, x, y);
    swizzle!(z, y, x, z);
    swizzle!(z, y, y, x);
    swizzle!(z, y, y, y);
    swizzle!(z, y, y, z);
    swizzle!(z, y, z, x);
    swizzle!(z, y, z, y);
    swizzle!(z, y, z, z);
    swizzle!(z, z, x, x);
    swizzle!(z, z, x, y);
    swizzle!(z, z, x, z);
    swizzle!(z, z, y, x);
    swizzle!(z, z, y, y);
    swizzle!(z, z, y, z);
    swizzle!(z, z, z, x);
    swizzle!(z, z, z, y);
    swizzle!(z, z, z, z);
}

impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| { a.x /= b.x; a.y /= b.y; a.z /= b.z; });

impl_op_ex!(*= |a: &mut Vec3, b: &f32| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(/= |a: &mut Vec3, b: &f32| { a.x /= b; a.y /= b; a.z /= b; });

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z } });
impl_op_ex!(-|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
    }
});
impl_op_ex!(*|a: &Vec3, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
    }
});
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x / b.x, y: a.y / b.y, z: a.z / b.z } });

impl_op_ex_commutative!(*|a: &Vec3, b: &f32| -> Vec3 {
    Vec3 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
    }
});
impl_op_ex!(/ |a: &Vec3, b: &f32| -> Vec3 { Vec3{x: a.x / b, y: a.y / b, z: a.z / b } });
impl_op_ex!(/ |a: &f32, b: &Vec3| -> Vec3 { Vec3{x: a / b.x, y: a / b.y, z: a / b.z } });

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(d: [f32; 3]) -> Self {
        Self {
            x: d[0],
            y: d[1],
            z: d[2],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators() {
        let a = Vec3::new(1.0, 2.0, 3.0);
        let b = Vec3::new(3.0, 4.0, 5.0);

        assert_eq!(
            -a,
            Vec3 {
                x: -1.0,
                y: -2.0,
                z: -3.0
            }
        );

        assert_eq!(a.sqr_magnitude(), 14.0);
        assert_eq!(a.magnitude(), 14.0f32.sqrt());

        assert_eq!(a.dot(b), 26.0);

        assert_eq!(
            a + b,
            Vec3 {
                x: 4.0,
                y: 6.0,
                z: 8.0
            }
        );
        assert_eq!(
            a - b,
            Vec3 {
                x: -2.0,
                y: -2.0,
                z: -2.0
            }
        );
        assert_eq!(
            a * b,
            Vec3 {
                x: 3.0,
                y: 8.0,
                z: 15.0
            }
        );
        assert_eq!(
            a / b,
            Vec3 {
                x: 1.0 / 3.0,
                y: 0.5,
                z: 3.0 / 5.0
            }
        );

        assert_eq!(
            a * 2.0,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );
        assert_eq!(
            2.0 * a,
            Vec3 {
                x: 2.0,
                y: 4.0,
                z: 6.0
            }
        );

        assert_eq!(
            a / 2.0,
            Vec3 {
                x: 0.5,
                y: 1.0,
                z: 1.5
            }
        );
        assert_eq!(
            2.0 / a,
            Vec3 {
                x: 2.0,
                y: 1.0,
                z: 2.0 / 3.0
            }
        );

        let mut c = a;

        assert_eq!(c.normalized(), a / a.magnitude());

        c.normalize();
        assert_eq!(c, a / a.magnitude());

        c = a;
        c += b;
        assert_eq!(c, a + b);

        c = a;
        c -= b;
        assert_eq!(c, a - b);

        c = a;
        c *= b;
        assert_eq!(c, a * b);

        c = a;
        c /= b;
        assert_eq!(c, a / b);

        c = a;
        c *= 2.0;
        assert_eq!(c, a * 2.0);

        c = a;
        c /= 2.0;
        assert_eq!(c, a / 2.0);
    }
}
