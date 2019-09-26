use crate::{
    camera::Camera, misc_types::SwapChain, raw_bindings::*, renderer::Renderer, scene::Scene,
    view::View,
};
use std::rc::Rc;

/// A handle to a Filament engine that will free the engine when dropped (wrapped in an Rc).
pub(crate) struct EngineHandle(pub *mut filament::Engine);

impl Drop for EngineHandle {
    fn drop(&mut self) {
        println!("Shutting engine down");
        unsafe {
            filament::Engine_nDestroyEngine(self.0);
        }
    }
}

/// Wraps a Filament engine and handles freeing the engine with no more references to the underlying
/// unmanaged engine exist.
#[derive(Clone)]
pub struct Engine {
    pub(crate) handle_rc: Rc<EngineHandle>,
}

impl Engine {
    pub fn new() -> Self {
        let f_engine = unsafe { filament::Engine_nCreateEngine() };
        Self {
            handle_rc: Rc::new(EngineHandle(f_engine)),
        }
    }

    pub(crate) fn handle(&self) -> *mut filament::Engine {
        (*self.handle_rc).0
    }

    pub fn create_swap_chain(&mut self, window_handle: *mut std::ffi::c_void) -> SwapChain {
        SwapChain::new(self.clone(), window_handle)
    }

    pub fn create_view(&mut self) -> View {
        View::new(self.clone())
    }

    pub fn create_scene(&mut self) -> Scene {
        Scene::new(self.clone())
    }

    pub fn create_renderer(&mut self) -> Renderer {
        Renderer::new(self.clone())
    }

    pub fn create_camera(&mut self) -> Camera {
        Camera::new(self.clone())
    }
}
