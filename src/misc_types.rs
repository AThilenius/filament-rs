use crate::{engine::Engine, raw_bindings::*};

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
