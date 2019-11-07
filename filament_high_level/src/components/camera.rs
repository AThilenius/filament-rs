use legion::prelude::*;
use nalgebra::{Orthographic3, Perspective3};

pub enum Camera {
    Perspective(Perspective3<f64>),
    Orthographic(Orthographic3<f64>),
}

impl Camera {
    pub fn new_perspective(fov: f64, near: f64, far: f64) -> Self {
        Self::Perspective(Perspective3::new(1.0, fov, near, far))
    }
}

/// Used as a World `Resource` to mark the main camera.
pub struct MainCamera {
    pub entity: Entity,
}

impl MainCamera {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
