use crate::{engine::Engine, raw_bindings::*};

pub struct Renderer {
    engine: Engine,
    handle: *mut filament::Renderer,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            filament::Engine_nDestroyRenderer(self.engine.handle(), self.handle);
        }
    }
}

impl Renderer {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::Engine_nCreateRenderer(engine.handle()) },
            engine,
        }
    }
}
