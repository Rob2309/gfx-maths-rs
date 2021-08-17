use std::ops::Neg;

use auto_ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Vec4 {
    /// Creates a zero vector
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec4 {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self{x, y, z, w}
    }

    /// Creates (0, 0, 0, 0)
    pub const fn zero() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }

    /// Creates (1, 1, 1, 1)
    pub const fn one() -> Self {
        Self{x: 1.0, y: 1.0, z: 1.0, w: 1.0}
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
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    /// Returns the dot product of `self` and `b`
    pub fn dot(&self, b: Vec4) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z + self.w * b.w
    }
}

impl_op_ex!(+= |a: &mut Vec4, b: &Vec4| { a.x += b.x; a.y += b.y; a.z += b.z; a.w += b.w; });
impl_op_ex!(-= |a: &mut Vec4, b: &Vec4| { a.x -= b.x; a.y -= b.y; a.z -= b.z; a.w -= b.w; });
impl_op_ex!(*= |a: &mut Vec4, b: &Vec4| { a.x *= b.x; a.y *= b.y; a.z *= b.z; a.w *= b.w; });
impl_op_ex!(/= |a: &mut Vec4, b: &Vec4| { a.x /= b.x; a.y /= b.y; a.z /= b.z; a.w /= b.w; });

impl_op_ex!(*= |a: &mut Vec4, b: &f32| { a.x *= b; a.y *= b; a.z *= b; a.w *= b; });
impl_op_ex!(/= |a: &mut Vec4, b: &f32| { a.x /= b; a.y /= b; a.z /= b; a.w /= b; });

impl_op_ex!(+ |a: &Vec4, b: &Vec4| -> Vec4 { Vec4{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z, w: a.w + b.w } });
impl_op_ex!(- |a: &Vec4, b: &Vec4| -> Vec4 { Vec4{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z, w: a.w - b.w } });
impl_op_ex!(* |a: &Vec4, b: &Vec4| -> Vec4 { Vec4{x: a.x * b.x, y: a.y * b.y, z: a.z * b.z, w: a.w * b.w } });
impl_op_ex!(/ |a: &Vec4, b: &Vec4| -> Vec4 { Vec4{x: a.x / b.x, y: a.y / b.y, z: a.z / b.z, w: a.w / b.w } });

impl_op_ex_commutative!(* |a: &Vec4, b: &f32| -> Vec4 { Vec4{x: a.x * b, y: a.y * b, z: a.z * b, w: a.w * b } });
impl_op_ex!(/ |a: &Vec4, b: &f32| -> Vec4 { Vec4{x: a.x / b, y: a.y / b, z: a.z / b, w: a.w / b } });
impl_op_ex!(/ |a: &f32, b: &Vec4| -> Vec4 { Vec4{x: a / b.x, y: a / b.y, z: a / b.z, w: a / b.w } });

impl Neg for Vec4 {
    type Output = Vec4;
    fn neg(self) -> Self::Output {
        Vec4{x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operators() {
        let a = Vec4::new(1.0, 2.0, 3.0, 4.0);
        let b = Vec4::new(3.0, 4.0, 5.0, 6.0);
        
        assert_eq!(-a, Vec4{x: -1.0, y: -2.0, z: -3.0, w: -4.0});

        assert_eq!(a.sqr_magnitude(), 30.0);
        assert_eq!(a.magnitude(), 30.0f32.sqrt());

        assert_eq!(a.dot(b), 50.0);

        assert_eq!(a + b, Vec4{x: 4.0, y: 6.0, z: 8.0, w: 10.0});
        assert_eq!(a - b, Vec4{x: -2.0, y: -2.0, z: -2.0, w: -2.0});
        assert_eq!(a * b, Vec4{x: 3.0, y: 8.0, z: 15.0, w: 24.0});
        assert_eq!(a / b, Vec4{x: 1.0 / 3.0, y: 0.5, z: 3.0/5.0, w: 4.0/6.0});

        assert_eq!(a * 2.0, Vec4{x: 2.0, y: 4.0, z: 6.0, w: 8.0});
        assert_eq!(2.0 * a, Vec4{x: 2.0, y: 4.0, z: 6.0, w: 8.0});

        assert_eq!(a / 2.0, Vec4{x: 0.5, y: 1.0, z: 1.5, w: 2.0});
        assert_eq!(2.0 / a, Vec4{x: 2.0, y: 1.0, z: 2.0/3.0, w: 0.5});

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
