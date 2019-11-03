#ifndef TEXTURE
#define TEXTURE

#include "opaque_types.h"

uint32_t SamplerParams_Create(uint8_t filterMag, uint8_t filterMin,
                              uint8_t wrapS, uint8_t wrapT, uint8_t wrapR,
                              uint8_t anisotropyLog2, uint8_t compareMode,
                              bool depthStencil, uint8_t padding0,
                              uint8_t compareFunc, uint8_t padding1,
                              uint8_t padding2);

Texture* Texture_Create(Engine* engine, uint32_t width, uint32_t height,
                        uint32_t depth, uint8_t levels, uint32_t sampler,
                        uint32_t format, uint32_t usage_flags);

uint64_t Texture_GetWidth(Texture* texture, uint64_t level);

uint64_t Texture_GetHeight(Texture* texture, uint64_t level);

uint64_t Texture_GetDepth(Texture* texture, uint64_t level);

uint64_t Texture_GetLevels(Texture* texture);

uint32_t Texture_GetTarget(Texture* texture);

uint32_t Texture_GetInternalFormat(Texture* texture);

void Texture_SetImageCopy(Texture* texture, Engine* engine, void* buffer,
                          uint64_t sizeInBytes, uint32_t level,
                          uint32_t xoffset, uint32_t yoffset, uint32_t width,
                          uint32_t height, uint32_t left, uint32_t bottom,
                          uint32_t type, uint32_t alignment, uint32_t stride,
                          uint32_t format);

void Texture_GenerateMipmaps(Texture* texture, Engine* engine);

#endif
