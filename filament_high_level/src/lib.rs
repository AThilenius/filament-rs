// Thoughts on the high-level API:
//
// The main interface with it should be via Components (data driven). There will be a system bundle
// that needs to be instantiated and ticked, but that should be a small chunk of the API.
//
// Resources should be stored as Entities (technically the data will be heap-allocated and the
// entity will just be a pointer to it).
//
// What about using Resources for a world?

// Meshes, Materials and Textures should be reference-counted. Each one should be a handle, which
// can point to a null resource (for when it's still loading).

// There are two separate things:
// - Resource data
//   - Async and Multi-threaded: Loaded off disk/network, processed (ex. tessellated)
//   - Main thread: Upload GPU data (ex. VertexBuffer, IndexBuffer, Texture...)
// - Tag components that point to those resources (including those still loading).

// A complex entity renderable might be wired thusly:
// RenderMeshTag {
//   mesh: MeshResourceHandle,
//   material: MaterialHandle,

//   // The material instance can have a Texture as a param. That can be lazy-loaded as well.
//   material_instance: MaterialInstanceHandle,
// }

// Ideally you create all of this declaratively, and the wiring and reactive part happens behind the
// scenes:

// let texture = Texture::from_path("normal_map.png", TextureFormat::RGB);
// let material = Material::from_bytes(MATERIAL_BYTES);
// let material_instance = MaterialInstance::new(material, vec![
//   // Note that this is a Texture HANDLE, meaning this material instance is stalled until the
//   // texture is loaded.
//   MaterialParameter::Texture("normalMap", texture, TextureSampler::default()),
//   MaterialParameter::Float("deltaTime", delta_time),
// ]);
// let mesh = Mesh::from(vertices, indices);

// This builds up a graph of Resources that need to be loaded:
// - Material (Param was blocked on Texture creation)
// - Texture "normal_map.png": Load bytes from disk, deserialize, create Filament Texture, load data
//   to the GPU.
// - MaterialInstance (Requires: Material & Texture)

// Think about how this is actually going to be used...
// - A user will have a `RenderVolume(sha1)` component attached to their world.
// - A system will pick this up and:
//   - (Async) Load Voxel data from net RPC query (can cache).
//   - (Parallel) Tesselate the Volume into a Vertex/Index array.
//   - (MainThread) Load those to VertexBuffer and IndexBuffer.

// So why not have 'provider' component just like LocalToWorld.
// It's up to the user to populate the provider components.

// Mesh, Texture and Material (compiled bytes) can all be loaded from unusual places (disk, network,
// something else). Those need to have 'provider' components.

// VertexBuffer, IndexBuffer, MaterialInstance can all be loaded without providers.

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

#[macro_use]
extern crate shrinkwraprs;

pub mod components;
pub mod input_handler;
pub mod input_system;
pub mod mesh;
pub mod prelude;
pub mod rendering_system;
pub mod window_system;

pub type ThreadLocalSystem = Box<dyn FnMut(&mut legion::prelude::World)>;
