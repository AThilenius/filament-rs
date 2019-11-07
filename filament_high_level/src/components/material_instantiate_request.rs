use legion::prelude::*;

pub struct MaterialInstantiateRequest {
    pub material_template: Entity,
}

impl MaterialInstantiateRequest {
    pub fn new(material_template: Entity) -> Self {
        MaterialInstantiateRequest { material_template }
    }
}
