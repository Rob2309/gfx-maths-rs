use std::{fmt::Display, ops::Neg};

use auto_ops::{impl_op_ex, impl_op_ex_commutative};

use crate::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Default for Vec2 {
    /// Creates a zero vector
    fn default() -> Self {
        Self::zero()
    }
}

impl Display for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y } = self;
        write!(f, "({x}, {y})")
    }
}

impl Vec2 {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates (0, 0)
    pub const fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Creates (1, 1)
    pub const fn one() -> Self {
        Self { x: 1.0, y: 1.0 }
    }

    /// Returns the square of the vector's length.
    ///
    /// Faster to compute than [`magnitude()`](Self::magnitude())
    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
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
        self
    }
    /// Returns a normalized copy of `self`
    #[must_use]
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    /// Returns the dot product of `self` and `b`
    pub fn dot(&self, b: Vec2) -> f32 {
        self.x * b.x + self.y * b.y
    }

    pub fn extend(&self, z: f32) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z,
        }
    }

    swizzle!(x, x);
    swizzle!(x, y);
    swizzle!(y, x);
    swizzle!(y, y);

    swizzle!(x, x, x);
    swizzle!(x, x, y);
    swizzle!(x, y, x);
    swizzle!(x, y, y);
    swizzle!(y, x, x);
    swizzle!(y, x, y);
    swizzle!(y, y, x);
    swizzle!(y, y, y);

    swizzle!(x, x, x, x);
    swizzle!(x, x, x, y);
    swizzle!(x, x, y, x);
    swizzle!(x, x, y, y);
    swizzle!(x, y, x, x);
    swizzle!(x, y, x, y);
    swizzle!(x, y, y, x);
    swizzle!(x, y, y, y);
    swizzle!(y, x, x, x);
    swizzle!(y, x, x, y);
    swizzle!(y, x, y, x);
    swizzle!(y, x, y, y);
    swizzle!(y, y, x, x);
    swizzle!(y, y, x, y);
    swizzle!(y, y, y, x);
    swizzle!(y, y, y, y);
}

impl_op_ex!(+= |a: &mut Vec2, b: &Vec2| { a.x += b.x; a.y += b.y; });
impl_op_ex!(-= |a: &mut Vec2, b: &Vec2| { a.x -= b.x; a.y -= b.y; });
impl_op_ex!(*= |a: &mut Vec2, b: &Vec2| { a.x *= b.x; a.y *= b.y; });
impl_op_ex!(/= |a: &mut Vec2, b: &Vec2| { a.x /= b.x; a.y /= b.y; });

impl_op_ex!(*= |a: &mut Vec2, b: &f32| { a.x *= b; a.y *= b });
impl_op_ex!(/= |a: &mut Vec2, b: &f32| { a.x /= b; a.y /= b });

impl_op_ex!(+ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x + b.x, y: a.y + b.y} });
impl_op_ex!(-|a: &Vec2, b: &Vec2| -> Vec2 {
    Vec2 {
        x: a.x - b.x,
        y: a.y - b.y,
    }
});
impl_op_ex!(*|a: &Vec2, b: &Vec2| -> Vec2 {
    Vec2 {
        x: a.x * b.x,
        y: a.y * b.y,
    }
});
impl_op_ex!(/ |a: &Vec2, b: &Vec2| -> Vec2 { Vec2{x: a.x / b.x, y: a.y / b.y} });

impl_op_ex_commutative!(*|a: &Vec2, b: &f32| -> Vec2 {
    Vec2 {
        x: a.x * b,
        y: a.y * b,
    }
});
impl_op_ex!(/ |a: &Vec2, b: &f32| -> Vec2 { Vec2{x: a.x / b, y: a.y / b} });
impl_op_ex!(/ |a: &f32, b: &Vec2| -> Vec2 { Vec2{x: a / b.x, y: a / b.y} });

impl Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Self::Output {
        Vec2 {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl From<[f32; 2]> for Vec2 {
    fn from(d: [f32; 2]) -> Self {
        Self { x: d[0], y: d[1] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators() {
        let a = Vec2::new(1.0, 2.0);
        let b = Vec2::new(3.0, 4.0);

        assert_eq!(-a, Vec2 { x: -1.0, y: -2.0 });

        assert_eq!(a.sqr_magnitude(), 5.0);
        assert_eq!(a.magnitude(), 5.0f32.sqrt());

        assert_eq!(a.dot(b), 11.0);

        assert_eq!(a + b, Vec2 { x: 4.0, y: 6.0 });
        assert_eq!(a - b, Vec2 { x: -2.0, y: -2.0 });
        assert_eq!(a * b, Vec2 { x: 3.0, y: 8.0 });
        assert_eq!(
            a / b,
            Vec2 {
                x: 1.0 / 3.0,
                y: 0.5
            }
        );

        assert_eq!(a * 2.0, Vec2 { x: 2.0, y: 4.0 });
        assert_eq!(2.0 * a, Vec2 { x: 2.0, y: 4.0 });

        assert_eq!(a / 2.0, Vec2 { x: 0.5, y: 1.0 });
        assert_eq!(2.0 / a, Vec2 { x: 2.0, y: 1.0 });

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
