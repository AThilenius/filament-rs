pub use filament_low_level::texture::{
    PixelDataFormat, PixelDataType, SamplerType, TextureFormat, TextureUsage,
};

pub struct TextureBytesLoadRequest {
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub levels: u8,
    pub sampler_type: SamplerType,
    pub format: TextureFormat,
    pub usage_flags: TextureUsage,
    pub pixel_data_type: PixelDataType,
    pub pixel_data_format: PixelDataFormat,
    pub pixel_data: Option<Vec<u8>>,
}

impl TextureBytesLoadRequest {
    pub fn new<T>(
        width: u32,
        height: u32,
        depth: u32,
        levels: u8,
        sampler_type: SamplerType,
        format: TextureFormat,
        usage_flags: TextureUsage,
        pixel_data_type: PixelDataType,
        pixel_data_format: PixelDataFormat,
        pixel_data: Vec<T>,
    ) -> Self
    where
        T: Sized,
    {
        // Convert to a Vec<u8> and forget the old vector.
        let pixel_data_u8: Vec<u8> = unsafe {
            Vec::from_raw_parts(
                pixel_data.as_ptr() as *mut u8,
                pixel_data.len() * std::mem::size_of::<T>(),
                pixel_data.capacity() * std::mem::size_of::<T>(),
            )
        };
        std::mem::forget(pixel_data);

        Self {
            width,
            height,
            depth,
            levels,
            sampler_type,
            format,
            usage_flags,
            pixel_data_type,
            pixel_data_format,
            pixel_data: Some(pixel_data_u8),
        }
    }

    pub fn new_standard<T>(
        width: u32,
        height: u32,
        format: TextureFormat,
        pixel_data_type: PixelDataType,
        pixel_data_format: PixelDataFormat,
        pixel_data: Vec<T>,
    ) -> Self
    where
        T: Sized,
    {
        Self::new(
            width,
            height,
            1,
            1,
            SamplerType::Sampler2d,
            format,
            TextureUsage::DefaultUsage,
            pixel_data_type,
            pixel_data_format,
            pixel_data,
        )
    }
}
