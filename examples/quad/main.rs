extern crate filament;
extern crate winit;

use filament::prelude::*;
use nalgebra::{Matrix4, Perspective3, Vector3};
use std::time::{Duration, Instant};
use winit::{Event, WindowEvent};

mod window_helpers;
use window_helpers::init_window;

fn make_vertex_buffer(engine: &mut Engine) -> VertexBuffer {
  let mut vertex_buffer = engine
    .create_vertex_buffer_builder()
    .vertex_count(4)
    .buffer_count(2)
    .attribute(VertexAttribute::Position, 0, AttributeType::Float3, 0, 0)
    .attribute(VertexAttribute::Tangents, 1, AttributeType::Short4, 0, 0)
    .normalized(VertexAttribute::Tangents, true)
    .build();

  let vertices: Vec<Vector3<f32>> = vec![
    Vector3::new(-1.0, -1.0, 0.0),
    Vector3::new(1.0, -1.0, 0.0),
    Vector3::new(1.0, 1.0, 0.0),
    Vector3::new(-1.0, 1.0, 0.0),
  ];

  let normals: Vec<Vector3<f32>> = vec![
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
    Vector3::new(0.0, 0.0, 1.0),
  ];

  vertex_buffer.set_buffer_at(0, &vertices);
  vertex_buffer.set_buffer_at(1, &normals);

  vertex_buffer
}

fn make_index_buffer(engine: &mut Engine) -> IndexBuffer {
  let mut index_buffer = engine.create_index_buffer_builder()
    .index_count(6)
    .buffer_type(IndexType::Ushort)
    .build();

  let indices: Vec<u16> = vec![0, 1, 2, 2, 3, 0];

  index_buffer.set_buffer(&indices);

  index_buffer
}

fn main() {
  let (mut window, mut event_loop, window_handle) = init_window("Quad Example");

  let mut engine = Engine::new(Backend::Opengl);
  let swap_chain = engine.create_swap_chain(window_handle);
  let renderer = engine.create_renderer();
  let mut view = engine.create_view();
  let mut scene = engine.create_scene();

  // Make the camera
  let mut camera = engine.create_camera();
  let content_size = window.get_inner_size().unwrap();
  camera.set_projection_fov(
    45.0,
    content_size.width / content_size.height,
    0.1,
    10000.0,
    Fov::Vertical,
  );

  // Setup the view
  view.set_viewport(0, 0, content_size.width as i32, content_size.height as i32);
  view.set_camera(&camera);
  view.set_scene(&scene);

  let vertex_buffer = make_vertex_buffer(&mut engine);
  let index_buffer = make_index_buffer(&mut engine);

  let mut exit = false;
  let target_frame_time = Duration::from_secs(1) / 144;

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

    // Then try to begin another frame (returns false if we need to skip a frame).
    if !renderer.begin_frame(&swap_chain) {
      continue;
    }
    renderer.render(&view);
    renderer.end_frame();

    let title = format!("Quad Example - Last frame: {}ms", frame_timer.elapsed().as_micros());
    window.set_title(&title);

    // Try to sleep long enough to hit the target frame time.
    if frame_timer.elapsed() < target_frame_time {
      std::thread::sleep(target_frame_time - frame_timer.elapsed());
    }
  }
}
