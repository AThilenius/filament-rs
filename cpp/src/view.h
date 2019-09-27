#ifndef VIEW
#define VIEW

#include "opaque_types.h"

void View_SetName(View* view, const char* name);

void View_SetScene(View* view, Scene* scene);

void View_SetCamera(View* view, Camera* camera);

void View_SetViewport(View* view, int32_t left, int32_t bottom, int32_t width,
                      int32_t height);

void View_SetClearColor(View* view, float linearR, float linearG, float linearB,
                        float linearA);

void View_GetClearColor(View* view, float* out);

void View_SetClearTargets(View* view, bool_t color, bool_t depth,
                          bool_t stencil);

void View_SetVisibleLayers(View* view, int32_t select, int32_t value);

void View_SetShadowsEnabled(View* view, bool_t enabled);

void View_SetRenderTarget(View* view, RenderTarget* renderTarget);

void View_SetSampleCount(View* view, int32_t count);

int32_t View_GetSampleCount(View* view);

void View_SetAntiAliasing(View* view, int32_t type);

int32_t View_GetAntiAliasing(View* view);

void View_SetToneMapping(View* view, int32_t type);

int32_t View_GetToneMapping(View* view);

void View_SetDithering(View* view, int32_t dithering);

int32_t View_GetDithering(View* view);

void View_SetDynamicResolutionOptions(View* view, bool_t enabled,
                                      bool_t homogeneousScaling,
                                      float targetFrameTimeMilli,
                                      float headRoomRatio, float scaleRate,
                                      float minScale, float maxScale,
                                      int32_t history);

void View_SetRenderQuality(View* view, int32_t hdrColorBufferQuality);

void View_SetDynamicLightingOptions(View* view, float zLightNear,
                                    float zLightFar);

void View_SetDepthPrepass(View* view, int32_t value);

void View_SetPostProcessingEnabled(View* view, bool_t enabled);

bool_t View_IsPostProcessingEnabled(View* view);

void View_SetFrontFaceWindingInverted(View* view, bool_t inverted);

bool_t View_IsFrontFaceWindingInverted(View* view);

void View_SetAmbientOcclusion(View* view, int32_t ordinal);

int32_t View_GetAmbientOcclusion(View* view);

void View_SetAmbientOcclusionOptions(View* view, float radius, float bias,
                                     float power, float resolution);

#endif
