#include <filament/View.h>

using namespace filament;

extern "C" void View_SetName(View* view, const char* name) {
  view->setName(name);
}

extern "C" void View_SetScene(View* view, Scene* scene) {
  view->setScene(scene);
}

extern "C" void View_SetCamera(View* view, Camera* camera) {
  view->setCamera(camera);
}

extern "C" void View_SetViewport(View* view, int left, int bottom, int width,
                                 int height) {
  view->setViewport({left, bottom, (uint32_t)width, (uint32_t)height});
}

extern "C" void View_SetClearColor(View* view, float linearR, float linearG,
                                   float linearB, float linearA) {
  view->setClearColor({linearR, linearG, linearB, linearA});
}

extern "C" void View_GetClearColor(View* view, float* out) {
  auto linearColor = view->getClearColor();
  out[0] = linearColor[0];
  out[1] = linearColor[1];
  out[2] = linearColor[2];
  out[3] = linearColor[3];
}

extern "C" void View_SetClearTargets(View* view, bool color, bool depth,
                                     bool stencil) {
  view->setClearTargets(color, depth, stencil);
}

extern "C" void View_SetVisibleLayers(View* view, int select, int value) {
  view->setVisibleLayers((uint8_t)select, (uint8_t)value);
}

extern "C" void View_SetShadowsEnabled(View* view, bool enabled) {
  view->setShadowsEnabled(enabled);
}

extern "C" void View_SetRenderTarget(View* view, RenderTarget* renderTarget) {
  view->setRenderTarget(renderTarget);
}

extern "C" void View_SetSampleCount(View* view, int count) {
  view->setSampleCount((uint8_t)count);
}

extern "C" int View_GetSampleCount(View* view) {
  return view->getSampleCount();
}

extern "C" void View_SetAntiAliasing(View* view, int type) {
  view->setAntiAliasing(View::AntiAliasing(type));
}

extern "C" int View_GetAntiAliasing(View* view) {
  return (int)view->getAntiAliasing();
}

extern "C" void View_SetToneMapping(View* view, int type) {
  view->setToneMapping(View::ToneMapping(type));
}

extern "C" int View_GetToneMapping(View* view) {
  return (int)view->getToneMapping();
}

extern "C" void View_SetDithering(View* view, int dithering) {
  view->setDithering((View::Dithering)dithering);
}

extern "C" int View_GetDithering(View* view) {
  return (int)view->getDithering();
}

extern "C" void View_SetDynamicResolutionOptions(
    View* view, bool enabled, bool homogeneousScaling,
    float targetFrameTimeMilli, float headRoomRatio, float scaleRate,
    float minScale, float maxScale, int history) {
  View::DynamicResolutionOptions options;
  options.enabled = enabled;
  options.homogeneousScaling = homogeneousScaling;
  options.targetFrameTimeMilli = targetFrameTimeMilli;
  options.headRoomRatio = headRoomRatio;
  options.scaleRate = scaleRate;
  options.minScale = filament::math::float2{minScale};
  options.maxScale = filament::math::float2{maxScale};
  options.history = (uint8_t)history;
  view->setDynamicResolutionOptions(options);
}

extern "C" void View_SetRenderQuality(View* view, int hdrColorBufferQuality) {
  View::RenderQuality renderQuality;
  renderQuality.hdrColorBuffer = View::QualityLevel(hdrColorBufferQuality);
  view->setRenderQuality(renderQuality);
}

extern "C" void View_SetDynamicLightingOptions(View* view, float zLightNear,
                                               float zLightFar) {
  view->setDynamicLightingOptions(zLightNear, zLightFar);
}

extern "C" void View_SetDepthPrepass(View* view, int value) {
  view->setDepthPrepass(View::DepthPrepass(value));
}

extern "C" void View_SetPostProcessingEnabled(View* view, bool enabled) {
  view->setPostProcessingEnabled(enabled);
}

extern "C" bool View_IsPostProcessingEnabled(View* view) {
  return static_cast<bool>(view->isPostProcessingEnabled());
}

extern "C" void View_SetFrontFaceWindingInverted(View* view, bool inverted) {
  view->setFrontFaceWindingInverted(inverted);
}

extern "C" bool View_IsFrontFaceWindingInverted(View* view) {
  return static_cast<bool>(view->isFrontFaceWindingInverted());
}

extern "C" void View_SetAmbientOcclusion(View* view, int ordinal) {
  view->setAmbientOcclusion((View::AmbientOcclusion)ordinal);
}

extern "C" int View_GetAmbientOcclusion(View* view) {
  return (int)view->getAmbientOcclusion();
}

extern "C" void View_SetAmbientOcclusionOptions(View* view, float radius,
                                                float bias, float power,
                                                float resolution) {
  View::AmbientOcclusionOptions options;
  options.radius = radius;
  options.bias = bias;
  options.power = power;
  options.resolution = resolution;
  view->setAmbientOcclusionOptions(options);
}
