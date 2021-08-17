# GFX Maths
This crate implements all the basic mathematical structures and operations
that are needed for almost any graphical program, namely:
- [Vec2](src/vec2.rs)
- [Vec3](src/vec3.rs)
- [Vec4](src/vec4.rs)
- [Quaternion](src/quaternion.rs)
- [Mat4](src/mat4.rs)

The usual operations are implemented via member functions and operator overloads.
Operators should handle almost exactly as they would in GLSL, e.g.
```Rust
use gfx_maths_rs::*;

let v = Vec3::new(5.0, 6.0, 7.0);
let s = 1.0 / v;

let t = Mat4::translate(Vec3::new(1.0, 0.0, 0.0)) * s;
```

# Notation
Vectors are always treated as column vectors, which is why
only [Mat4](src/mat4.rs) * [Vec4](src/vec4.rs) is implemented and not [Vec4](src/vec4.rs) * [Mat4](src/mat4.rs).
