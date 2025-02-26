// Take a look at the license at the top of the repository in the LICENSE file.

//! # Rust Graphene bindings
//!
//! This library contains safe Rust bindings for [Graphene](https://github.com/ebassi/graphene).
//! It is a part of [gtk-rs](https://gtk-rs.org/).
//!
//! Graphene 2.44 is the lowest supported version for the underlying library.

#![cfg_attr(feature = "dox", feature(doc_cfg))]

pub use ffi;
pub use glib;

// Graphene has no runtime to initialize
macro_rules! assert_initialized_main_thread {
    () => {};
}

// No-op
macro_rules! skip_assert_initialized {
    () => {};
}

#[allow(clippy::transmute_ptr_to_ref)]
#[allow(clippy::type_complexity)]
#[allow(clippy::unreadable_literal)]
#[allow(clippy::derive_hash_xor_eq)]
#[allow(unused_imports)]
mod auto;

pub mod prelude;

pub use crate::auto::*;

mod box_;
mod euler;
mod frustum;
mod matrix;
mod plane;
mod point;
mod point3_d;
mod quad;
mod quaternion;
mod ray;
mod rect;
mod size;
mod sphere;
mod triangle;
mod vec2;
mod vec3;
mod vec4;
