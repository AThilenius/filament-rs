pub use crate::{
    engine::Engine,
    index_buffer::IndexBuffer,
    material::MaterialInstance,
    misc_types::{BoundingBox, Entity},
    raw_bindings::filament,
    vertex_buffer::VertexBuffer,
};

pub enum PrimitiveType {
    Point = 0,
    Lines = 1,
    Triangles = 4,
    None = 0xFF,
}

pub struct RenderableManager;

impl RenderableManager {
    pub fn builder(count: u32) -> RenderableManagerBuilder {
        RenderableManagerBuilder::new(unsafe { filament::RenderableManager_Builder_Create(count) })
    }
}

pub struct RenderableManagerBuilder {
    pub(crate) handle: *mut filament::RenderableManagerBuilder,
}

impl RenderableManagerBuilder {
    pub(self) fn new(handle: *mut filament::RenderableManagerBuilder) -> Self {
        Self { handle }
    }

    pub fn bounding_box(self, bounding_box: BoundingBox) -> Self {
        let bb_ptr: *const BoundingBox = &bounding_box;
        unsafe {
            filament::RenderableManager_Builder_SetBoundingBox(self.handle, bb_ptr as *mut f32);
        }
        self
    }

    pub fn culling(self, culled: bool) -> Self {
        unsafe {
            filament::RenderableManager_Builder_SetCulling(self.handle, culled);
        }

        self
    }

    pub fn material(self, index: u64, material_instance: &MaterialInstance) -> Self {
        unsafe {
            filament::RenderableManager_Builder_SetMaterial(
                self.handle,
                index,
                material_instance.handle,
            );
        }

        self
    }

    pub fn geometry(
        self,
        index: u64,
        primitive_type: PrimitiveType,
        vertex_buffer: &VertexBuffer,
        index_buffer: &IndexBuffer,
    ) -> Self {
        unsafe {
            filament::RenderableManager_Builder_SetGeometry(
                self.handle,
                index,
                primitive_type as i32,
                vertex_buffer.handle,
                index_buffer.handle,
            );
        }

        self
    }

    pub fn build(self, engine: &Engine, entity: Entity) {
        unsafe {
            filament::RenderableManager_Builder_Build(self.handle, engine.handle(), entity);
        }
    }
}
