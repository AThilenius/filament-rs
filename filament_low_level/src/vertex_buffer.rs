use crate::{engine::Engine, raw_bindings::*};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VertexAttribute {
    Position = 0,
    Tangents = 1,
    Color = 2,
    UV0 = 3,
    UV1 = 4,
    BoneIndices = 5,
    BoneWeights = 6,
    Custom0 = 8,
    Custom1 = 9,
    Custom2 = 10,
    Custom3 = 11,
    Custom4 = 12,
    Custom5 = 13,
    Custom6 = 14,
    Custom7 = 15,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum AttributeType {
    Byte = 0,
    Byte2 = 1,
    Byte3 = 2,
    Byte4 = 3,
    Ubyte = 4,
    Ubyte2 = 5,
    Ubyte3 = 6,
    Ubyte4 = 7,
    Short = 8,
    Short2 = 9,
    Short3 = 10,
    Short4 = 11,
    Ushort = 12,
    Ushort2 = 13,
    Ushort3 = 14,
    Ushort4 = 15,
    Int = 16,
    Uint = 17,
    Float = 18,
    Float2 = 19,
    Float3 = 20,
    Float4 = 21,
    Half = 22,
    Half2 = 23,
    Half3 = 24,
    Half4 = 25,
}

pub struct VertexBufferBuilder {
    engine: Engine,
    pub(crate) handle: *mut filament::VertexBufferBuilder,
}

impl Drop for VertexBufferBuilder {
    fn drop(&mut self) {
        unsafe {
            filament::VertexBuffer_DestroyBuilder(self.handle);
        }
    }
}

impl VertexBufferBuilder {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::VertexBuffer_CreateBuilder() },
            engine,
        }
    }

    pub fn vertex_count(self, vertex_count: u32) -> Self {
        unsafe {
            filament::VertexBuffer_BuilderVertexCount(self.handle, vertex_count);
        }
        self
    }

    pub fn buffer_count(self, buffer_count: u8) -> Self {
        unsafe {
            filament::VertexBuffer_BuilderBufferCount(self.handle, buffer_count);
        }
        self
    }

    pub fn attribute(
        self,
        attribute: VertexAttribute,
        buffer_index: u8,
        attribute_type: AttributeType,
        byte_offset: u32,
        byte_stride: u8,
    ) -> Self {
        unsafe {
            filament::VertexBuffer_BuilderAttribute(
                self.handle,
                attribute as i32,
                buffer_index,
                attribute_type as i32,
                byte_offset,
                byte_stride,
            );
        }
        self
    }

    pub fn normalized(self, attribute: VertexAttribute, normalized: bool) -> Self {
        unsafe {
            filament::VertexBuffer_BuilderNormalized(self.handle, attribute as i32, normalized);
        }
        self
    }

    pub fn build(self) -> VertexBuffer {
        let handle =
            unsafe { filament::VertexBuffer_BuilderBuild(self.handle, self.engine.handle()) };
        VertexBuffer::new(handle, self.engine.clone())
    }
}

#[derive(Clone)]
pub struct VertexBuffer {
    engine: Engine,
    pub(crate) handle: *mut filament::VertexBuffer,
}

impl VertexBuffer {
    pub(self) fn new(handle: *mut filament::VertexBuffer, engine: Engine) -> Self {
        Self { handle, engine }
    }

    pub fn get_vertex_count(&self) -> u64 {
        unsafe { filament::VertexBuffer_GetVertexCount(self.handle) }
    }

    pub fn set_buffer_at<T: Sized>(&mut self, buffer_index: u8, data: Vec<T>) {
        unsafe {
            filament::VertexBuffer_SetBufferAt(
                self.handle,
                self.engine.handle(),
                buffer_index,
                data.as_ptr() as *mut std::ffi::c_void,
                (std::mem::size_of::<T>() * data.len()) as u64,
                Some(crate::deallocate_rust_buffer),
            );
            // Forget the vector (will be freed in the deallocate_rust_buffer callback).
            std::mem::forget(data);
        }
    }
}
