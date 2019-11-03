# Google Filament - Rust Bindings

> Partial bindings with no unit tests... use are your own risk. No `crate` is
> provided for that reason.

Partial Rust bindings for [Google Filament](https://github.com/google/filament).
These bindings are semi-idiomatic, the API itself conforms very closely with
that of Filament. All linear-algebra types are marshaled to and from
[nalgebra](https://www.nalgebra.org/) types. No windowing code is included, the
`examples` use [winit](https://docs.rs/winit/0.20.0-alpha3/winit/) to create a
window, pump events, and most importantly get a native surface handle to pass to
Filament (see `/examples/init_and_shutdown.rs` for a minimal windowing and init
example).

## Building

**Only Windows and OSX are currently supported.**

```sh
# Filament deps will be auto downloaded and extracted to `/target`.
cargo build
```

Filament deps will be downloaded and extracted inside `target` once per target
type. These deps are **surprisingly large**, 436MiB tarball and a 2.77GiB
extracted directory for Windows, at time of writing.

## Usage

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

// Impl `VertexDefinition` to provide Filament with all the info it needs to build `VertexBuffer`s from Vectors of this
// struct.
impl VertexDefinition for Vertex {
    fn attribute_definitions() -> Vec<VertexAttributeDefinition> {
        vec![
            VertexAttributeDefinition::new(VertexAttribute::Position, AttributeType::Float2, false),
            VertexAttributeDefinition::new(VertexAttribute::UV0, AttributeType::Float2, false),
        ]
    }
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
unmanaged memory, freeing the buffer once the copy-to-GPU is done.

```rust
// Uses the interleaved vertex struct defined above.
let (vertex_buffer, index_buffer) = Vertex::make(&mut engine, vertices, indices);
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
