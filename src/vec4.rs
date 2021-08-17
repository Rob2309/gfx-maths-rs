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
    fn default() -> Self {
        Self::zero()
    }
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self{x, y, z, w}
    }

    pub fn zero() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0, w: 0.0}
    }

    pub fn one() -> Self {
        Self{x: 1.0, y: 1.0, z: 1.0, w: 1.0}
    }

    pub fn sqr_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn magnitude(&self) -> f32 {
        self.sqr_magnitude().sqrt()
    }

    pub fn normalize(&mut self) -> &mut Self {
        let m = self.magnitude();
        self.x /= m;
        self.y /= m;
        self.z /= m;
        self.w /= m;
        self
    }
    pub fn normalized(&self) -> Self {
        *self.clone().normalize()
    }

    pub fn dot(&self, b: &Vec4) -> f32 {
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
