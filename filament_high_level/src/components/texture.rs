pub struct Texture {
    pub texture_handle: u32,
}

impl Texture {
    pub fn new(texture_handle: u32) -> Self {
        Self { texture_handle }
    }
}
