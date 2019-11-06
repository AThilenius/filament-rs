use crate::low_level::{engine::Engine, raw_bindings::*};
use std::mem;

pub enum IndexType {
    Ushort = 0,
    Uint = 1,
}

pub struct IndexBufferBuilder {
    engine: Engine,
    pub(crate) handle: *mut filament::IndexBufferBuilder,
}

impl Drop for IndexBufferBuilder {
    fn drop(&mut self) {
        unsafe {
            filament::IndexBuffer_DestroyBuilder(self.handle);
        }
    }
}

impl IndexBufferBuilder {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::IndexBuffer_CreateBuilder() },
            engine,
        }
    }

    pub fn index_count(self, index_count: u32) -> Self {
        unsafe {
            filament::IndexBuffer_BuilderIndexCount(self.handle, index_count);
        }
        self
    }

    pub fn buffer_type(self, index_type: IndexType) -> Self {
        unsafe {
            filament::IndexBuffer_BuilderBufferType(self.handle, index_type as i32);
        }
        self
    }

    pub fn build(self) -> IndexBuffer {
        let handle =
            unsafe { filament::IndexBuffer_BuilderBuild(self.handle, self.engine.handle()) };
        IndexBuffer::new(handle, self.engine.clone())
    }
}

#[derive(Clone)]
pub struct IndexBuffer {
    engine: Engine,
    pub(crate) handle: *mut filament::IndexBuffer,
}

impl IndexBuffer {
    pub(self) fn new(handle: *mut filament::IndexBuffer, engine: Engine) -> Self {
        Self { handle, engine }
    }

    pub fn get_index_count(&self) -> u64 {
        unsafe { filament::IndexBuffer_GetIndexCount(self.handle) }
    }

    pub fn set_buffer<T: Sized>(&mut self, data: Vec<T>) {
        unsafe {
            filament::IndexBuffer_SetBuffer(
                self.handle,
                self.engine.handle(),
                data.as_ptr() as *mut std::ffi::c_void,
                (std::mem::size_of::<T>() * data.len()) as u64,
                Some(crate::low_level::deallocate_rust_buffer),
            );
        }
        // Forget the vector (will be freed in the deallocate_rust_buffer callback).
        mem::forget(data);
    }
}
