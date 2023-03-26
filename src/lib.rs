#![doc = include_str!("../README.md")]

#![allow(unknown_lints)]
#![warn(clippy::all)]

// Used to make docs.rs more readable
#![cfg_attr(docs_rs, feature(doc_auto_cfg))]

#[macro_use]
mod macros;

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

pub mod color;
pub use color::*;
