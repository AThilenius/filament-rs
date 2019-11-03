extern crate filament;
extern crate winit;

use filament::prelude::*;
use nalgebra::{Matrix4, Vector2, Vector3};
use std::{
    thread::sleep,
    time::{Duration, Instant},
};
use winit::{Event, WindowEvent};

mod window_helpers;
use window_helpers::init_window;

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/bin/texture_unlit.filamat");

#[repr(C)]
#[derive(Clone, Default)]
struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[repr(C)]
struct Vertex {
    pub position: Vector2<f32>,
    pub uv: Vector2<f32>,
}

impl VertexDefinition for Vertex {
    fn attribute_definitions() -> Vec<VertexAttributeDefinition> {
        vec![
            VertexAttributeDefinition::new(VertexAttribute::Position, AttributeType::Float2, false),
            VertexAttributeDefinition::new(VertexAttribute::UV0, AttributeType::Float2, false),
        ]
    }
}

fn main() {
    let vertices = vec![
        Vertex {
            position: Vector2::new(1.0, 0.0),
            uv: Vector2::new(1.0, 0.0),
        },
        Vertex {
            position: Vector2::new(0.5, 1.0),
            uv: Vector2::new(0.0, 1.0),
        },
        Vertex {
            position: Vector2::new(-0.5, 0.0),
            uv: Vector2::new(0.0, 0.0),
        },
    ];
    let indices: Vec<u16> = vec![0, 1, 2];

    // Generate some simple data for a 256x256 RGB texture.
    let mut texture_data = vec![RgbColor::default(); 256 * 256];
    for y in 0..256 {
        for x in 0..256 {
            texture_data[y * 256 + x] = RgbColor {
                r: x as u8,
                g: y as u8,
                b: 0,
            };
        }
    }

    let (window, mut event_loop, window_handle) = init_window();
    let hidpi = window.get_hidpi_factor();
    let (width, height) = window.get_inner_size().unwrap().to_physical(hidpi).into();
    let aspect = width as f64 / height as f64;

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

    // Use the vertex definition to build out a vertex and index buffer from striped data.
    let (vertex_buffer, index_buffer) = Vertex::make(&mut engine, vertices, indices);

    // Make the sampler and texture from the simple texture data above.
    let sampler = TextureSampler::default();
    let mut texture = Texture::new_standard(&engine, 256, 256, TextureFormat::RGB8);
    texture.set_image_copy(
        &texture_data,
        256,
        256,
        PixelDataType::Ubyte,
        PixelDataFormat::RGB,
    );

    let material = engine.create_material(MATERIAL_BYTES);
    let mut material_instance = material.create_instance();
    material_instance.set_parameter(MaterialParameter::Texture("texture", &texture, sampler));

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

    let mut exit = false;
    let target_frame_time = Duration::from_secs(1) / 144;
    let total_time = Instant::now();

    while !exit {
        let frame_timer = Instant::now();

        // Poll window events first.
        event_loop.poll_events(|event| match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => {
                exit = true;
            }
            _ => {}
        });

        // Rotate the triangle and dolly the camera out.
        let elapsed_seconds = total_time.elapsed().as_millis() as f32 / 1000.0_f32;
        engine.get_transform_manager().set_transform(
            entity,
            Matrix4::new_rotation(Vector3::new(0.0, 0.0, elapsed_seconds)),
        );
        camera.set_model_matrix(Matrix4::new_translation(&Vector3::new(
            0.0,
            0.0,
            1.0 + (elapsed_seconds / 5.0),
        )));

        // Then try to begin another frame (returns false if we need to skip a frame).
        if renderer.begin_frame(&swap_chain) {
            renderer.render(&view);
            renderer.end_frame();
        }

        let title = format!(
            "Quad Example - Last frame: {:.2}ms",
            (frame_timer.elapsed().as_micros() as f64) / 1000_f64
        );
        window.set_title(&title);

        // Try to sleep long enough to hit the target frame time.
        if frame_timer.elapsed() < target_frame_time {
            sleep(target_frame_time - frame_timer.elapsed());
        }
    }
}
