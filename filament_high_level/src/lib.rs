// Thoughts on the high-level API:

// The main interface with it should be via Components (data driven). There will be a system bundle
// that needs to be instantiated and ticked, but that should be a small chunk of the API.

// Resources should be stored as Entities (technically the data will be heap-allocated and the
// entity will just be a pointer to it).

// Full load cycle then would be:

// > let mesh_handle = *world.insert((), vec![(MeshFileLoadRequest("some.obj"),)]).first().unwrap();
// Causes the cascade:
//   -> C:MeshFileLoadRequest(String)
//   -> S:FileLoadSystem
//   -> C:MeshFormatLoadRequest(Box<Bytes>, Format)
//   -> S:MeshFormatLoadSystem
// * -> C:MeshBytesLoadRequest(Box<Vertices>, Box<Indices>, VertexDefinitionParams)
// * -> S:MeshBytesLoadSystem

// >let texture_handle = *world.insert((), vec![(TextureFileLoadRequest("normal.png"),)]).first()..;
// Causes the cascade:
//   -> C:TextureFileLoadRequest(String)
//   -> S:FileLoadSystem
//   -> C:TextureFormatLoadRequest(Box<Bytes>, Format)
//   -> S:TextureFormatLoadSystem,
// * -> C:TextureBytesLoadRequest(Box<Bytes>, TextureFormatInfo)
// * -> S:TextureBytesLoadSystem

// >let material_handle = *world.insert((), vec![(MaterialBytesLoadRequest(BYTES),)]).first()...;
// Causes the cascade:
// * -> C:MaterialBytesLoadRequest(Bytes)
// * -> S:MaterialLoadSystem

// >let material_instance_handle = *world.insert((), vec![
//    (MaterialInstanceCreateRequest(material_handle, ),)
//  ]).first().unwrap();
// Causes the cascade:
// * -> C:MaterialBytesLoadRequest(Bytes)
// * -> S:MaterialLoadSystem

extern crate log;

pub mod asset_helpers;
pub mod components;
pub mod engine;
pub mod input_handler;
pub mod input_system;
pub mod prelude;
pub mod rendering_system;
pub mod window_system;

pub type ThreadLocalSystem = Box<dyn FnMut(&mut legion::prelude::World)>;
