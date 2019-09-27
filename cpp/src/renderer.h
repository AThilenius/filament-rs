#ifndef RENDERER
#define RENDERER

#include "opaque_types.h"

bool Renderer_BeginFrame(Renderer* renderer, SwapChain* swapChain);
void Renderer_EndFrame(Renderer* renderer);
void Renderer_Render(Renderer* renderer, View* view);
void Renderer_CopyFrame(Renderer* renderer, SwapChain* dstSwapChain,
                        int32_t dstLeft, int32_t dstBottom, int32_t dstWidth,
                        int32_t dstHeight, int32_t srcLeft, int32_t srcBottom,
                        int32_t srcWidth, int32_t srcHeight, int32_t flags);
double Renderer_GetUserTime(Renderer* renderer);
void Renderer_ResetUserTime(Renderer* renderer);

#endif
