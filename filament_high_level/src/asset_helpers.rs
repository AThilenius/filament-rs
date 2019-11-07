use crate::components::*;
use legion::prelude::*;

pub fn load_mesh_bytes<T>(world: &mut World, vertices: Vec<T>, indices: Vec<u16>) -> Entity
where
    T: VertexDefinition + 'static,
{
    *world
        .insert((), vec![(MeshBytesLoadRequest::new(vertices, indices),)])
        .first()
        .unwrap()
}

pub fn load_texture_bytes_standard<T>(
    world: &mut World,
    width: u32,
    height: u32,
    format: TextureFormat,
    pixel_data_type: PixelDataType,
    pixel_data_format: PixelDataFormat,
    pixel_data: Vec<T>,
) -> Entity
where
    T: Sized,
{
    *world
        .insert(
            (),
            vec![(TextureBytesLoadRequest::new_standard(
                width,
                height,
                format,
                pixel_data_type,
                pixel_data_format,
                pixel_data,
            ),)],
        )
        .first()
        .unwrap()
}

pub fn load_material_template_bytes(world: &mut World, bytes: Vec<u8>) -> Entity {
    *world
        .insert((), vec![(MaterialTemplateBytesLoadRequest::new(bytes),)])
        .first()
        .unwrap()
}

pub fn create_material(world: &mut World, material_template: Entity) -> Entity {
    *world
        .insert(
            (),
            vec![(MaterialInstantiateRequest::new(material_template),)],
        )
        .first()
        .unwrap()
}
