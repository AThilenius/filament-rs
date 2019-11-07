use crate::{components::*, ThreadLocalSystem};
use filament_low_level::prelude as ll;
use legion::prelude::*;
use nalgebra::Vector3;
use std::collections::HashMap;
use winit::Window;

struct RenderableLink {
    pub filament_entity: u32,
}

impl RenderableLink {
    pub fn new(filament_entity: u32) -> Self {
        Self { filament_entity }
    }
}

pub fn build(world: &mut World) -> ThreadLocalSystem {
    let window = world.resources.get::<Window>().unwrap();
    let hidpi = window.get_hidpi_factor();
    let (width, height) = window.get_inner_size().unwrap().to_physical(hidpi).into();
    let aspect = width as f64 / height as f64;

    log::trace!("Creating a Filament Engine, SwapChain and Renderer");
    let mut engine = ll::Engine::new(ll::Backend::Default);
    let swap_chain = engine.create_swap_chain(get_active_surface(&window));
    let renderer = engine.create_renderer();
    let mut view = engine.create_view();
    let mut scene = engine.create_scene();

    // Make the camera
    let mut camera = engine.create_camera();
    camera.set_projection_fov(60.0, aspect, 0.1, 10000.0, ll::Fov::Vertical);

    // Setup the view
    log::trace!("Initialize Filament View");
    view.set_scene(&scene);
    view.set_camera(&camera);
    view.set_viewport(0, 0, width, height);
    view.set_clear_color(0.0, 0.0, 1.0, 1.0);
    view.set_clear_targets(true, true, false);

    // Where all low_level resource handles are stored.
    let mut next_id = 0_u32;
    let mut buffers = HashMap::new();
    let mut material_templates = HashMap::new();
    let mut materials = HashMap::new();
    let mut textures = HashMap::new();

    Box::new(move |world| {
        // Sync: For each MeshBytesLoadRequest, load the filament vertex and index buffers.
        let command_buffer = CommandBuffer::default();
        for (entity, mut req) in <Write<MeshBytesLoadRequest>>::query().iter_entities(world) {
            if req.data.is_none() {
                continue;
            }

            let (vertices, indices) = req.data.take().unwrap();
            let (vertices_len, indices_len) = (vertices.len(), indices.len());

            log::trace!(
                "Start processing MeshBytesLoadRequest: {} vertices, {} indices",
                vertices_len,
                indices_len
            );

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
            command_buffer.add_component(entity, Mesh::new(next_id));
            next_id += 1;

            log::trace!("End processing MeshBytesLoadRequest");
        }
        command_buffer.write(world);

        // Sync: For each TextureBytesLoadRequest, load the texture and data.
        let command_buffer = CommandBuffer::default();
        for (entity, mut req) in <Write<TextureBytesLoadRequest>>::query().iter_entities(world) {
            log::trace!("Start processing TextureBytesLoadRequest");

            let mut texture = ll::Texture::new(
                &engine,
                req.width,
                req.height,
                req.depth,
                req.levels,
                req.sampler_type,
                req.format,
                req.usage_flags,
            );
            texture.set_image(
                req.pixel_data.take().unwrap(),
                req.width,
                req.height,
                req.pixel_data_type,
                req.pixel_data_format,
            );
            // Create a resource handle for it, and add it to the entity.
            textures.insert(next_id, texture);
            command_buffer.remove_component::<TextureBytesLoadRequest>(entity);
            command_buffer.add_component(entity, Texture::new(next_id));
            next_id += 1;

            log::trace!("End processing TextureBytesLoadRequest");
        }
        command_buffer.write(world);

        // Sync: For each MaterialBytesLoadRequest, load the filament material.
        let command_buffer = CommandBuffer::default();
        for (entity, req) in <Read<MaterialTemplateBytesLoadRequest>>::query().iter_entities(world)
        {
            log::trace!("Start processing MaterialTemplateBytesLoadRequest");
            let material = engine.create_material(&req.data);

            // Create a resource handle for it, and add it to the entity.
            material_templates.insert(next_id, material);
            command_buffer.remove_component::<MaterialTemplateBytesLoadRequest>(entity);
            command_buffer.add_component(entity, MaterialTemplate::new(next_id));
            next_id += 1;
            log::trace!("End processing MaterialTemplateBytesLoadRequest");
        }
        command_buffer.write(world);

        // Async(MaterialTemplate): For each MaterialInstantiateRequest, create a Filament material.
        let command_buffer = CommandBuffer::default();
        for (entity, req) in <Read<MaterialInstantiateRequest>>::query().iter_entities(world) {
            if let Some(material_template) =
                world.get_component::<MaterialTemplate>(req.material_template)
            {
                log::trace!("Start processing (async, ready) MaterialInstantiateRequest");
                let material_template = material_templates
                    .get(&material_template.material_template_handle)
                    .unwrap();
                let instance = material_template.create_instance();
                materials.insert(next_id, instance);
                command_buffer.remove_component::<MaterialInstantiateRequest>(entity);
                command_buffer.add_component(entity, Material::new(next_id, req.material_template));
                next_id += 1;
                log::trace!("End processing MaterialInstantiateRequest");
            }
        }
        command_buffer.write(world);

        // Async(Mesh, MaterialInstance, Texture): For each RenderMesh that changed (or if the
        // Material itself changed), update the params for the material. Also create a
        // RenderableLink if one is missing.
        let command_buffer = CommandBuffer::default();
        'entity_loop: for (entity, render_mesh) in <Read<RenderMesh>>::query().iter_entities(world)
        {
            // Ensure the mesh is loaded.
            if world.get_component::<Mesh>(render_mesh.mesh).is_none() {
                log::trace!("Skipping RenderMesh load: Mesh is not loaded.");
                continue 'entity_loop;
            }

            // Ensure the Material is loaded.
            if world
                .get_component::<Material>(render_mesh.material)
                .is_none()
            {
                log::trace!("Skipping RenderMesh load: Material is not loaded.");
                continue 'entity_loop;
            }

            // Ensure all reference params are loaded (just Textures).
            for (_, param) in render_mesh.parameter_binds.iter() {
                if let MaterialParameterBind::Texture(entity, _) = param {
                    if world.get_component::<Texture>(*entity).is_none() {
                        log::trace!("Skipping RenderMesh load: Texture is not loaded.");
                        continue 'entity_loop;
                    }
                }
            }

            log::trace!("Start processing (async, ready) RenderMesh");

            // If it's missing a `RenderableLink`, then create that as everything is ready to go.
            if world.get_component::<RenderableLink>(entity).is_none() {
                // Build and attach a renderable to the entity.
                let f_entity = ll::EntityManager::get().create();
                let (vertex_buffer, index_buffer, _index_count) = buffers
                    .get(
                        &world
                            .get_component::<Mesh>(render_mesh.mesh)
                            .unwrap()
                            .mesh_handle,
                    )
                    .unwrap();
                let material = materials
                    .get(
                        &world
                            .get_component::<Material>(render_mesh.material)
                            .unwrap()
                            .instance_handle,
                    )
                    .unwrap();
                scene.add_entity(f_entity);
                ll::RenderableManager::builder(1)
                    // TODO: Need to compute actual bounding boxes.
                    .bounding_box(ll::BoundingBox {
                        center: Vector3::new(-1., -1., -1.),
                        half_extent: Vector3::new(1., 1., 1.),
                    })
                    .culling(false)
                    .material(0, &material)
                    .geometry(
                        0,
                        ll::PrimitiveType::Triangles,
                        &vertex_buffer,
                        &index_buffer,
                    )
                    .build(&engine, f_entity);

                // The Filament entity is just a u32, so it's attached directly to the legion
                // component. This reduces indirection by one while iterating renderables.
                command_buffer.remove_component::<RenderMesh>(entity);
                command_buffer.add_component(entity, RenderableLink::new(f_entity));
            }

            // Set all material parameters.
            let material_handle = world
                .get_component::<Material>(render_mesh.material)
                .unwrap()
                .instance_handle;
            let material = materials.get_mut(&material_handle).unwrap();
            for (name, param) in render_mesh.parameter_binds.iter() {
                match param {
                    MaterialParameterBind::Texture(entity, sampler) => {
                        let texture_handle = world
                            .get_component::<Texture>(*entity)
                            .unwrap()
                            .texture_handle;
                        let texture = textures.get(&texture_handle).unwrap();
                        material.set_parameter(
                            name,
                            ll::MaterialParameterBind::Texture(&texture, *sampler),
                        );
                    }
                    _ => panic!("Unsupported parameter type {:?}", param),
                }
            }

            log::trace!("End processing RenderMesh");
        }
        command_buffer.write(world);

        // Update the transform matrix for all entities with both a `RenderableLink` and a
        // `LocalToWorld`.
        for (renderable_link, local_to_world) in
            <(Read<RenderableLink>, Read<LocalToWorld>)>::query()
                .filter(changed::<RenderableLink>() | changed::<LocalToWorld>())
                .iter(world)
        {
            engine
                .get_transform_manager()
                .set_transform(renderable_link.filament_entity, local_to_world.0);
        }

        // Update the camera matrix.
        if let Some(main_camera) = world.resources.get::<MainCamera>() {
            if let Some(camera_cmp) = world.get_component::<Camera>(main_camera.entity) {
                if let Some(local_to_world) =
                    world.get_component::<LocalToWorld>(main_camera.entity)
                {
                    // Set the projection matrix (update the aspect if needed).
                    match *camera_cmp {
                        Camera::Perspective(mut perspective3) => {
                            let window = world.resources.get::<Window>().unwrap();
                            let hidpi = window.get_hidpi_factor();
                            let (width, height): (f64, f64) =
                                window.get_inner_size().unwrap().to_physical(hidpi).into();
                            perspective3.set_aspect(width / height);
                            camera.set_custom_projection(
                                perspective3.to_homogeneous(),
                                perspective3.znear(),
                                perspective3.zfar(),
                            );
                        }
                        Camera::Orthographic(orthographic3) => {
                            camera.set_custom_projection(
                                orthographic3.to_homogeneous(),
                                orthographic3.znear(),
                                orthographic3.zfar(),
                            );
                        }
                    }

                    // Update the model matrix (view matrix because it's a camera).
                    camera.set_model_matrix(local_to_world.0);
                } else {
                    panic!(
                        "The entity MainCamera points to must also have a LocalToWorld component"
                    );
                }
            } else {
                panic!("The MainCamera points to an entity without a Camera component on it.");
            }
        } else {
            panic!("No main camera set. Add a `MainCamera` as a World Resource.");
        }

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
