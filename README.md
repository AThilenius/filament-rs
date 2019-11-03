# Google Filament - Rust Bindings

> Partial bindings with no unit tests... use are your own risk. No `crate` is
> provided for that reason.

Rust bindings for [Google Filament](https://github.com/google/filament).
Filament-rs is split between a (partially complete) low-level API and a (much
less complete) high-level API

The low-level API is semi-idiomatic and only _mostly_ safe. _(It should be safe
once the bindings stabilize.)_ The conforms fairly closely with the original
Filament API, with things like overloading being replaced by enums. All linear
algebra types are marshaled to and from [nalgebra](https://www.nalgebra.org/)
types. See the `Usage: Low-level API` section for details on how to use the
low-level API.

The high-level API is a work in progress. It uses [Legion
ECS](https://github.com/TomGillen/legion) and
[Winit](https://docs.rs/winit/0.20.0-alpha3/winit/) for windowing, without
requiring that the user manually construct things like vertex or index buffers.
Much of the API is inspired by [Amethyst](https://github.com/amethyst/amethyst).

## Building

**Only Windows and OSX are currently supported.** Linux should theoretically be
easy to add, and WASM should also be theoretically possible (would require more
work though).

```sh
# Filament deps will be auto downloaded and extracted to `/target`.
cargo build
```

Filament deps will be downloaded and extracted inside `target` once per target
type. These deps are **surprisingly large**, 436MiB tarball and a 2.77GiB
extracted directory for Windows, at time of writing.

## Usage: Low-level API

**See `examples/triangle` for a complete triangle demo. Use
`cargo run --example triangle` to run.**

Include a compiled Filament `Material` in the binary

```rust
const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/bin/color_unlit.filamat");
```

Define a vertex type (assuming you want to interleave vertex data). This also
defined everything Filament will need to use this struct directly as a for
vertex data.

```rust
// Must be defined `#[repr(C)]` for safe FFI to C.
#[repr(C)]
struct Vertex {
    pub position: Vector2<f32>,
    pub uv: Vector2<f32>,
}
```

Create an `Engine`, `SwapChain`, `Renderer`, `View`, `Camera` and `Scene`.

```rust
let mut engine = Engine::new(Backend::Default);
let swap_chain = engine.create_swap_chain(window_handle);
let renderer = engine.create_renderer();
let mut view = engine.create_view();
let mut scene = engine.create_scene();

// Make the camera
let mut camera = engine.create_camera();
camera.set_projection_fov(60.0, aspect, 0.1, 10000.0, Fov::Vertical);

// Setup the view
view.set_scene(&scene);
view.set_camera(&camera);
view.set_viewport(0, 0, width, height);
view.set_clear_color(0.0, 0.0, 1.0, 1.0);
view.set_clear_targets(true, true, false);
```

Build a Vertex and Index buffer. Note that this will copy both buffers into
unmanaged memory, freeing the buffer once the copy-to-GPU is done, meaning the
Rust-side buffers can be freed at any time after these calls.

```rust
let mut vertex_buffer = engine
    .create_vertex_buffer_builder()
    .vertex_count(3)
    .buffer_count(1)
    .attribute(VertexAttribute::Position, 0, AttributeType::Float2, 0, 12)
    .attribute(VertexAttribute::Color, 0, AttributeType::Ubyte4, 8, 12)
    .normalized(VertexAttribute::Color, true)
    .build();
vertex_buffer.set_buffer_at_copy(0, &vertices);

let mut index_buffer = engine
    .create_index_buffer_builder()
    .index_count(3)
    .buffer_type(IndexType::Ushort)
    .build();
index_buffer.set_buffer_copy(&indices);
```

Create a `Material` and `MaterialInstance` from the embedded bytes above.

```rust
let material = engine.create_material(MATERIAL_BYTES);
let material_instance = material.create_instance();
```

Create an Entity with the `EntityManager` and `RenderableManager` and assign a
renderable to it. Filament uses a simple
[ECS](https://en.wikipedia.org/wiki/Entity_component_system) for entity
management.

```rust
let entity = EntityManager::get().create();
scene.add_entity(entity);

RenderableManager::builder(1)
    .bounding_box(BoundingBox {
        center: Vector3::new(-1., -1., -1.),
        half_extent: Vector3::new(1., 1., 1.),
    })
    .culling(false)
    .material(0, &material_instance)
    .geometry(0, PrimitiveType::Triangles, &vertex_buffer, &index_buffer)
    .build(&engine, entity);
```

Entities can be transformed using the `TransformManager`.

```rust
engine.get_transform_manager().set_transform(
    entity,
    // This is a standard nalgebra Matrix4<f32>.
    Matrix4::new_rotation(Vector3::new(0.0, 0.0, 2.34)),
);
```

In the render loop, begin, draw and end the frame with

```rust
if renderer.begin_frame(&swap_chain) {
    renderer.render(&view);
    renderer.end_frame();
}
```
