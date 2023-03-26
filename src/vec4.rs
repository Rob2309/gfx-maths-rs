use std::{fmt::Display, ops::Neg};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

#[derive(Debug, Clone, Copy, PartialEq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Display for Vec4 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y, z, w } = self;
        write!(f, "({x}, {y}, {z}, {w})")
    }
}

impl Vec4 {
    /// The zero vector (0, 0, 0)
    pub const ZERO: Self = Self::new(0.0, 0.0, 0.0, 0.0);
    /// The one vector (1, 1, 1)
    pub const ONE: Self = Self::new(1.0, 1.0, 1.0, 1.0);

    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Returns the square of the vector's length.
    ///
    /// Faster to compute than [`magnitude()`](Self::magnitude())
    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
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
        self.w /= m;
        self
    }
    /// Returns a normalized copy of `self`
    #[must_use]
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    /// Returns the dot product of `self` and `b`
    pub fn dot(&self, b: Vec4) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
}

/// Vec4 swizzles
impl Vec4 {
    swizzle!(x, x);
    swizzle!(x, y);
    swizzle!(x, z);
    swizzle!(x, w);
    swizzle!(y, x);
    swizzle!(y, y);
    swizzle!(y, z);
    swizzle!(y, w);
    swizzle!(z, x);
    swizzle!(z, y);
    swizzle!(z, z);
    swizzle!(z, w);
    swizzle!(w, x);
    swizzle!(w, y);
    swizzle!(w, z);
    swizzle!(w, w);

    swizzle!(x, x, x);
    swizzle!(x, x, y);
    swizzle!(x, x, z);
    swizzle!(x, x, w);
    swizzle!(x, y, x);
    swizzle!(x, y, y);
    swizzle!(x, y, z);
    swizzle!(x, y, w);
    swizzle!(x, z, x);
    swizzle!(x, z, y);
    swizzle!(x, z, z);
    swizzle!(x, z, w);
    swizzle!(x, w, x);
    swizzle!(x, w, y);
    swizzle!(x, w, z);
    swizzle!(x, w, w);
    swizzle!(y, x, x);
    swizzle!(y, x, y);
    swizzle!(y, x, z);
    swizzle!(y, x, w);
    swizzle!(y, y, x);
    swizzle!(y, y, y);
    swizzle!(y, y, z);
    swizzle!(y, y, w);
    swizzle!(y, z, x);
    swizzle!(y, z, y);
    swizzle!(y, z, z);
    swizzle!(y, z, w);
    swizzle!(y, w, x);
    swizzle!(y, w, y);
    swizzle!(y, w, z);
    swizzle!(y, w, w);
    swizzle!(z, x, x);
    swizzle!(z, x, y);
    swizzle!(z, x, z);
    swizzle!(z, x, w);
    swizzle!(z, y, x);
    swizzle!(z, y, y);
    swizzle!(z, y, z);
    swizzle!(z, y, w);
    swizzle!(z, z, x);
    swizzle!(z, z, y);
    swizzle!(z, z, z);
    swizzle!(z, z, w);
    swizzle!(z, w, x);
    swizzle!(z, w, y);
    swizzle!(z, w, z);
    swizzle!(z, w, w);
    swizzle!(w, x, x);
    swizzle!(w, x, y);
    swizzle!(w, x, z);
    swizzle!(w, x, w);
    swizzle!(w, y, x);
    swizzle!(w, y, y);
    swizzle!(w, y, z);
    swizzle!(w, y, w);
    swizzle!(w, z, x);
    swizzle!(w, z, y);
    swizzle!(w, z, z);
    swizzle!(w, z, w);
    swizzle!(w, w, x);
    swizzle!(w, w, y);
    swizzle!(w, w, z);
    swizzle!(w, w, w);

    swizzle!(x, x, x, x);
    swizzle!(x, x, x, y);
    swizzle!(x, x, x, z);
    swizzle!(x, x, x, w);
    swizzle!(x, x, y, x);
    swizzle!(x, x, y, y);
    swizzle!(x, x, y, z);
    swizzle!(x, x, y, w);
    swizzle!(x, x, z, x);
    swizzle!(x, x, z, y);
    swizzle!(x, x, z, z);
    swizzle!(x, x, z, w);
    swizzle!(x, x, w, x);
    swizzle!(x, x, w, y);
    swizzle!(x, x, w, z);
    swizzle!(x, x, w, w);
    swizzle!(x, y, x, x);
    swizzle!(x, y, x, y);
    swizzle!(x, y, x, z);
    swizzle!(x, y, x, w);
    swizzle!(x, y, y, x);
    swizzle!(x, y, y, y);
    swizzle!(x, y, y, z);
    swizzle!(x, y, y, w);
    swizzle!(x, y, z, x);
    swizzle!(x, y, z, y);
    swizzle!(x, y, z, z);
    swizzle!(x, y, z, w);
    swizzle!(x, y, w, x);
    swizzle!(x, y, w, y);
    swizzle!(x, y, w, z);
    swizzle!(x, y, w, w);
    swizzle!(x, z, x, x);
    swizzle!(x, z, x, y);
    swizzle!(x, z, x, z);
    swizzle!(x, z, x, w);
    swizzle!(x, z, y, x);
    swizzle!(x, z, y, y);
    swizzle!(x, z, y, z);
    swizzle!(x, z, y, w);
    swizzle!(x, z, z, x);
    swizzle!(x, z, z, y);
    swizzle!(x, z, z, z);
    swizzle!(x, z, z, w);
    swizzle!(x, z, w, x);
    swizzle!(x, z, w, y);
    swizzle!(x, z, w, z);
    swizzle!(x, z, w, w);
    swizzle!(x, w, x, x);
    swizzle!(x, w, x, y);
    swizzle!(x, w, x, z);
    swizzle!(x, w, x, w);
    swizzle!(x, w, y, x);
    swizzle!(x, w, y, y);
    swizzle!(x, w, y, z);
    swizzle!(x, w, y, w);
    swizzle!(x, w, z, x);
    swizzle!(x, w, z, y);
    swizzle!(x, w, z, z);
    swizzle!(x, w, z, w);
    swizzle!(x, w, w, x);
    swizzle!(x, w, w, y);
    swizzle!(x, w, w, z);
    swizzle!(x, w, w, w);
    swizzle!(y, x, x, x);
    swizzle!(y, x, x, y);
    swizzle!(y, x, x, z);
    swizzle!(y, x, x, w);
    swizzle!(y, x, y, x);
    swizzle!(y, x, y, y);
    swizzle!(y, x, y, z);
    swizzle!(y, x, y, w);
    swizzle!(y, x, z, x);
    swizzle!(y, x, z, y);
    swizzle!(y, x, z, z);
    swizzle!(y, x, z, w);
    swizzle!(y, x, w, x);
    swizzle!(y, x, w, y);
    swizzle!(y, x, w, z);
    swizzle!(y, x, w, w);
    swizzle!(y, y, x, x);
    swizzle!(y, y, x, y);
    swizzle!(y, y, x, z);
    swizzle!(y, y, x, w);
    swizzle!(y, y, y, x);
    swizzle!(y, y, y, y);
    swizzle!(y, y, y, z);
    swizzle!(y, y, y, w);
    swizzle!(y, y, z, x);
    swizzle!(y, y, z, y);
    swizzle!(y, y, z, z);
    swizzle!(y, y, z, w);
    swizzle!(y, y, w, x);
    swizzle!(y, y, w, y);
    swizzle!(y, y, w, z);
    swizzle!(y, y, w, w);
    swizzle!(y, z, x, x);
    swizzle!(y, z, x, y);
    swizzle!(y, z, x, z);
    swizzle!(y, z, x, w);
    swizzle!(y, z, y, x);
    swizzle!(y, z, y, y);
    swizzle!(y, z, y, z);
    swizzle!(y, z, y, w);
    swizzle!(y, z, z, x);
    swizzle!(y, z, z, y);
    swizzle!(y, z, z, z);
    swizzle!(y, z, z, w);
    swizzle!(y, z, w, x);
    swizzle!(y, z, w, y);
    swizzle!(y, z, w, z);
    swizzle!(y, z, w, w);
    swizzle!(y, w, x, x);
    swizzle!(y, w, x, y);
    swizzle!(y, w, x, z);
    swizzle!(y, w, x, w);
    swizzle!(y, w, y, x);
    swizzle!(y, w, y, y);
    swizzle!(y, w, y, z);
    swizzle!(y, w, y, w);
    swizzle!(y, w, z, x);
    swizzle!(y, w, z, y);
    swizzle!(y, w, z, z);
    swizzle!(y, w, z, w);
    swizzle!(y, w, w, x);
    swizzle!(y, w, w, y);
    swizzle!(y, w, w, z);
    swizzle!(y, w, w, w);
    swizzle!(z, x, x, x);
    swizzle!(z, x, x, y);
    swizzle!(z, x, x, z);
    swizzle!(z, x, x, w);
    swizzle!(z, x, y, x);
    swizzle!(z, x, y, y);
    swizzle!(z, x, y, z);
    swizzle!(z, x, y, w);
    swizzle!(z, x, z, x);
    swizzle!(z, x, z, y);
    swizzle!(z, x, z, z);
    swizzle!(z, x, z, w);
    swizzle!(z, x, w, x);
    swizzle!(z, x, w, y);
    swizzle!(z, x, w, z);
    swizzle!(z, x, w, w);
    swizzle!(z, y, x, x);
    swizzle!(z, y, x, y);
    swizzle!(z, y, x, z);
    swizzle!(z, y, x, w);
    swizzle!(z, y, y, x);
    swizzle!(z, y, y, y);
    swizzle!(z, y, y, z);
    swizzle!(z, y, y, w);
    swizzle!(z, y, z, x);
    swizzle!(z, y, z, y);
    swizzle!(z, y, z, z);
    swizzle!(z, y, z, w);
    swizzle!(z, y, w, x);
    swizzle!(z, y, w, y);
    swizzle!(z, y, w, z);
    swizzle!(z, y, w, w);
    swizzle!(z, z, x, x);
    swizzle!(z, z, x, y);
    swizzle!(z, z, x, z);
    swizzle!(z, z, x, w);
    swizzle!(z, z, y, x);
    swizzle!(z, z, y, y);
    swizzle!(z, z, y, z);
    swizzle!(z, z, y, w);
    swizzle!(z, z, z, x);
    swizzle!(z, z, z, y);
    swizzle!(z, z, z, z);
    swizzle!(z, z, z, w);
    swizzle!(z, z, w, x);
    swizzle!(z, z, w, y);
    swizzle!(z, z, w, z);
    swizzle!(z, z, w, w);
    swizzle!(z, w, x, x);
    swizzle!(z, w, x, y);
    swizzle!(z, w, x, z);
    swizzle!(z, w, x, w);
    swizzle!(z, w, y, x);
    swizzle!(z, w, y, y);
    swizzle!(z, w, y, z);
    swizzle!(z, w, y, w);
    swizzle!(z, w, z, x);
    swizzle!(z, w, z, y);
    swizzle!(z, w, z, z);
    swizzle!(z, w, z, w);
    swizzle!(z, w, w, x);
    swizzle!(z, w, w, y);
    swizzle!(z, w, w, z);
    swizzle!(z, w, w, w);
    swizzle!(w, x, x, x);
    swizzle!(w, x, x, y);
    swizzle!(w, x, x, z);
    swizzle!(w, x, x, w);
    swizzle!(w, x, y, x);
    swizzle!(w, x, y, y);
    swizzle!(w, x, y, z);
    swizzle!(w, x, y, w);
    swizzle!(w, x, z, x);
    swizzle!(w, x, z, y);
    swizzle!(w, x, z, z);
    swizzle!(w, x, z, w);
    swizzle!(w, x, w, x);
    swizzle!(w, x, w, y);
    swizzle!(w, x, w, z);
    swizzle!(w, x, w, w);
    swizzle!(w, y, x, x);
    swizzle!(w, y, x, y);
    swizzle!(w, y, x, z);
    swizzle!(w, y, x, w);
    swizzle!(w, y, y, x);
    swizzle!(w, y, y, y);
    swizzle!(w, y, y, z);
    swizzle!(w, y, y, w);
    swizzle!(w, y, z, x);
    swizzle!(w, y, z, y);
    swizzle!(w, y, z, z);
    swizzle!(w, y, z, w);
    swizzle!(w, y, w, x);
    swizzle!(w, y, w, y);
    swizzle!(w, y, w, z);
    swizzle!(w, y, w, w);
    swizzle!(w, z, x, x);
    swizzle!(w, z, x, y);
    swizzle!(w, z, x, z);
    swizzle!(w, z, x, w);
    swizzle!(w, z, y, x);
    swizzle!(w, z, y, y);
    swizzle!(w, z, y, z);
    swizzle!(w, z, y, w);
    swizzle!(w, z, z, x);
    swizzle!(w, z, z, y);
    swizzle!(w, z, z, z);
    swizzle!(w, z, z, w);
    swizzle!(w, z, w, x);
    swizzle!(w, z, w, y);
    swizzle!(w, z, w, z);
    swizzle!(w, z, w, w);
    swizzle!(w, w, x, x);
    swizzle!(w, w, x, y);
    swizzle!(w, w, x, z);
    swizzle!(w, w, x, w);
    swizzle!(w, w, y, x);
    swizzle!(w, w, y, y);
    swizzle!(w, w, y, z);
    swizzle!(w, w, y, w);
    swizzle!(w, w, z, x);
    swizzle!(w, w, z, y);
    swizzle!(w, w, z, z);
    swizzle!(w, w, z, w);
    swizzle!(w, w, w, x);
    swizzle!(w, w, w, y);
    swizzle!(w, w, w, z);
    swizzle!(w, w, w, w);
}

impl_op_ex!(+= |a: &mut Vec4, b: &Vec4| { a.x += b.x; a.y += b.y; a.z += b.z; a.w += b.w; });
impl_op_ex!(-= |a: &mut Vec4, b: &Vec4| { a.x -= b.x; a.y -= b.y; a.z -= b.z; a.w -= b.w; });
impl_op_ex!(*= |a: &mut Vec4, b: &Vec4| { a.x *= b.x; a.y *= b.y; a.z *= b.z; a.w *= b.w; });
impl_op_ex!(/= |a: &mut Vec4, b: &Vec4| { a.x /= b.x; a.y /= b.y; a.z /= b.z; a.w /= b.w; });

impl_op_ex!(*= |a: &mut Vec4, b: &f32| { a.x *= b; a.y *= b; a.z *= b; a.w *= b; });
impl_op_ex!(/= |a: &mut Vec4, b: &f32| { a.x /= b; a.y /= b; a.z /= b; a.w /= b; });

impl_op_ex!(+ |a: &Vec4, b: &Vec4| -> Vec4 { Vec4{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w } });
impl_op_ex!(-|a: &Vec4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.x - b.x,
        y: a.y - b.y,
        z: a.z - b.z,
        w: a.w - b.w,
    }
});
impl_op_ex!(*|a: &Vec4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.x * b.x,
        y: a.y * b.y,
        z: a.z * b.z,
        w: a.w * b.w,
    }
});
impl_op_ex!(/ |a: &Vec4, b: &Vec4| -> Vec4 { Vec4{x: a.x / b.x, y: a.y / b.y, z: a.z / b.z, w: a.w / b.w } });

impl_op_ex_commutative!(*|a: &Vec4, b: &f32| -> Vec4 {
    Vec4 {
        x: a.x * b,
        y: a.y * b,
        z: a.z * b,
        w: a.w * b,
    }
});
impl_op_ex!(/ |a: &Vec4, b: &f32| -> Vec4 { Vec4{x: a.x / b, y: a.y / b, z: a.z / b, w: a.w / b } });
impl_op_ex!(/ |a: &f32, b: &Vec4| -> Vec4 { Vec4{x: a / b.x, y: a / b.y, z: a / b.z, w: a / b.w } });

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        Vec4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

impl From<[f32; 4]> for Vec4 {
    fn from(d: [f32; 4]) -> Self {
        Self {
            x: d[0],
            y: d[1],
            z: d[2],
            w: d[3],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(3.0, 4.0, 5.0, 6.0);

        assert_eq!(
            -a,
            Vec4 {
                x: -1.0,
                y: -2.0,
                z: -3.0,
                w: -4.0
            }
        );

        assert_eq!(a.sqr_magnitude(), 30.0);
        assert_eq!(a.magnitude(), 30.0f32.sqrt());

        assert_eq!(a.dot(b), 50.0);

        assert_eq!(
            a + b,
            Vec4 {
                x: 4.0,
                y: 6.0,
                z: 8.0,
                w: 10.0
            }
        );
        assert_eq!(
            a - b,
            Vec4 {
                x: -2.0,
                y: -2.0,
                z: -2.0,
                w: -2.0
            }
        );
        assert_eq!(
            a * b,
            Vec4 {
                x: 3.0,
                y: 8.0,
                z: 15.0,
                w: 24.0
            }
        );
        assert_eq!(
            a / b,
            Vec4 {
                x: 1.0 / 3.0,
                y: 0.5,
                z: 3.0 / 5.0,
                w: 4.0 / 6.0
            }
        );

        assert_eq!(
            a * 2.0,
            Vec4 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
                w: 8.0
            }
        );
        assert_eq!(
            2.0 * a,
            Vec4 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
                w: 8.0
            }
        );

        assert_eq!(
            a / 2.0,
            Vec4 {
                x: 0.5,
                y: 1.0,
                z: 1.5,
                w: 2.0
            }
        );
        assert_eq!(
            2.0 / a,
            Vec4 {
                x: 2.0,
                y: 1.0,
                z: 2.0 / 3.0,
                w: 0.5
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
