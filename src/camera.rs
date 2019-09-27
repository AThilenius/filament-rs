use crate::{engine::Engine, raw_bindings::*};
use nalgebra::{Matrix4, Vector3};

pub enum Projection {
  Perspective = 0,
  Ortho = 1,
}

pub enum Fov {
  Vertical = 0,
  Horizontal = 1,
}

pub struct Camera {
  engine: Engine,
  pub(crate) handle: *mut filament::Camera,
}

impl Drop for Camera {
  fn drop(&mut self) {
    unsafe {
      filament::Engine_DestroyCamera(self.engine.handle(), self.handle);
    }
  }
}

impl Camera {
  pub(crate) fn new(engine: Engine) -> Self {
    Self {
      handle: unsafe { filament::Engine_CreateCamera(engine.handle()) },
      engine,
    }
  }

  pub fn set_projection(
    &mut self,
    projection: Projection,
    left: f64,
    right: f64,
    bottom: f64,
    top: f64,
    near: f64,
    far: f64,
  ) {
    unsafe {
      filament::Camera_SetProjection(
        self.handle,
        projection as i32,
        left,
        right,
        bottom,
        top,
        near,
        far,
      );
    }
  }

  pub fn set_projection_fov(
    &mut self,
    fov_in_degrees: f64,
    aspect: f64,
    near: f64,
    far: f64,
    fov: Fov,
  ) {
    unsafe {
      filament::Camera_SetProjectionFov(self.handle, fov_in_degrees, aspect, near, far, fov as i32);
    }
  }

  pub fn set_lens_projection(&mut self, focal_length: f64, near: f64, far: f64) {
    unsafe {
      filament::Camera_SetLensProjection(self.handle, focal_length, near, far);
    }
  }

  pub fn set_custom_projection(&mut self, in_matrix: Matrix4<f64>, near: f64, far: f64) {
    let slice = in_matrix.as_slice();
    unsafe {
      filament::Camera_SetCustomProjection(self.handle, slice.as_ptr() as *mut f64, near, far);
    }
  }

  pub fn look_at(&mut self, eye: Vector3<f64>, center: Vector3<f64>, up: Vector3<f64>) {
    unsafe {
      filament::Camera_LookAt(
        self.handle,
        eye.x,
        eye.y,
        eye.z,
        center.x,
        center.y,
        center.z,
        up.x,
        up.y,
        up.z,
      );
    }
  }

  pub fn get_near(&mut self) -> f32 {
    unsafe { filament::Camera_GetNear(self.handle) }
  }

  pub fn get_culling_far(&mut self) -> f32 {
    unsafe { filament::Camera_GetCullingFar(self.handle) }
  }

  pub fn set_model_matrix(&mut self, matrix: Matrix4<f32>) {
    let slice = matrix.as_slice();
    unsafe {
      filament::Camera_SetModelMatrix(self.handle, slice.as_ptr() as *mut f32);
    }
  }

  pub fn get_projection_matrix(&mut self) -> Matrix4<f64> {
    let mut out: [f64; 16] = [0_f64; 16];
    unsafe { filament::Camera_GetProjectionMatrix(self.handle, out.as_mut_ptr()) }
    Matrix4::from_column_slice(&out)
  }

  pub fn get_model_matrix(&mut self) -> Matrix4<f32> {
    let mut out: [f32; 16] = [0_f32; 16];
    unsafe { filament::Camera_GetModelMatrix(self.handle, out.as_mut_ptr()) }
    Matrix4::from_column_slice(&out)
  }

  pub fn get_view_matrix(&mut self) -> Matrix4<f32> {
    let mut out: [f32; 16] = [0_f32; 16];
    unsafe { filament::Camera_GetViewMatrix(self.handle, out.as_mut_ptr()) }
    Matrix4::from_column_slice(&out)
  }

  pub fn get_position(&mut self) -> Vector3<f32> {
    let mut out: [f32; 3] = [0_f32; 3];
    unsafe { filament::Camera_GetPosition(self.handle, out.as_mut_ptr()) }
    Vector3::new(out[0], out[1], out[2])
  }

  pub fn get_left_vector(&mut self) -> Vector3<f32> {
    let mut out: [f32; 3] = [0_f32; 3];
    unsafe { filament::Camera_GetPosition(self.handle, out.as_mut_ptr()) }
    Vector3::new(out[0], out[1], out[2])
  }

  pub fn get_up_vector(&mut self) -> Vector3<f32> {
    let mut out: [f32; 3] = [0_f32; 3];
    unsafe { filament::Camera_GetUpVector(self.handle, out.as_mut_ptr()) }
    Vector3::new(out[0], out[1], out[2])
  }

  pub fn get_forward_vector(&mut self) -> Vector3<f32> {
    let mut out: [f32; 3] = [0_f32; 3];
    unsafe { filament::Camera_GetForwardVector(self.handle, out.as_mut_ptr()) }
    Vector3::new(out[0], out[1], out[2])
  }

  pub fn set_exposure(&mut self, aperture: f32, shutter_speed: f32, sensitivity: f32) {
    unsafe {
      filament::Camera_SetExposure(self.handle, aperture, shutter_speed, sensitivity);
    }
  }

  pub fn get_aperture(&mut self) -> f32 {
    unsafe { filament::Camera_GetAperture(self.handle) }
  }

  pub fn get_shutter_speed(&mut self) -> f32 {
    unsafe { filament::Camera_GetShutterSpeed(self.handle) }
  }

  pub fn get_sensitivity(&mut self) -> f32 {
    unsafe { filament::Camera_GetSensitivity(self.handle) }
  }
}
