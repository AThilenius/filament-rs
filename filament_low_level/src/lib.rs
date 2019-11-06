pub mod camera;
pub mod engine;
pub mod entity_manager;
pub mod index_buffer;
pub mod material;
pub mod misc_types;
pub mod prelude;
pub mod raw_bindings;
pub mod renderable_manager;
pub mod renderer;
pub mod scene;
pub mod texture;
pub mod texture_sampler;
pub mod vertex_buffer;
pub mod view;

#[no_mangle]
/// A callback from Filament to de-allocate a buffer (after it has been copied to the GPU).
pub(crate) unsafe extern "C" fn deallocate_rust_buffer(
    ptr: *mut std::ffi::c_void,
    size: u64,
    _user: *mut std::ffi::c_void,
) {
    let size = size as usize;
    drop(Vec::from_raw_parts(ptr, size, size));
}
