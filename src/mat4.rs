use auto_ops::impl_op_ex;

use crate::{Quaternion, Vec3, Vec4};

/// A struct representing a 4x4 matrix.
///
/// It's values are stored in column-major order by default,
/// as expected by OpenGL and accepted by all other APIs.
/// To change this, use feature `mat-row-major`
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(C)]
pub struct Mat4 {
    pub values: [f32; 4 * 4],
}

impl Default for Mat4 {
    /// Creates the identity matrix.
    fn default() -> Self {
        Self::IDENTITY
    }
}

#[cfg(not(feature = "mat-row-major"))]
const fn cr(c: usize, r: usize) -> usize {
    r + c * 4
}
#[cfg(feature = "mat-row-major")]
const fn cr(c: usize, r: usize) -> usize {
    r * 4 + c
}

impl Mat4 {
    /// The identity matrix
    pub const IDENTITY: Self = Self {
        values: [
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    };

    /// Creates a 3D translation matrix.
    pub const fn translate(t: Vec3) -> Self {
        let mut res = Self::IDENTITY;

        res.values[cr(3, 0)] = t.x;
        res.values[cr(3, 1)] = t.y;
        res.values[cr(3, 2)] = t.z;

        res
    }

    /// Creates a 3D rotation matrix.
    pub fn rotate(r: Quaternion) -> Self {
        let mut res = Self::IDENTITY;

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
    pub const fn scale(s: Vec3) -> Self {
        let mut res = Self::IDENTITY;

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
    pub fn orthographic_vulkan(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mut res = Self::IDENTITY;

        let a = 2.0 / (right - left);
        let b = (-left - right) / (right - left);
        let c = 2.0 / (top - bottom);
        let d = (-bottom - top) / (top - bottom);
        let e = 1.0 / (far - near);
        let f = -near / (far - near);

        res.values[cr(0, 0)] = a;
        res.values[cr(3, 0)] = b;

        res.values[cr(1, 1)] = c;
        res.values[cr(3, 1)] = d;

        res.values[cr(2, 2)] = e;
        res.values[cr(3, 2)] = f;

        res
    }

    /// Creates an orthographic projection matrix
    /// with z mapped to \[-1; 1\], as expected by OpenGL.
    pub fn orthographic_opengl(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mut res = Self::IDENTITY;

        let a = 2.0 / (right - left);
        let b = (-left - right) / (right - left);
        let c = 2.0 / (top - bottom);
        let d = (-bottom - top) / (top - bottom);
        let e = 2.0 / (far - near);
        let f = (-near - far) / (far - near);

        res.values[cr(0, 0)] = a;
        res.values[cr(3, 0)] = b;

        res.values[cr(1, 1)] = c;
        res.values[cr(3, 1)] = d;

        res.values[cr(2, 2)] = e;
        res.values[cr(3, 2)] = f;

        res
    }

    /// Creates an inverse orthographics projection matrix
    /// with z mapped to \[0; 1\], as expected by Vulkan.
    ///
    /// The world position of a uv/depth pair can be reconstructed with the same code as shown
    /// in [`inverse_perspective_vulkan()`](Self::inverse_perspective_vulkan())
    pub fn inverse_orthographic_vulkan(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mut res = Self::IDENTITY;

        let a = 2.0 * (right - left);
        let b = (-left - right) / (right - left);
        let c = 2.0 * (top - bottom);
        let d = (-bottom - top) / (top - bottom);
        let e = 1.0 / (far - near);
        let f = -near / (far - near);

        res.values[cr(0, 0)] = 1.0 / a;
        res.values[cr(3, 0)] = -b / a;

        res.values[cr(1, 1)] = 1.0 / c;
        res.values[cr(3, 1)] = -d / c;

        res.values[cr(2, 2)] = 1.0 / e;
        res.values[cr(3, 2)] = -f / e;

        res
    }

    /// Creates an inverse orthographics projection matrix
    /// with z mapped to \[-1; 1\], as expected by OpenGL.
    ///
    /// The world position of a uv/depth pair can be reconstructed with the same code as shown
    /// in [`inverse_perspective_opengl()`](Self::inverse_perspective_opengl())
    pub fn inverse_orthographic_opengl(
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        near: f32,
        far: f32,
    ) -> Self {
        let mut res = Self::IDENTITY;

        let a = 2.0 * (right - left);
        let b = (-left - right) / (right - left);
        let c = 2.0 * (top - bottom);
        let d = (-bottom - top) / (top - bottom);
        let e = 2.0 * (far - near);
        let f = (-near - far) / (far - near);

        res.values[cr(0, 0)] = 1.0 / a;
        res.values[cr(3, 0)] = -b / a;

        res.values[cr(1, 1)] = 1.0 / c;
        res.values[cr(3, 1)] = -d / c;

        res.values[cr(2, 2)] = 1.0 / e;
        res.values[cr(3, 2)] = -f / e;

        res
    }

    /// Creates a perspective projection matrix
    /// with z mapped to \[0; 1\], as expected by Vulkan.
    pub fn perspective_vulkan(fov_rad: f32, near: f32, far: f32, aspect: f32) -> Self {
        let mut res = Self::IDENTITY;
        let thfov = (fov_rad * 0.5).tan();

        res.values[cr(0, 0)] = 1.0 / (thfov * aspect);
        res.values[cr(1, 1)] = 1.0 / thfov;

        res.values[cr(2, 2)] = far / (far - near);
        res.values[cr(3, 2)] = (-far * near) / (far - near);

        res.values[cr(2, 3)] = 1.0;
        res.values[cr(3, 3)] = 0.0;

        res
    }

    /// Creates a perspective projection matrix
    /// with z mapped to \[-1; 1\], as expected by OpenGL.
    pub fn perspective_opengl(fov_rad: f32, near: f32, far: f32, aspect: f32) -> Self {
        let mut res = Self::IDENTITY;
        let thfov = (fov_rad * 0.5).tan();

        res.values[cr(0, 0)] = 1.0 / (thfov * aspect);
        res.values[cr(1, 1)] = 1.0 / thfov;

        res.values[cr(2, 2)] = (far + near) / (far - near);
        res.values[cr(3, 2)] = (-2.0 * far * near) / (far - near);

        res.values[cr(2, 3)] = 1.0;
        res.values[cr(3, 3)] = 0.0;

        res
    }

    /// Creates an inverse perspective matrix
    /// with z mapped to \[0; 1\], as expected by Vulkan.
    ///
    /// This matrix can be used to reconstruct the world position from a uv/depth pair in a shader.
    /// This pseudo code can be used:
    /// ```GLSL
    /// vec4 clipPos = vec4(uv.xy, depth, 1.0);
    /// vec4 worldPos = invProjection * clipPos;
    /// worldPos /= worldPos.w;
    /// ```
    pub fn inverse_perspective_vulkan(fov_rad: f32, near: f32, far: f32, aspect: f32) -> Self {
        let mut res = Self::IDENTITY;

        let thfov = (fov_rad * 0.5).tan();
        let c = far / (far - near);
        let d = (-far * near) / (far - near);

        res.values[cr(0, 0)] = thfov * aspect;
        res.values[cr(1, 1)] = thfov;

        res.values[cr(3, 2)] = 1.0;

        res.values[cr(2, 2)] = 0.0;
        res.values[cr(2, 3)] = 1.0 / d;
        res.values[cr(3, 3)] = -c / d;

        res
    }

    /// Creates an inverse perspective matrix
    /// with z mapped to \[-1; 1\], as expected by OpenGL.
    ///
    /// This matrix can be used to reconstruct the world position from a uv/depth pair in a shader.
    /// This pseudo code can be used:
    /// ```GLSL
    /// vec4 clipPos = vec4(uv.xy, depth * 2.0 - 1.0, 1.0);
    /// vec4 worldPos = invProjection * clipPos;
    /// worldPos /= worldPos.w;
    /// ```
    pub fn inverse_perspective_opengl(fov_rad: f32, near: f32, far: f32, aspect: f32) -> Self {
        let mut res = Self::IDENTITY;

        let thfov = (fov_rad * 0.5).tan();
        let c = (far + near) / (far - near);
        let d = (-2.0 * far * near) / (far - near);

        res.values[cr(0, 0)] = thfov * aspect;
        res.values[cr(1, 1)] = thfov;

        res.values[cr(3, 2)] = 1.0;

        res.values[cr(2, 2)] = 0.0;
        res.values[cr(2, 3)] = 1.0 / d;
        res.values[cr(3, 3)] = -c / d;

        res
    }

    /// Returns a value indexed by `column` and `row`
    pub const fn get(&self, column: usize, row: usize) -> f32 {
        self.values[cr(column, row)]
    }

    /// Sets the value indexed by `column` and `row` to `val`
    pub fn set(&mut self, column: usize, row: usize, val: f32) {
        self.values[cr(column, row)] = val;
    }

    /// Returns a transposed copy of `self`.
    #[must_use]
    pub fn transposed(&self) -> Mat4 {
        let mut res = Mat4::IDENTITY;

        for c in 0..4 {
            for r in 0..4 {
                res.values[cr(r, c)] = self.values[cr(c, r)];
            }
        }

        res
    }

    /// Returns the underlying values as a slice
    pub fn as_slice(&self) -> &[f32] {
        &self.values
    }

    /// Returns the underlying values as a mutable slice
    pub fn as_mut_slice(&mut self) -> &mut [f32] {
        &mut self.values
    }

    /// Returns the underlying values as a pointer with no size information
    pub fn as_ptr(&self) -> *const f32 {
        self.values.as_ptr()
    }

    /// Returns the underlying values as a mutable pointer with no size information
    pub fn as_mut_ptr(&mut self) -> *mut f32 {
        self.values.as_mut_ptr()
    }
}

impl_op_ex!(*|a: &Mat4, b: &Mat4| -> Mat4 {
    let mut res = Mat4::IDENTITY;

    for r in 0..4 {
        for c in 0..4 {
            res.values[cr(c, r)] = a.values[cr(0, r)] * b.values[cr(c, 0)]
                + a.values[cr(1, r)] * b.values[cr(c, 1)]
                + a.values[cr(2, r)] * b.values[cr(c, 2)]
                + a.values[cr(3, r)] * b.values[cr(c, 3)];
        }
    }

    res
});

impl_op_ex!(*|a: &Mat4, b: &Vec4| -> Vec4 {
    Vec4 {
        x: a.values[cr(0, 0)] * b.x
            + a.values[cr(1, 0)] * b.y
            + a.values[cr(2, 0)] * b.z
            + a.values[cr(3, 0)] * b.w,
        y: a.values[cr(0, 1)] * b.x
            + a.values[cr(1, 1)] * b.y
            + a.values[cr(2, 1)] * b.z
            + a.values[cr(3, 1)] * b.w,
        z: a.values[cr(0, 2)] * b.x
            + a.values[cr(1, 2)] * b.y
            + a.values[cr(2, 2)] * b.z
            + a.values[cr(3, 2)] * b.w,
        w: a.values[cr(0, 3)] * b.x
            + a.values[cr(1, 3)] * b.y
            + a.values[cr(2, 3)] * b.z
            + a.values[cr(3, 3)] * b.w,
    }
});

impl_op_ex!(*|a: &Mat4, b: &Vec3| -> Vec3 {
    Vec3 {
        x: a.values[cr(0, 0)] * b.x + a.values[cr(1, 0)] * b.y + a.values[cr(2, 0)] * b.z,
        y: a.values[cr(0, 1)] * b.x + a.values[cr(1, 1)] * b.y + a.values[cr(2, 1)] * b.z,
        z: a.values[cr(0, 2)] * b.x + a.values[cr(1, 2)] * b.y + a.values[cr(2, 2)] * b.z,
    }
});

impl From<[f32; 16]> for Mat4 {
    fn from(d: [f32; 16]) -> Self {
        Self { values: d }
    }
}

impl From<[[f32; 4]; 4]> for Mat4 {
    fn from(d: [[f32; 4]; 4]) -> Self {
        Self {
            values: [
                d[0][0], d[0][1], d[0][2], d[0][3], d[1][0], d[1][1], d[1][2], d[1][3], d[2][0],
                d[2][1], d[2][2], d[2][3], d[3][0], d[3][1], d[3][2], d[3][3],
            ],
        }
    }
}

impl std::ops::Index<(usize, usize)> for Mat4 {
    type Output = f32;

    fn index(&self, (c, r): (usize, usize)) -> &Self::Output {
        &self.values[cr(c, r)]
    }
}

impl std::ops::IndexMut<(usize, usize)> for Mat4 {
    fn index_mut(&mut self, (c, r): (usize, usize)) -> &mut Self::Output {
        &mut self.values[cr(c, r)]
    }
}
