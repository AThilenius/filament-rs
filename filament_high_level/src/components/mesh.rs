#![allow(unused)]
use filament_low_level::{
    engine::Engine,
    index_buffer::{IndexBuffer, IndexType},
    vertex_buffer::VertexBuffer,
};
use nalgebra::{Vector3, Vector4};

pub use filament_low_level::vertex_buffer::{AttributeType, VertexAttribute};

// Offsets and strides (assume all values are 1-Byte)
// ----------------------------------------
// | X  Y  Z  R  G  B |  X  Y  Z  R  G  B |
// ----------------------------------------
// Position { offset: 0, stride: sizeof(Vertex) = 6 }
// Color    { offset: 3, stride: sizeof(Vertex) = 6 }
pub struct VertexAttributeDefinition {
    pub(crate) attribute: VertexAttribute,
    pub(crate) attribute_type: AttributeType,
    pub(crate) attribute_size: u32,
    pub(crate) normalized: bool,
}

impl VertexAttributeDefinition {
    pub fn new(
        attribute: VertexAttribute,
        attribute_type: AttributeType,
        normalized: bool,
    ) -> Self {
        VertexAttributeDefinition {
            attribute,
            attribute_type,
            normalized,
            attribute_size: match attribute_type {
                AttributeType::Byte => 1,
                AttributeType::Byte2 => 2,
                AttributeType::Byte3 => 3,
                AttributeType::Byte4 => 4,
                AttributeType::Ubyte => 1,
                AttributeType::Ubyte2 => 2,
                AttributeType::Ubyte3 => 3,
                AttributeType::Ubyte4 => 4,
                AttributeType::Short => 2,
                AttributeType::Short2 => 4,
                AttributeType::Short3 => 6,
                AttributeType::Short4 => 8,
                AttributeType::Ushort => 2,
                AttributeType::Ushort2 => 4,
                AttributeType::Ushort3 => 6,
                AttributeType::Ushort4 => 8,
                AttributeType::Int => 4,
                AttributeType::Uint => 4,
                AttributeType::Float => 4,
                AttributeType::Float2 => 8,
                AttributeType::Float3 => 12,
                AttributeType::Float4 => 16,
                AttributeType::Half => 2,
                AttributeType::Half2 => 4,
                AttributeType::Half3 => 6,
                AttributeType::Half4 => 8,
            },
        }
    }
}

pub trait VertexDefinition: Sized + Send + Sync {
    fn attribute_definitions() -> Vec<VertexAttributeDefinition>;

    fn make(
        engine: &mut Engine,
        vertices: Vec<Self>,
        indices: Vec<u16>,
    ) -> (VertexBuffer, IndexBuffer)
    where
        Self: Sized,
    {
        let (vertex_count, index_count) = (vertices.len(), indices.len());

        let mut vertex_builder = engine
            .create_vertex_buffer_builder()
            .buffer_count(1)
            .vertex_count(vertex_count as u32);

        let size_of = std::mem::size_of::<Self>();
        let mut offset = 0_u32;

        for attribute_definition in Self::attribute_definitions() {
            vertex_builder = vertex_builder.attribute(
                attribute_definition.attribute,
                0,
                attribute_definition.attribute_type,
                offset,
                size_of as u8,
            );
            offset += attribute_definition.attribute_size;

            if attribute_definition.normalized {
                vertex_builder = vertex_builder.normalized(attribute_definition.attribute, true);
            }
        }

        let mut vertex_buffer = vertex_builder.build();

        vertex_buffer.set_buffer_at(0, vertices);

        let mut index_buffer = engine
            .create_index_buffer_builder()
            .index_count(index_count as u32)
            .buffer_type(IndexType::Ushort)
            .build();

        index_buffer.set_buffer(indices);

        (vertex_buffer, index_buffer)
    }
}

pub struct Mesh {
    pub mesh_handle: u32,
}

impl Mesh {
    pub fn new(mesh_handle: u32) -> Self {
        Self { mesh_handle }
    }
}
