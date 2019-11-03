use crate::low_level::{misc_types::Entity, raw_bindings::*};

pub struct EntityManager {
    pub(crate) handle: *mut filament::EntityManager,
}

impl EntityManager {
    pub fn get() -> Self {
        Self {
            handle: unsafe { filament::EntityManager_get() },
        }
    }

    pub fn create(&mut self) -> Entity {
        unsafe { filament::EntityManager_create(self.handle) }
    }

    pub fn create_n(&mut self, count: usize) -> Vec<Entity> {
        let mut entities: Vec<Entity> = Vec::with_capacity(count);
        unsafe {
            filament::EntityManager_create_n(self.handle, count as u32, entities.as_mut_ptr());
        }
        entities
    }

    pub fn destroy(&mut self, entity: Entity) {
        unsafe {
            filament::EntityManager_destroy(self.handle, entity);
        }
    }

    pub fn destroy_n(&mut self, entities: &[Entity]) {
        unsafe {
            filament::EntityManager_destroy_n(
                self.handle,
                entities.len() as u32,
                entities.as_ptr() as *mut Entity,
            );
        }
    }

    pub fn is_alive(&mut self, entity: Entity) {
        unsafe {
            filament::EntityManager_isAlive(self.handle, entity);
        }
    }
}
