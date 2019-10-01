use crate::{engine::Engine, raw_bindings::*};
use nalgebra::{Matrix4, Vector3};

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

pub struct TransformManager {
  engine: Engine,
}

impl TransformManager {
  pub fn new(engine: Engine) -> Self {
    TransformManager { engine }
  }

  pub fn set_transform(&mut self, entity: Entity, transform: Matrix4<f32>) {
    unsafe {
      filament::TransformManager_SetTransform(
        self.engine.handle(),
        entity,
        transform.as_ptr() as *mut f32,
      );
    }
  }
}
