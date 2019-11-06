#ifndef CAMERA
#define CAMERA

#include "opaque_types.h"

void Camera_SetProjection(Camera* camera, int32_t projection, double left,
                          double right, double bottom, double top, double near,
                          double far);

void Camera_SetProjectionFov(Camera* camera, double fovInDegrees, double aspect,
                             double near, double far, int32_t fov);

void Camera_SetLensProjection(Camera* camera, double focalLength, double near,
                              double far);

void Camera_SetCustomProjection(Camera* camera, double* inMatrix, double near,
                                double far);

void Camera_LookAt(Camera* camera, double eye_x, double eye_y, double eye_z,
                   double center_x, double center_y, double center_z,
                   double up_x, double up_y, double up_z);

float Camera_GetNear(Camera* camera);

float Camera_GetCullingFar(Camera* camera);

void Camera_SetModelMatrix(Camera* camera, float* in);

void Camera_GetProjectionMatrix(Camera* camera, double* out);

void Camera_GetModelMatrix(Camera* camera, float* out);

void Camera_GetViewMatrix(Camera* camera, float* out);

void Camera_GetPosition(Camera* camera, float* out);

void Camera_GetLeftVector(Camera* camera, float* out);

void Camera_GetUpVector(Camera* camera, float* out);

void Camera_GetForwardVector(Camera* camera, float* out);

void Camera_SetExposure(Camera* camera, float aperture, float shutterSpeed,
                        float sensitivity);

float Camera_GetAperture(Camera* camera);

float Camera_GetShutterSpeed(Camera* camera);

float Camera_GetSensitivity(Camera* camera);

#endif
