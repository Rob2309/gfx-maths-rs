use std::fmt::Display;

use crate::Vec3;

use auto_ops::impl_op_ex;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Default for Quaternion {
    /// Creates an identity rotation
    fn default() -> Self {
        Self::identity()
    }
}

impl Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { x, y, z, w } = self;
        write!(f, "({x}, {y}, {z}, {w})")
    }
}

impl Quaternion {
    pub const fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }

    /// Creates (0, 0, 0, 1), which represents no rotation
    pub const fn identity() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }

    /// Creates a rotation of `radians` radians around `axis`.
    ///
    /// The rotation will be counter clock wise when looking along the direction of `axis`.
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

    /// Returns the vector (1, 0, 0) rotated by `self`
    pub fn right(&self) -> Vec3 {
        Vec3 {
            x: self.x * self.x - self.y * self.y - self.z * self.z + self.w * self.w,
            y: 2.0 * (self.z * self.w + self.x * self.y),
            z: 2.0 * (self.x * self.z - self.y * self.w),
        }
    }

    /// Returns the vector (0, 1, 0) rotated by `self`
    pub fn up(&self) -> Vec3 {
        Vec3 {
            x: 2.0 * (self.x * self.y - self.z * self.w),
            y: -self.x * self.x + self.y * self.y - self.z * self.z + self.w * self.w,
            z: 2.0 * (self.x * self.w + self.y * self.z),
        }
    }

    /// Returns the vector (0, 0, 1) rotated by `self`
    pub fn forward(&self) -> Vec3 {
        Vec3 {
            x: 2.0 * (self.x * self.z + self.y * self.w),
            y: 2.0 * (self.y * self.z - self.x * self.w),
            z: -self.x * self.x - self.y * self.y + self.z * self.z + self.w * self.w,
        }
    }

    /// Creates a Quaternion from euler angles in radians
    ///
    /// The rotation order is Z -> Y -> X
    pub fn from_euler_radians_zyx(euler: &Vec3) -> Self {
        let cx = (euler.x * 0.5).cos();
        let cy = (euler.y * 0.5).cos();
        let cz = (euler.z * 0.5).cos();

        let sx = (euler.x * 0.5).sin();
        let sy = (euler.y * 0.5).sin();
        let sz = (euler.z * 0.5).sin();

        Self {
            x: sx * cy * cz - cx * sy * sz,
            y: cx * sy * cz + sx * cy * sz,
            z: cx * cy * sz - sx * sy * cz,
            w: cx * cy * cz + sx * sy * sz,
        }
    }

    /// Creates a Quaternion from euler angles in degrees
    ///
    /// The rotation order is Z -> Y -> X
    pub fn from_euler_angles_zyx(euler: &Vec3) -> Self {
        Self::from_euler_radians_zyx(&Vec3::new(
            euler.x.to_radians(),
            euler.y.to_radians(),
            euler.z.to_radians(),
        ))
    }

    /// Converts this Quaternion to euler angles in radians
    ///
    /// The rotation order is Z -> Y -> X
    pub fn to_euler_radians_zyx(&self) -> Vec3 {
        Vec3 {
            x: f32::atan2(
                2.0 * (self.w * self.x + self.y * self.z),
                1.0 - 2.0 * (self.x * self.x + self.y * self.y),
            ),
            y: f32::asin(2.0 * (self.w * self.y - self.z * self.x)),
            z: f32::atan2(
                2.0 * (self.w * self.z + self.x * self.y),
                1.0 - 2.0 * (self.y * self.y + self.z * self.z),
            ),
        }
    }

    /// Converts this Quaternion to euler angles in degrees
    ///
    /// The rotation order is Z -> Y -> X
    pub fn to_euler_angles_zyx(&self) -> Vec3 {
        let rad = self.to_euler_radians_zyx();
        Vec3::new(rad.x.to_degrees(), rad.y.to_degrees(), rad.z.to_degrees())
    }
}

impl_op_ex!(*|a: &Quaternion, b: &Quaternion| -> Quaternion {
    Quaternion {
        x: a.w * b.x + a.x * b.w + a.y * b.z - a.z * b.y,
        y: a.w * b.y + a.y * b.w + a.z * b.x - a.x * b.z,
        z: a.w * b.z + a.z * b.w + a.x * b.y - a.y * b.x,
        w: a.w * b.w - a.x * b.x - a.y * b.y - a.z * b.z,
    }
});

impl_op_ex!(*|a: &Quaternion, b: &Vec3| -> Vec3 {
    let x2 = a.x * a.x;
    let y2 = a.y * a.y;
    let z2 = a.z * a.z;
    let w2 = a.w * a.w;

    let xx = a.x * b.x;
    let yy = a.y * b.y;
    let zz = a.z * b.z;

    Vec3 {
        x: b.x * (x2 - y2 - z2 + w2)
            + 2.0 * (a.x * yy + a.x * zz + a.w * a.y * b.z - a.w * a.z * b.y),
        y: b.y * (-x2 + y2 - z2 + w2)
            + 2.0 * (a.y * xx + a.y * zz + a.w * a.z * b.x - a.w * a.x * b.z),
        z: b.z * (-x2 - y2 + z2 + w2)
            + 2.0 * (a.z * xx + a.z * yy + a.w * a.x * b.y - a.w * a.y * b.x),
    }
});

impl_op_ex!(-|a: &Quaternion| -> Quaternion {
    Quaternion {
        x: -a.x,
        y: -a.y,
        z: -a.z,
        w: a.w,
    }
});

impl From<[f32; 4]> for Quaternion {
    fn from(d: [f32; 4]) -> Self {
        Self {
            x: d[0],
            y: d[1],
            z: d[2],
            w: d[3],
        }
    }
}
