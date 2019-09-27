use crate::{engine::Engine, raw_bindings::*};

pub struct Camera {
  engine: Engine,
  pub(crate) handle: *mut filament::Camera,
}

impl Drop for Camera {
  fn drop(&mut self) {
    unsafe {
      filament::Engine_DestroyCamera(self.engine.handle(), self.handle);
    }
  }
}

impl Camera {
  pub(crate) fn new(engine: Engine) -> Self {
    Self {
      handle: unsafe { filament::Engine_CreateCamera(engine.handle()) },
      engine,
    }
  }
}
