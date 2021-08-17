use crate::Vec3;

use auto_ops::*;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quaternion {
    fn default() -> Self {
        Self::identity()
    }
}

impl Quaternion {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self{x, y, z, w}
    }

    pub fn identity() -> Self {
        Self{x: 0.0, y: 0.0, z: 0.0, w: 1.0}
    }

    pub fn axis_angle(mut axis: Vec3, radians: f32) -> Self {
        axis.normalize();
        axis *= (radians * 0.5).sin();

        Self {
            x: axis.x,
            y: axis.y,
            z: axis.z,
            w: (radians * 0.5).cos(),
        }
    }

    pub fn right(&self) -> Vec3 {
        Vec3 {
            x: self.x * self.x - self.y * self.y - self.z * self.z + self.w * self.w,
            y: 2.0 * (self.z * self.w + self.x * self.y),
            z: 2.0 * (self.x * self.z - self.y * self.w),
        }
    }

    pub fn up(&self) -> Vec3 {
        Vec3 {
            x: 2.0 * (self.x * self.y - self.z * self.w),
            y: -self.x * self.x + self.y * self.y - self.z * self.z + self.w * self.w,
            z: 2.0 * (self.x * self.w + self.y * self.z),
        }
    }

    pub fn forward(&self) -> Vec3 {
        Vec3 {
            x: 2.0 * (self.x * self.z + self.y * self.w),
            y: 2.0 * (self.y * self.z - self.x * self.w),
            z: -self.x * self.x - self.y * self.y + self.z * self.z + self.w * self.w,
        }
    }
}

impl_op_ex!(* |a: &Quaternion, b: &Quaternion| -> Quaternion { Quaternion {
    x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
    y: a.w * b.y + a.y * b.w + a.z * b.x - a.x * b.z,
    z: a.w * b.z + a.z * b.w + a.x * b.y - a.y * b.x,
    w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
} });

impl_op_ex!(* |a: &Quaternion, b: &Vec3| -> Vec3 {
    let x2 = a.x * a.x;
    let y2 = a.y * a.y;
    let z2 = a.z * a.z;
    let w2 = a.w * a.w;

    let xx = a.x * b.x;
    let yy = a.y * b.y;
    let zz = a.z * b.z;

    Vec3 {
        x: b.x * (x2 - y2 - z2 + w2) + 2.0 * (a.x * yy + a.x * zz + a.w * a.y * b.z - a.w * a.z * b.y),
        y: b.y * (-x2 + y2 - z2 + w2) + 2.0 * (a.y * xx + a.y * zz + a.w * a.z * b.x - a.w * a.x * b.z),
        z: b.z * (-x2 - y2 + z2 + w2) + 2.0 * (a.z * xx + a.z * yy + a.w * a.x * b.y - a.w * a.y * b.x),
    }
});

impl_op_ex!(- |a: &Quaternion| -> Quaternion {
    Quaternion {
        x: -a.x,
        y: -a.y,
        z: -a.z,
        w: a.w,
    }
});
