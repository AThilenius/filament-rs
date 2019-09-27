use crate::{
    camera::Camera, engine::Engine, misc_types::RenderTarget, raw_bindings::*, scene::Scene,
};
use nalgebra::Matrix4;
use std::ffi::CString;

pub struct View {
    engine: Engine,
    pub(crate) handle: *mut filament::View,
}

impl Drop for View {
    fn drop(&mut self) {
        unsafe {
            filament::Engine_DestroyView(self.engine.handle(), self.handle);
        }
    }
}

impl View {
    pub(crate) fn new(engine: Engine) -> Self {
        Self {
            handle: unsafe { filament::Engine_CreateView(engine.handle()) },
            engine,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        unsafe {
            let c_str = CString::new(name).unwrap();
            filament::View_SetName(self.handle, c_str.as_ptr());
        }
    }

    pub fn set_scene(&mut self, scene: &Scene) {
        unsafe {
            filament::View_SetScene(self.handle, scene.handle);
        }
    }

    pub fn set_camera(&mut self, camera: &Camera) {
        unsafe {
            filament::View_SetCamera(self.handle, camera.handle);
        }
    }

    pub fn set_viewport(&mut self, left: i32, bottom: i32, width: i32, height: i32) {
        unsafe {
            filament::View_SetViewport(self.handle, left, bottom, width, height);
        }
    }

    pub fn set_clear_color(&mut self, linear_r: f32, linear_g: f32, linear_b: f32, linear_a: f32) {
        unsafe {
            filament::View_SetClearColor(self.handle, linear_r, linear_g, linear_b, linear_a);
        }
    }

    pub fn get_clear_color(&self) -> Matrix4<f32> {
        let mut out = [0_f32; 16];
        unsafe {
            filament::View_GetClearColor(self.handle, out.as_mut_ptr());
        }
        Matrix4::from_column_slice(&out)
    }

    pub fn set_clear_targets(&mut self, color: bool, depth: bool, stencil: bool) {
        unsafe {
            filament::View_SetClearTargets(self.handle, color, depth, stencil);
        }
    }

    pub fn set_visible_layers(&mut self, select: i32, value: i32) {
        unsafe {
            filament::View_SetVisibleLayers(self.handle, select, value);
        }
    }

    pub fn set_shadows_enabled(&mut self, enabled: bool) {
        unsafe {
            filament::View_SetShadowsEnabled(self.handle, enabled);
        }
    }

    pub fn set_render_target(&mut self, render_target: RenderTarget) {
        unsafe {
            filament::View_SetRenderTarget(self.handle, render_target.handle);
        }
    }

    pub fn set_sample_count(&mut self, count: i32) {
        unsafe {
            filament::View_SetSampleCount(self.handle, count);
        }
    }

    pub fn get_sample_count(&self) -> i32 {
        unsafe { filament::View_GetSampleCount(self.handle) }
    }

    pub fn set_anti_aliasing(&mut self, aa_type: i32) {
        unsafe {
            filament::View_SetAntiAliasing(self.handle, aa_type);
        }
    }

    pub fn get_anti_aliasing(&self) -> i32 {
        unsafe { filament::View_GetAntiAliasing(self.handle) }
    }

    pub fn set_tone_mapping(&mut self, tm_type: i32) {
        unsafe {
            filament::View_SetToneMapping(self.handle, tm_type);
        }
    }

    pub fn get_tone_mapping(&self) -> i32 {
        unsafe { filament::View_GetToneMapping(self.handle) }
    }

    pub fn set_dithering(&mut self, dithering: i32) {
        unsafe {
            filament::View_SetDithering(self.handle, dithering);
        }
    }

    pub fn get_dithering(&self) -> i32 {
        unsafe { filament::View_GetDithering(self.handle) }
    }

    pub fn set_dynamic_resolution_options(
        &mut self,
        enabled: bool,
        homogeneous_scaling: bool,
        target_frame_time_milli: f32,
        head_room_ratio: f32,
        scale_rate: f32,
        min_scale: f32,
        max_scale: f32,
        history: i32,
    ) {
        unsafe {
            filament::View_SetDynamicResolutionOptions(
                self.handle,
                enabled,
                homogeneous_scaling,
                target_frame_time_milli,
                head_room_ratio,
                scale_rate,
                min_scale,
                max_scale,
                history,
            );
        }
    }

    pub fn set_render_quality(&mut self, hdr_color_buffer_quality: i32) {
        unsafe {
            filament::View_SetRenderQuality(self.handle, hdr_color_buffer_quality);
        }
    }

    pub fn set_dynamic_lighting_options(&mut self, z_light_near: f32, z_light_far: f32) {
        unsafe {
            filament::View_SetDynamicLightingOptions(self.handle, z_light_near, z_light_far);
        }
    }

    pub fn set_depth_prepass(&mut self, value: i32) {
        unsafe {
            filament::View_SetDepthPrepass(self.handle, value);
        }
    }

    pub fn set_post_processing_enabled(&mut self, enabled: bool) {
        unsafe {
            filament::View_SetPostProcessingEnabled(self.handle, enabled);
        }
    }

    pub fn is_post_processing_enabled(&self) -> bool {
        unsafe { filament::View_IsPostProcessingEnabled(self.handle) }
    }

    pub fn set_front_face_winding_inverted(&mut self, inverted: bool) {
        unsafe {
            filament::View_SetFrontFaceWindingInverted(self.handle, inverted);
        }
    }

    pub fn is_front_face_winding_inverted(&self) -> bool {
        unsafe { filament::View_IsFrontFaceWindingInverted(self.handle) }
    }

    pub fn set_ambient_occlusion(&mut self, ordinal: i32) {
        unsafe {
            filament::View_SetAmbientOcclusion(self.handle, ordinal);
        }
    }

    pub fn get_ambient_occlusion(&self) -> i32 {
        unsafe { filament::View_GetAmbientOcclusion(self.handle) }
    }

    pub fn set_ambient_occlusion_options(
        &mut self,
        radius: f32,
        bias: f32,
        power: f32,
        resolution: f32,
    ) {
        unsafe {
            filament::View_SetAmbientOcclusionOptions(self.handle, radius, bias, power, resolution);
        }
    }
}
