#include <filament/Camera.h>

using namespace filament;

extern "C" void Camera_SetProjection(Camera* camera, int projection,
                                     double left, double right, double bottom,
                                     double top, double near, double far) {

  camera->setProjection((Camera::Projection)projection, left, right, bottom,
                        top, near, far);
}

extern "C" void Camera_SetProjectionFov(Camera* camera, double fovInDegrees,
                                        double aspect, double near, double far,
                                        int fov) {

  camera->setProjection(fovInDegrees, aspect, near, far, (Camera::Fov)fov);
}

extern "C" void Camera_SetLensProjection(Camera* camera, double focalLength,
                                         double near, double far) {

  camera->setLensProjection(focalLength, near, far);
}

extern "C" void Camera_SetCustomProjection(Camera* camera, double* inMatrix,
                                           double near, double far) {

  camera->setCustomProjection(
      *reinterpret_cast<const filament::math::mat4*>(inMatrix), near, far);
}

extern "C" void Camera_LookAt(Camera* camera, double eye_x, double eye_y,
                              double eye_z, double center_x, double center_y,
                              double center_z, double up_x, double up_y,
                              double up_z) {
  camera->lookAt({eye_x, eye_y, eye_z}, {center_x, center_y, center_z},
                 {up_x, up_y, up_z});
}

extern "C" float Camera_GetNear(Camera* camera) { return camera->getNear(); }

extern "C" float Camera_GetCullingFar(Camera* camera) {
  return camera->getCullingFar();
}

extern "C" void Camera_SetModelMatrix(Camera* camera, float* in) {
  camera->setModelMatrix(*reinterpret_cast<const filament::math::mat4f*>(in));
}

extern "C" void Camera_GetProjectionMatrix(Camera* camera, double* out) {
  const filament::math::mat4& m = camera->getProjectionMatrix();
  std::copy_n(&m[0][0], 16, out);
}

extern "C" void Camera_GetModelMatrix(Camera* camera, float* out) {
  const filament::math::mat4f& m = camera->getModelMatrix();
  std::copy_n(&m[0][0], 16, out);
}

extern "C" void Camera_GetViewMatrix(Camera* camera, float* out) {
  const filament::math::mat4f& m = camera->getViewMatrix();
  std::copy_n(&m[0][0], 16, out);
}

extern "C" void Camera_GetPosition(Camera* camera, float* out) {
  reinterpret_cast<filament::math::float3&>(*out) = camera->getPosition();
}

extern "C" void Camera_GetLeftVector(Camera* camera, float* out) {
  reinterpret_cast<filament::math::float3&>(*out) = camera->getLeftVector();
}

extern "C" void Camera_GetUpVector(Camera* camera, float* out) {
  reinterpret_cast<filament::math::float3&>(*out) = camera->getUpVector();
}

extern "C" void Camera_GetForwardVector(Camera* camera, float* out) {
  reinterpret_cast<filament::math::float3&>(*out) = camera->getForwardVector();
}

extern "C" void Camera_SetExposure(Camera* camera, float aperture,
                                   float shutterSpeed, float sensitivity) {
  camera->setExposure(aperture, shutterSpeed, sensitivity);
}

extern "C" float Camera_GetAperture(Camera* camera) {
  return camera->getAperture();
}

extern "C" float Camera_GetShutterSpeed(Camera* camera) {
  return camera->getShutterSpeed();
}

extern "C" float Camera_GetSensitivity(Camera* camera) {
  return camera->getSensitivity();
}
