#![allow(non_camel_case_types)]
use crate::low_level::{engine::Engine, raw_bindings::*};

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerType {
    Sampler2d,
    SamplerCubemap,
    SamplerExternal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerFormat {
    Int,
    Uint,
    Float,
    Shadow,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelDataFormat {
    R,
    RInteger,
    RG,
    RgInteger,
    RGB,
    RgbInteger,
    RGBA,
    RgbaInteger,
    UNUSED,
    DepthComponent,
    DepthStencil,
    Alpha,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum PixelDataType {
    Ubyte,
    Byte,
    Ushort,
    Short,
    Uint,
    Int,
    Half,
    Float,
    Compressed,
    UINT_10F_11F_11F_REV,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u16)]
pub enum TextureFormat {
    // 8-bits per element
    R8,
    R8_SNORM,
    R8UI,
    R8I,
    STENCIL8,

    // 16-bits per element
    R16F,
    R16UI,
    R16I,
    RG8,
    RG8_SNORM,
    RG8UI,
    RG8I,
    RGB565,
    RGB9_E5, // 9995 is actually 32 bpp but it's here for historical reasons.
    RGB5_A1,
    RGBA4,
    DEPTH16,

    // 24-bits per element
    RGB8,
    SRGB8,
    RGB8_SNORM,
    RGB8UI,
    RGB8I,
    DEPTH24,

    // 32-bits per element
    R32F,
    R32UI,
    R32I,
    RG16F,
    RG16UI,
    RG16I,
    R11F_G11F_B10F,
    RGBA8,
    SRGB8_A8,
    RGBA8_SNORM,
    UNUSED, // used to be rgbm
    RGB10_A2,
    RGBA8UI,
    RGBA8I,
    DEPTH32F,
    DEPTH24_STENCIL8,
    DEPTH32F_STENCIL8,

    // 48-bits per element
    RGB16F,
    RGB16UI,
    RGB16I,

    // 64-bits per element
    RG32F,
    RG32UI,
    RG32I,
    RGBA16F,
    RGBA16UI,
    RGBA16I,

    // 96-bits per element
    RGB32F,
    RGB32UI,
    RGB32I,

    // 128-bits per element
    RGBA32F,
    RGBA32UI,
    RGBA32I,

    // compressed formats

    // Mandatory in GLES 3.0 and GL 4.3
    EAC_R11,
    EAC_R11_SIGNED,
    EAC_RG11,
    EAC_RG11_SIGNED,
    ETC2_RGB8,
    ETC2_SRGB8,
    ETC2_RGB8_A1,
    ETC2_SRGB8_A1,
    ETC2_EAC_RGBA8,
    ETC2_EAC_SRGBA8,

    // Available everywhere except Android/iOS
    DXT1_RGB,
    DXT1_RGBA,
    DXT3_RGBA,
    DXT5_RGBA,

    // ASTC formats are available with a GLES extension
    RGBA_ASTC_4x4,
    RGBA_ASTC_5x4,
    RGBA_ASTC_5x5,
    RGBA_ASTC_6x5,
    RGBA_ASTC_6x6,
    RGBA_ASTC_8x5,
    RGBA_ASTC_8x6,
    RGBA_ASTC_8x8,
    RGBA_ASTC_10x5,
    RGBA_ASTC_10x6,
    RGBA_ASTC_10x8,
    RGBA_ASTC_10x10,
    RGBA_ASTC_12x10,
    RGBA_ASTC_12x12,
    SRGB8_ALPHA8_ASTC_4x4,
    SRGB8_ALPHA8_ASTC_5x4,
    SRGB8_ALPHA8_ASTC_5x5,
    SRGB8_ALPHA8_ASTC_6x5,
    SRGB8_ALPHA8_ASTC_6x6,
    SRGB8_ALPHA8_ASTC_8x5,
    SRGB8_ALPHA8_ASTC_8x6,
    SRGB8_ALPHA8_ASTC_8x8,
    SRGB8_ALPHA8_ASTC_10x5,
    SRGB8_ALPHA8_ASTC_10x6,
    SRGB8_ALPHA8_ASTC_10x8,
    SRGB8_ALPHA8_ASTC_10x10,
    SRGB8_ALPHA8_ASTC_12x10,
    SRGB8_ALPHA8_ASTC_12x12,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum TextureUsage {
    ColorAttachment = 0x1,
    DepthAttachment = 0x2,
    StencilAttachment = 0x4,
    Uploadable = 0x8,
    Sampleable = 0x10,
    DefaultUsage = 0x8 | 0x10,
}

#[derive(Clone)]
pub struct Texture {
    engine: Engine,
    pub(crate) handle: *mut filament::Texture,
}

impl Texture {
    pub fn new_standard(engine: &Engine, width: u32, height: u32, format: TextureFormat) -> Self {
        Self::new(
            engine,
            width,
            height,
            1,
            1,
            SamplerType::Sampler2d,
            format,
            TextureUsage::DefaultUsage,
        )
    }

    pub fn new(
        engine: &Engine,
        width: u32,
        height: u32,
        depth: u32,
        levels: u8,
        sampler: SamplerType,
        format: TextureFormat,
        usage_flags: TextureUsage,
    ) -> Self {
        let handle = unsafe {
            filament::Texture_Create(
                engine.handle(),
                width,
                height,
                depth,
                levels,
                sampler as u32,
                format as u32,
                usage_flags as u32,
            )
        };
        Self {
            handle,
            engine: engine.clone(),
        }
    }

    pub fn get_width(&self, level: u64) -> u64 {
        unsafe { filament::Texture_GetWidth(self.handle, level) }
    }

    pub fn get_height(&self, level: u64) -> u64 {
        unsafe { filament::Texture_GetHeight(self.handle, level) }
    }

    pub fn get_depth(&self, level: u64) -> u64 {
        unsafe { filament::Texture_GetDepth(self.handle, level) }
    }

    pub fn get_levels(&self) -> u64 {
        unsafe { filament::Texture_GetLevels(self.handle) }
    }

    pub fn set_image_copy<T: Sized>(
        &mut self,
        data: &[T],
        width: u32,
        height: u32,
        data_type: PixelDataType,
        format: PixelDataFormat,
    ) {
        unsafe {
            filament::Texture_SetImageCopy(
                self.handle,
                self.engine.handle(),
                data.as_ptr() as *mut std::ffi::c_void,
                (std::mem::size_of::<T>() * data.len()) as u64,
                // Level
                0,
                // X Offset
                0,
                // Y Offset
                0,
                width,
                height,
                // Left
                0,
                // Bottom
                0,
                data_type as u32,
                // Alignment
                1,
                // Stride
                0,
                format as u32,
            );
        }
    }

    pub fn set_sub_image_copy<T: Sized>(
        &mut self,
        data: &[T],
        level: u32,
        x_offset: u32,
        y_offset: u32,
        width: u32,
        height: u32,
        left: u32,
        bottom: u32,
        data_type: PixelDataType,
        alignment: u32,
        stride: u32,
        format: PixelDataFormat,
    ) {
        unsafe {
            filament::Texture_SetImageCopy(
                self.handle,
                self.engine.handle(),
                data.as_ptr() as *mut std::ffi::c_void,
                (std::mem::size_of::<T>() * data.len()) as u64,
                level,
                x_offset,
                y_offset,
                width,
                height,
                left,
                bottom,
                data_type as u32,
                alignment,
                stride,
                format as u32,
            );
        }
    }

    pub fn generate_mip_maps(&self) {
        unsafe {
            filament::Texture_GenerateMipmaps(self.handle, self.engine.handle());
        }
    }
}
