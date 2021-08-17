use std::ops::Neg;

use auto_ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Default for Vec3 {
    /// Creates a zero vector
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self{x, y, z}
    }

    /// Creates (0, 0, 0)
    pub fn zero() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0}
    }

    /// Creates (1, 1, 1)
    pub fn one() -> Self {
        Self{x: 1.0, y: 1.0, z: 1.0}
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
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    /// Returns the dot product of `self` and `b`
    pub fn dot(&self, b: &Vec3) -> f32 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    /// Returns the cross product of `self` and `b`
    pub fn cross(&self, b: &Vec3) -> Vec3 {
        Vec3{
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }
}

impl_op_ex!(+= |a: &mut Vec3, b: &Vec3| { a.x += b.x; a.y += b.y; a.z += b.z; });
impl_op_ex!(-= |a: &mut Vec3, b: &Vec3| { a.x -= b.x; a.y -= b.y; a.z -= b.z; });
impl_op_ex!(*= |a: &mut Vec3, b: &Vec3| { a.x *= b.x; a.y *= b.y; a.z *= b.z; });
impl_op_ex!(/= |a: &mut Vec3, b: &Vec3| { a.x /= b.x; a.y /= b.y; a.z /= b.z; });

impl_op_ex!(*= |a: &mut Vec3, b: &f32| { a.x *= b; a.y *= b; a.z *= b; });
impl_op_ex!(/= |a: &mut Vec3, b: &f32| { a.x /= b; a.y /= b; a.z /= b; });

impl_op_ex!(+ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x + b.x, y: a.y + b.y, z: a.z + b.z } });
impl_op_ex!(- |a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x - b.x, y: a.y - b.y, z: a.z - b.z } });
impl_op_ex!(* |a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x * b.x, y: a.y * b.y, z: a.z * b.z } });
impl_op_ex!(/ |a: &Vec3, b: &Vec3| -> Vec3 { Vec3{x: a.x / b.x, y: a.y / b.y, z: a.z / b.z } });

impl_op_ex_commutative!(* |a: &Vec3, b: &f32| -> Vec3 { Vec3{x: a.x * b, y: a.y * b, z: a.z * b } });
impl_op_ex!(/ |a: &Vec3, b: &f32| -> Vec3 { Vec3{x: a.x / b, y: a.y / b, z: a.z / b } });
impl_op_ex!(/ |a: &f32, b: &Vec3| -> Vec3 { Vec3{x: a / b.x, y: a / b.y, z: a / b.z } });

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3{x: -self.x, y: -self.y, z: -self.z}
    }
}
