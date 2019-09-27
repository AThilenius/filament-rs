#![allow(dead_code, unused_imports)]

extern crate nalgebra;

pub use nalgebra::{Matrix3, Matrix4, Vector2, Vector3};

pub mod filament {
    #![allow(non_snake_case, non_camel_case_types, non_upper_case_globals)]
    use super::*;
    pub type Entity = u32;
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
