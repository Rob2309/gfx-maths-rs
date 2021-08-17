use auto_ops::*;

use crate::{Quaternion, Vec3, Vec4};

/// A struct representing a 4x4 matrix.
/// 
/// It's values are stored in column-major order by default,
/// as expected by OpenGL and accepted by all other APIs.
/// To change this, use feature `mat-row-major`
#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(C)]
pub struct Mat4 {
    pub values: [f32; 4*4],
}

impl Default for Mat4 {
    /// Creates the identity matrix.
    fn default() -> Self {
        Self::identity()
    }
}

#[cfg(not(feature="mat-row-major"))]
const fn cr(c: usize, r: usize) -> usize {
    r + c * 4
}
#[cfg(feature="mat-row-major")]
const fn cr(c: usize, r: usize) -> usize {
    r * 4 + c
}

impl Mat4 {
    /// Creates the identity matrix.
    pub fn identity() -> Self {
        Self {
            values: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ]
        }
    }

    /// Creates a 3D translation matrix.
    pub fn translate(t: Vec3) -> Self {
        let mut res = Self::identity();

        res.values[cr(3, 0)] = t.x;
        res.values[cr(3, 1)] = t.y;
        res.values[cr(3, 2)] = t.z;
        
        res
    }

    /// Creates a 3D rotation matrix.
    pub fn rotate(r: Quaternion) -> Self {
        let mut res = Self::identity();

        let right = r.right();
        let up = r.up();
        let fwd = r.forward();

        res.values[cr(0, 0)] = right.x;
        res.values[cr(0, 1)] = right.y;
        res.values[cr(0, 2)] = right.z;

        res.values[cr(1, 0)] = up.x;
        res.values[cr(1, 1)] = up.y;
        res.values[cr(1, 2)] = up.z;

        res.values[cr(2, 0)] = fwd.x;
        res.values[cr(2, 1)] = fwd.y;
        res.values[cr(2, 2)] = fwd.z;

        res
    }

    /// Creates a 3D scale matrix.
    pub fn scale(s: Vec3) -> Self {
        let mut res = Self::identity();

        res.values[cr(0, 0)] = s.x;
        res.values[cr(1, 1)] = s.y;
        res.values[cr(2, 2)] = s.z;

        res
    }

    /// Creates a 3D local-to-world/object-to-world matrix.
    /// 
    /// When multiplying this matrix by a vector, it will be
    /// - scaled by `s`
    /// - rotated by `r`
    /// - translated by `t`
    /// 
    /// in this order.
    pub fn local_to_world(t: Vec3, r: Quaternion, s: Vec3) -> Self {
        Self::translate(t) * Self::rotate(r) * Self::scale(s)
    }

    /// Creates a 3D world-to-local/world-to-object matrix.
    /// 
    /// This matrix does the opposite of [`local_to_world()`](Self::local_to_world())
    pub fn world_to_local(t: Vec3, r: Quaternion, s: Vec3) -> Self {
        Self::scale(1.0 / s) * Self::rotate(-r) * Self::translate(-t)
    }

    /// Creates an orthographic projection matrix
    /// with z mapped to \[0; 1\], as expected by Vulkan.
    #[cfg(feature="mat-vulkan")]
    pub fn orthographic(l: f32, r: f32, b: f32, t: f32, n: f32, f: f32) -> Self {
        let mut res = Self::identity();

        res.values[cr(0, 0)] = 2.0 * (r - l);
        res.values[cr(3, 0)] = (-l - r) / (r - l);

        res.values[cr(1, 1)] = 2.0 * (t - b);
        res.values[cr(3, 1)] = (-b - t) / (t - b);

        res.values[cr(2, 2)] = 1.0 / (f - n);
        res.values[cr(3, 2)] = -n / (f - n);

        res
    }

    /// Creates a perspective projection matrix
    /// with z mapped to \[0; 1\], as expected by Vulkan.
    #[cfg(feature="mat-vulkan")]
    pub fn perspective(fov_rad: f32, near: f32, far: f32, aspect: f32) -> Self {
        let mut res = Self::identity();
        let thfov = (fov_rad * 0.5).tan();

        res.values[cr(0, 0)] = 1.0 / (thfov * aspect);
        res.values[cr(1, 1)] = 1.0 / thfov;
        
        res.values[cr(2, 2)] = far / (far - near);
        res.values[cr(3, 2)] = (-far * near) / (far - near);

        res.values[cr(2, 3)] = 1.0;
        res.values[cr(3, 3)] = 0.0;

        res
    }

    /// Returns a value indexed by `column` and `row`
    pub fn get(&self, column: usize, row: usize) -> f32 {
        self.values[cr(column, row)]
    }

    /// Sets the value indexed by `column` and `row` to `val`
    pub fn set(&mut self, column: usize, row: usize, val: f32) {
        self.values[cr(column, row)] = val;
    }

    /// Returns a transposed copy of `self`.
    pub fn transposed(&self) -> Mat4 {
        let mut res = Mat4::identity();

        for c in 0..4 {
            for r in 0..4 {
                res.values[cr(r, c)] = self.values[cr(c, r)];
            }
        }

        res
    }
}

impl_op_ex!(* |a: &Mat4, b: &Mat4| -> Mat4 {
    let mut res = Mat4::identity();

    for r in 0..4 {
        for c in 0..4 {
            res.values[cr(c, r)] = 
                a.values[cr(0, r)] * b.values[cr(c, 0)] +
                a.values[cr(1, r)] * b.values[cr(c, 1)] +
                a.values[cr(2, r)] * b.values[cr(c, 2)] +
                a.values[cr(3, r)] * b.values[cr(c, 3)];
        }
    }

    res
});

impl_op_ex!(* |a: &Mat4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.values[cr(0, 0)] * b.x + a.values[cr(1, 0)] * b.y + a.values[cr(2, 0)] * b.z + a.values[cr(3, 0)] * b.w,
        y: a.values[cr(0, 1)] * b.x + a.values[cr(1, 1)] * b.y + a.values[cr(2, 1)] * b.z + a.values[cr(3, 1)] * b.w,
        z: a.values[cr(0, 2)] * b.x + a.values[cr(1, 2)] * b.y + a.values[cr(2, 2)] * b.z + a.values[cr(3, 2)] * b.w,
        w: a.values[cr(0, 3)] * b.x + a.values[cr(1, 3)] * b.y + a.values[cr(2, 3)] * b.z + a.values[cr(3, 3)] * b.w,
    }
});

impl_op_ex!(* |a: &Mat4, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.values[cr(0, 0)] * b.x + a.values[cr(1, 0)] * b.y + a.values[cr(2, 0)] * b.z,
        y: a.values[cr(0, 1)] * b.x + a.values[cr(1, 1)] * b.y + a.values[cr(2, 1)] * b.z,
        z: a.values[cr(0, 2)] * b.x + a.values[cr(1, 2)] * b.y + a.values[cr(2, 2)] * b.z,
    }
});
