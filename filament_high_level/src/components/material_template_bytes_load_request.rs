pub struct MaterialTemplateBytesLoadRequest {
    pub(crate) data: Vec<u8>,
}

impl MaterialTemplateBytesLoadRequest {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}
