#include <backend/PixelBufferDescriptor.h>
#include <filament/Engine.h>
#include <filament/Renderer.h>
#include <filament/Viewport.h>

using namespace filament;
using namespace backend;

extern "C" bool Renderer_BeginFrame(Renderer* renderer, SwapChain* swapChain) {
  return (bool)renderer->beginFrame(swapChain);
}

extern "C" void Renderer_EndFrame(Renderer* renderer) { renderer->endFrame(); }

extern "C" void Renderer_Render(Renderer* renderer, View* view) {
  renderer->render(view);
}

extern "C" void Renderer_CopyFrame(Renderer* renderer, SwapChain* dstSwapChain,
                                   int dstLeft, int dstBottom, int dstWidth,
                                   int dstHeight, int srcLeft, int srcBottom,
                                   int srcWidth, int srcHeight, int flags) {
  const filament::Viewport dstViewport{dstLeft, dstBottom, (uint32_t)dstWidth,
                                       (uint32_t)dstHeight};
  const filament::Viewport srcViewport{srcLeft, srcBottom, (uint32_t)srcWidth,
                                       (uint32_t)srcHeight};
  renderer->copyFrame(dstSwapChain, dstViewport, srcViewport, (uint32_t)flags);
}

extern "C" double Renderer_GetUserTime(Renderer* renderer) {
  return renderer->getUserTime();
}

extern "C" void Renderer_ResetUserTime(Renderer* renderer) {
  renderer->resetUserTime();
}
