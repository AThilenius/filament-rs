use crate::mesh::{VertexAttributeDefinition, VertexDefinition};

pub struct MeshBytesLoadRequest {
    pub(crate) size_of_t: usize,
    pub(crate) data: Option<(Vec<u8>, Vec<u16>)>,
    pub(crate) definitions: Vec<VertexAttributeDefinition>,
}

impl MeshBytesLoadRequest {
    pub fn new<T>(vertices: Vec<T>, indices: Vec<u16>) -> Self
    where
        T: VertexDefinition + 'static,
    {
        // Convert to a Vec<u8> and forget the old vector.
        let vertices_u8: Vec<u8> = unsafe {
            Vec::from_raw_parts(
                vertices.as_ptr() as *mut u8,
                vertices.len() * std::mem::size_of::<T>(),
                vertices.capacity() * std::mem::size_of::<T>(),
            )
        };
        std::mem::forget(vertices);

        assert!(
            std::mem::size_of::<T>() <= 255,
            "The size of all vertex types must be less than 255 bytes."
        );

        MeshBytesLoadRequest {
            size_of_t: std::mem::size_of::<T>(),
            data: Some((vertices_u8, indices)),
            definitions: T::attribute_definitions(),
        }
    }
}
