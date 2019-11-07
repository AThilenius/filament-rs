use legion::prelude::*;

pub struct Material {
    pub instance_handle: u32,
    pub material_template: Entity,
}

impl Material {
    pub fn new(instance_handle: u32, material_template: Entity) -> Self {
        Material {
            instance_handle,
            material_template,
        }
    }
}
