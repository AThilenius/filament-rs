use crate::{engine::Engine, raw_bindings::*};

pub struct Scene {
    engine: Engine,
    pub(crate) handle: *mut filament::Scene,
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            filament::Engine_DestroyScene(self.engine.handle(), self.handle);
        }
    }
}

impl Scene {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::Engine_CreateScene(engine.handle()) },
            engine,
        }
    }
}
