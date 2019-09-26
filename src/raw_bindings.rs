#![allow(dead_code, unused_imports)]
extern crate nalgebra;

pub use nalgebra::{Matrix3, Matrix4, Vector2, Vector3};

pub mod filament {
    use super::*;
    pub type Entity = u32;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
