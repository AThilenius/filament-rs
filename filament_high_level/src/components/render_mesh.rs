use legion::prelude::*;
use std::collections::HashMap;

// Re-export texture sampler types (they are all enums and facades around a u32).
pub use filament_low_level::texture_sampler::*;

// This is redefined from the low-level version as it stores Entity references for Texture refs.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum MaterialParameterBind {
    Float(f32),
    Texture(Entity, TextureSampler),
}

pub struct RenderMesh {
    pub mesh: Entity,
    pub material: Entity,
    pub parameter_binds: HashMap<String, MaterialParameterBind>,
}
