use crate::{high_level::components::*, low_level::prelude as ll, ThreadLocalSystem};
use legion::prelude::*;
use std::collections::HashMap;
use winit::Window;

pub fn build(world: &mut World) -> ThreadLocalSystem {
    let window = world.resources.get::<Window>().unwrap();
    let hidpi = window.get_hidpi_factor();
    let (width, height) = window.get_inner_size().unwrap().to_physical(hidpi).into();
    let aspect = width as f64 / height as f64;

    let mut engine = ll::Engine::new(ll::Backend::Default);
    let swap_chain = engine.create_swap_chain(get_active_surface(&window));
    let renderer = engine.create_renderer();
    let mut view = engine.create_view();
    let scene = engine.create_scene();

    // Make the camera
    let mut camera = engine.create_camera();
    camera.set_projection_fov(60.0, aspect, 0.1, 10000.0, ll::Fov::Vertical);

    // Setup the view
    view.set_scene(&scene);
    view.set_camera(&camera);
    view.set_viewport(0, 0, width, height);
    view.set_clear_color(0.0, 0.0, 1.0, 1.0);
    view.set_clear_targets(true, true, false);

    // Where all ll resource handles are stored.
    let mut next_id = 0_u32;
    let mut buffers = HashMap::new();
    let mut materials = HashMap::new();

    Box::new(move |world| {
        // For each MeshBytesLoadRequest, load the filament vertex and index buffers.
        let command_buffer = CommandBuffer::default();
        for (entity, mut req) in <Write<MeshBytesLoadRequest>>::query().iter_entities(world) {
            if req.data.is_none() {
                continue;
            }

            let (vertices, indices) = req.data.take().unwrap();
            let (vertices_len, indices_len) = (vertices.len(), indices.len());

            let mut vertex_builder = engine
                .create_vertex_buffer_builder()
                .buffer_count(1)
                .vertex_count(vertices_len as u32);

            let mut offset = 0_u32;

            for attribute_definition in req.definitions.iter() {
                vertex_builder = vertex_builder.attribute(
                    attribute_definition.attribute,
                    0,
                    attribute_definition.attribute_type,
                    offset,
                    req.size_of_t as u8,
                );
                offset += attribute_definition.attribute_size;

                if attribute_definition.normalized {
                    vertex_builder =
                        vertex_builder.normalized(attribute_definition.attribute, true);
                }
            }

            let mut vertex_buffer = vertex_builder.build();

            // 'Transfers ownership' of vertices to unmanaged code.
            vertex_buffer.set_buffer_at(0, vertices);

            let mut index_buffer = engine
                .create_index_buffer_builder()
                .index_count(indices_len as u32)
                .buffer_type(ll::IndexType::Ushort)
                .build();

            // 'Transfers ownership' of indices to unmanaged code.
            index_buffer.set_buffer(indices);

            // Create a resource handle for it, and add it to the entity.
            buffers.insert(next_id, (vertex_buffer, index_buffer, indices_len));
            command_buffer.add_component(entity, Mesh(next_id));
            next_id += 1;
        }
        command_buffer.write(world);

        // For each MaterialBytesLoadRequest, load the filament material.
        let command_buffer = CommandBuffer::default();
        for (entity, req) in <Read<MaterialBytesLoadRequest>>::query().iter_entities(world) {
            let material = engine.create_material(&req.data);

            // Create a resource handle for it, and add it to the entity.
            materials.insert(next_id, material);
            command_buffer.remove_component::<MaterialBytesLoadRequest>(entity);
            command_buffer.add_component(entity, MaterialTemplate(next_id));
            next_id += 1;
        }
        command_buffer.write(world);

        // For each MaterialInstantiateRequest, create a Filament material.
        // TODO

        // Then try to begin another frame (returns false if we need to skip a frame).
        if renderer.begin_frame(&swap_chain) {
            renderer.render(&view);
            renderer.end_frame();
        }
    })
}

#[cfg(target_os = "macos")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
    use winit::os::macos::WindowExt;
    window.get_nsview()
}

#[cfg(target_os = "windows")]
fn get_active_surface(window: &Window) -> *mut std::ffi::c_void {
    use winit::os::windows::WindowExt;
    window.get_hwnd()
}
