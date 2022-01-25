//! This crate implements all the basic mathematical structures and operations
//! that are needed for almost any graphical program, namely:
//! - [`Vec2`]
//! - [`Vec3`]
//! - [`Vec4`]
//! - [`Quaternion`]
//! - [`Mat4`]
//!
//! The usual operations are implemented via member functions and operator overloads.
//! Operators should handle almost exactly as they would in GLSL, e.g.
//! ```
//! use gfx_maths::*;
//!
//! let v = Vec3::new(5.0, 6.0, 7.0);
//! let s = 1.0 / v;
//!
//! let t = Mat4::translate(Vec3::new(1.0, 0.0, 0.0)) * s;
//! ```
//!
//! # Notation
//! Vectors are always treated as column vectors, which is why
//! only [`Mat4`] * [`Vec4`] is implemented and not [`Vec4`] * [`Mat4`].

#![allow(unknown_lints)]
#![warn(clippy::all, clippy::pedantic)]
#![allow(
    clippy::float_cmp,
    clippy::must_use_candidate,
    clippy::many_single_char_names,
    clippy::too_many_lines
)]

pub mod vec2;
pub use vec2::*;

pub mod vec3;
pub use vec3::*;

pub mod vec4;
pub use vec4::*;

pub mod quaternion;
pub use quaternion::*;

pub mod mat4;
pub use mat4::*;
