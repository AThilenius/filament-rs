use crate::{engine::Engine, misc_types::Entity, raw_bindings::*};

pub struct Scene {
  engine: Engine,
  pub(crate) handle: *mut filament::Scene,
}

impl Drop for Scene {
  fn drop(&mut self) {
    unsafe {
      filament::Engine_DestroyScene(self.engine.handle(), self.handle);
    }
  }
}

impl Scene {
  pub(crate) fn new(engine: Engine) -> Self {
    Self {
      handle: unsafe { filament::Engine_CreateScene(engine.handle()) },
      engine,
    }
  }

  pub fn add_entity(&mut self, entity: Entity) {
    unsafe {
      filament::Scene_AddEntity(self.handle, entity);
    }
  }

  pub fn remove_entity(&mut self, entity: Entity) {
    unsafe {
      filament::Scene_RemoveEntity(self.handle, entity);
    }
  }

  // void Scene_SetSkybox(Scene* scene, Skybox* skybox);

  // void Scene_SetIndirectLight(Scene* scene, IndirectLight* indirectLight);

  // void Scene_AddEntities(Scene* scene, Entity* entities, uint32_t entities_len);

  // int32_t Scene_GetRenderableCount(Scene* scene);

  // int32_t Scene_GetLightCount(Scene* scene);
}
