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

const MATERIAL_BYTES: &'static [u8] = include_bytes!("../materials/bin/color_unlit.filamat");

#[repr(C)]
struct Vertex {
  pub position: Vector2<f32>,
  pub color: u32,
}

fn main() {
  let vertices = vec![
    Vertex {
      position: Vector2::new(1.0, 0.0),
      color: 0xffff0000,
    },
    Vertex {
      position: Vector2::new(-0.5, 0.866),
      color: 0xff00ff00,
    },
    Vertex {
      position: Vector2::new(-0.5, -0.866),
      color: 0xff0000ff,
    },
  ];
  let indices: Vec<u16> = vec![0, 1, 2];

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

  let mut vertex_buffer = engine
    .create_vertex_buffer_builder()
    .vertex_count(3)
    .buffer_count(1)
    .attribute(VertexAttribute::Position, 0, AttributeType::Float2, 0, 12)
    .attribute(VertexAttribute::Color, 0, AttributeType::Ubyte4, 8, 12)
    .normalized(VertexAttribute::Color, true)
    .build();
  vertex_buffer.set_buffer_at(0, &vertices);

  let mut index_buffer = engine
    .create_index_buffer_builder()
    .index_count(3)
    .buffer_type(IndexType::Ushort)
    .build();
  index_buffer.set_buffer(&indices);

  let material = engine.create_material(MATERIAL_BYTES);
  let material_instance = material.create_instance();

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
