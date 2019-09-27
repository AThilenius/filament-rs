use crate::{engine::Engine, misc_types::SwapChain, raw_bindings::*, view::View};

pub struct Renderer {
    engine: Engine,
    pub(crate) handle: *mut filament::Renderer,
}

impl Drop for Renderer {
    fn drop(&mut self) {
        unsafe {
            filament::Engine_DestroyRenderer(self.engine.handle(), self.handle);
        }
    }
}

impl Renderer {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::Engine_CreateRenderer(engine.handle()) },
            engine,
        }
    }

    pub fn begin_frame(&self, swap_chain: &SwapChain) {
        unsafe {
            filament::Renderer_BeginFrame(self.handle, swap_chain.handle);
        }
    }

    pub fn end_frame(&self) {
        unsafe {
            filament::Renderer_EndFrame(self.handle);
        }
    }

    pub fn render(&self, view: &View) {
        unsafe {
            filament::Renderer_Render(self.handle, view.handle);
        }
    }

    pub fn copy_frame(
        &self,
        swap_chain: &SwapChain,
        dst_left: i32,
        dst_bottom: i32,
        dst_width: i32,
        dst_height: i32,
        src_left: i32,
        src_bottom: i32,
        src_width: i32,
        src_height: i32,
        flags: i32,
    ) {
        unsafe {
            filament::Renderer_CopyFrame(
                self.handle,
                swap_chain.handle,
                dst_left,
                dst_bottom,
                dst_width,
                dst_height,
                src_left,
                src_bottom,
                src_width,
                src_height,
                flags,
            );
        }
    }
}
