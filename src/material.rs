use crate::{engine::Engine, raw_bindings::*};

pub struct Material {
  pub(crate) handle: *mut filament::Material,
}

impl Material {
  pub(crate) fn new(engine: Engine, data: &[u8]) -> Self {
    Self {
      handle: unsafe {
        filament::Material_Create(
          engine.handle(),
          data.as_ptr() as *mut std::ffi::c_void,
          data.len() as u64,
        )
      },
    }
  }

  pub fn create_instance(&self) -> MaterialInstance {
    MaterialInstance::new(self.handle)
  }
}

pub struct MaterialInstance {
  pub(crate) handle: *mut filament::MaterialInstance,
}

impl MaterialInstance {
  pub(self) fn new(material_handle: *mut filament::Material) -> Self {
    Self {
      handle: unsafe { filament::Material_CreateInstance(material_handle) },
    }
  }
}
