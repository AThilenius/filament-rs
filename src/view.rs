use crate::{engine::Engine, raw_bindings::*};

pub struct View {
    engine: Engine,
    handle: *mut filament::View,
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            filament::Engine_nDestroyView(self.engine.handle(), self.handle);
        }
    }
}

impl View {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::Engine_nCreateView(engine.handle()) },
            engine,
        }
    }
}
