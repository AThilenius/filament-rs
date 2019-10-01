use crate::{engine::Engine, raw_bindings::*};
use nalgebra::Vector3;

pub type Entity = u32;

#[repr(C)]
pub struct BoundingBox {
  pub center: Vector3<f32>,
  pub half_extent: Vector3<f32>,
}

pub struct SwapChain {
  engine: Engine,
  pub(crate) handle: *mut filament::SwapChain,
}

impl Drop for SwapChain {
  fn drop(&mut self) {
    unsafe {
      filament::Engine_DestroySwapChain(self.engine.handle(), self.handle);
    }
  }
}

impl SwapChain {
  pub(crate) fn new(engine: Engine, surface_handle: *mut std::ffi::c_void) -> Self {
    Self {
      handle: unsafe { filament::Engine_CreateSwapChain(engine.handle(), surface_handle, 0) },
      engine,
    }
  }
}

pub struct RenderTarget {
  pub(crate) handle: *mut filament::RenderTarget,
}

pub struct EntityManager;
impl EntityManager {
  pub fn create() -> Entity {
    unsafe { filament::EntityManager_create() }
  }

  pub fn destroy(entity: Entity) {
    unsafe {
      filament::EntityManager_destroy(entity);
    }
  }

  pub fn is_alive(entity: Entity) {
    unsafe {
      filament::EntityManager_isAlive(entity);
    }
  }
}
