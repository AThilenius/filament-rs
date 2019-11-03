use crate::raw_bindings::*;

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerWrapMode {
  ClampToEdge,
  Repeat,
  MirroredRepeat,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerMinFilter {
  // don't change the enums values
  Nearest = 0,
  Linear = 1,
  NearestMipmapNearest = 2,
  LinearMipmapNearest = 3,
  NearestMipmapLinear = 4,
  LinearMipmapLinear = 5,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerMagFilter {
  // don't change the enums values
  Nearest = 0,
  Linear = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerCompareMode {
  // don't change the enums values
  None = 0,
  CompareToTexture = 1,
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum SamplerCompareFunc {
  // don't change the enums values
  LE = 0,
  GE,
  L,
  G,
  E,
  NE,
  A,
  N,
}

pub struct TextureSampler(pub u32);

impl Default for TextureSampler {
  fn default() -> Self {
    unsafe {
      Self(filament::SamplerParams_Create(
        SamplerMagFilter::Nearest as u8,
        SamplerMinFilter::Nearest as u8,
        SamplerWrapMode::ClampToEdge as u8,
        SamplerWrapMode::ClampToEdge as u8,
        SamplerWrapMode::ClampToEdge as u8,
        4,
        SamplerCompareMode::None as u8,
        false,
        0,
        SamplerCompareFunc::LE as u8,
        0,
        0,
      ))
    }
  }
}

impl TextureSampler {
  pub fn new_mag_min(filter_mag: SamplerMagFilter, filter_min: SamplerMinFilter) -> Self {
    unsafe {
      Self(filament::SamplerParams_Create(
        filter_mag as u8,
        filter_min as u8,
        SamplerWrapMode::ClampToEdge as u8,
        SamplerWrapMode::ClampToEdge as u8,
        SamplerWrapMode::ClampToEdge as u8,
        4,
        SamplerCompareMode::None as u8,
        false,
        0,
        SamplerCompareFunc::LE as u8,
        0,
        0,
      ))
    }
  }

  pub fn new(
    filter_mag: SamplerMagFilter,
    filter_min: SamplerMinFilter,
    wrap_s: SamplerWrapMode,
    wrap_t: SamplerWrapMode,
    wrap_r: SamplerWrapMode,
    anisotropy_log_2: u8,
    compare_mode: SamplerCompareMode,
    depth_stencil: bool,
    compare_func: SamplerCompareFunc,
    padding_0: u8,
    padding_1: u8,
    padding_2: u8,
  ) -> Self {
    unsafe {
      Self(filament::SamplerParams_Create(
        filter_mag as u8,
        filter_min as u8,
        wrap_s as u8,
        wrap_t as u8,
        wrap_r as u8,
        anisotropy_log_2,
        compare_mode as u8,
        depth_stencil,
        padding_0,
        compare_func as u8,
        padding_1,
        padding_2,
      ))
    }
  }
}
