pub struct MaterialTemplate {
    pub material_template_handle: u32,
}

impl MaterialTemplate {
    pub fn new(material_template_handle: u32) -> Self {
        Self {
            material_template_handle,
        }
    }
}
