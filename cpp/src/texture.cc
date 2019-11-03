#include <algorithm>
#include <functional>

#include <backend/BufferDescriptor.h>
#include <filament/Engine.h>
#include <filament/Stream.h>
#include <filament/Texture.h>

using namespace filament;
using namespace backend;

namespace {
void free_buffer(void* buffer, size_t _size, void* _user) { free(buffer); }
} // namespace

extern "C" uint32_t SamplerParams_Create(uint8_t filterMag, uint8_t filterMin,
                                         uint8_t wrapS, uint8_t wrapT,
                                         uint8_t wrapR, uint8_t anisotropyLog2,
                                         uint8_t compareMode, bool depthStencil,
                                         uint8_t padding0, uint8_t compareFunc,
                                         uint8_t padding1, uint8_t padding2) {
  SamplerParams params;
  params.filterMag = (SamplerMagFilter)filterMag;
  params.filterMin = (SamplerMinFilter)filterMin;
  params.wrapS = (SamplerWrapMode)wrapS;
  params.wrapT = (SamplerWrapMode)wrapT;
  params.wrapR = (SamplerWrapMode)wrapR;
  params.anisotropyLog2 = anisotropyLog2;
  params.compareMode = (SamplerCompareMode)compareMode;
  params.depthStencil = depthStencil;
  params.padding0 = padding0;
  params.compareFunc = (SamplerCompareFunc)compareFunc;
  params.padding1 = padding1;
  params.padding2 = padding2;
  return params.u;
}

extern "C" Texture* Texture_Create(Engine* engine, uint32_t width,
                                   uint32_t height, uint32_t depth,
                                   uint8_t levels, uint32_t sampler,
                                   uint32_t format, uint32_t usage_flags) {
  Texture::Builder builder;
  builder.width(width);
  builder.height(height);
  builder.depth(depth);
  builder.levels(levels);
  builder.sampler((Texture::Sampler)sampler);
  builder.format((Texture::InternalFormat)format);
  builder.usage((Texture::Usage)usage_flags);

  return builder.build(*engine);
}

extern "C" uint64_t Texture_GetWidth(Texture* texture, uint64_t level) {

  return texture->getWidth(level);
}

extern "C" uint64_t Texture_GetHeight(Texture* texture, uint64_t level) {

  return texture->getHeight(level);
}

extern "C" uint64_t Texture_GetDepth(Texture* texture, uint64_t level) {

  return texture->getDepth(level);
}

extern "C" uint64_t Texture_GetLevels(Texture* texture) {

  return texture->getLevels();
}

extern "C" uint32_t Texture_GetTarget(Texture* texture) {

  return (uint32_t)texture->getTarget();
}

extern "C" uint32_t Texture_GetInternalFormat(Texture* texture) {

  return (uint32_t)texture->getFormat();
}

extern "C" void
Texture_SetImageCopy(Texture* texture, Engine* engine, void* buffer,
                     uint64_t sizeInBytes, uint32_t level, uint32_t xoffset,
                     uint32_t yoffset, uint32_t width, uint32_t height,
                     uint32_t left, uint32_t bottom, uint32_t type,
                     uint32_t alignment, uint32_t stride, uint32_t format) {
  void* buffer_duplicate = malloc(sizeInBytes);
  memcpy(buffer_duplicate, buffer, sizeInBytes);

  Texture::PixelBufferDescriptor desc(
      buffer_duplicate, sizeInBytes, (backend::PixelDataFormat)format,
      (backend::PixelDataType)type, (uint8_t)alignment, left, bottom, stride,
      &free_buffer, nullptr);

  texture->setImage(*engine, (size_t)level, xoffset, yoffset, width, height,
                    std::move(desc));
}

extern "C" void Texture_GenerateMipmaps(Texture* texture, Engine* engine) {
  texture->generateMipmaps(*engine);
}

// extern "C" int Texture_SetImageCubemap(
//     Texture* texture, Engine* engine, int level, jobject storage, int
//     remaining, int left, int bottom, int type, int alignment, int stride, int
//     format, jintArray faceOffsetsInBytes_, jobject handler, jobject runnable)
//     {

//   Engine* engine = (Engine*)nativeEngine;

//   int* faceOffsetsInBytes =
//       env->GetIntArrayElements(faceOffsetsInBytes_, nullptr);
//   Texture::FaceOffsets faceOffsets;
//   std::copy_n(faceOffsetsInBytes, 6, faceOffsets.offsets);
//   env->ReleaseIntArrayElements(faceOffsetsInBytes_, faceOffsetsInBytes,
//                                JNI_ABORT);

//   size_t sizeInBytes =
//       6 * getTextureDataSize(texture, (size_t)level, (Texture::Format)format,
//                              (Texture::Type)type, (size_t)stride,
//                              (size_t)alignment);

//   AutoBuffer nioBuffer(env, storage, 0);
//   if (sizeInBytes > (size_t(remaining) << nioBuffer.getShift())) {
//     // BufferOverflowException
//     return -1;
//   }

//   void* buffer = nioBuffer.getData();
//   auto* callback = JniBufferCallback::make(engine, env, handler, runnable,
//                                            std::move(nioBuffer));

//   Texture::PixelBufferDescriptor desc(
//       buffer, sizeInBytes, (backend::PixelDataFormat)format,
//       (backend::PixelDataType)type, (uint8_t)alignment, (uint32_t)left,
//       (uint32_t)bottom, (uint32_t)stride, &JniBufferCallback::invoke,
//       callback);

//   texture->setImage(*engine, (size_t)level, std::move(desc), faceOffsets);

//   return 0;
// }

// extern "C" void Texture_SetExternalImage(Texture* texture, Engine* engine,
//                                          long int eglImage) {

//   Engine* engine = (Engine*)nativeEngine;
//   texture->setExternalImage(*engine, (void*)eglImage);
// }

// extern "C" void Texture_SetExternalStream(Texture* texture, Engine* engine,
//                                           long int nativeStream) {

//   Engine* engine = (Engine*)nativeEngine;
//   Stream* stream = (Stream*)nativeStream;
//   texture->setExternalStream(*engine, stream);
// }

// extern "C" bool Texture_IsStreamValidForTexture(Texture* texture, long int) {

//   return (bool)(texture->getTarget() == SamplerType::SAMPLER_EXTERNAL);
// }

// extern "C" int Texture_GeneratePrefilterMipmap(
//     Texture* texture, Engine* engine, int width, int height, jobject storage,
//     int remaining, int left, int top, int type, int alignment, int stride,
//     int format, jintArray faceOffsetsInBytes_, jobject handler,
//     jobject runnable, int sampleCount, bool mirror) {

//   Engine* engine = (Engine*)nativeEngine;

//   int* faceOffsetsInBytes =
//       env->GetIntArrayElements(faceOffsetsInBytes_, nullptr);
//   Texture::FaceOffsets faceOffsets;
//   std::copy_n(faceOffsetsInBytes, 6, faceOffsets.offsets);
//   env->ReleaseIntArrayElements(faceOffsetsInBytes_, faceOffsetsInBytes,
//                                JNI_ABORT);

//   stride = stride ? stride : width;
//   size_t sizeInBytes =
//       6 * Texture::computeTextureDataSize((Texture::Format)format,
//                                           (Texture::Type)type,
//                                           (size_t)stride, (size_t)height,
//                                           (size_t)alignment);

//   AutoBuffer nioBuffer(env, storage, 0);
//   if (sizeInBytes > (size_t(remaining) << nioBuffer.getShift())) {
//     // BufferOverflowException
//     return -1;
//   }

//   void* buffer = nioBuffer.getData();
//   auto* callback = JniBufferCallback::make(engine, env, handler, runnable,
//                                            std::move(nioBuffer));

//   Texture::PixelBufferDescriptor desc(
//       buffer, sizeInBytes, (backend::PixelDataFormat)format,
//       (backend::PixelDataType)type, (uint8_t)alignment, (uint32_t)left,
//       (uint32_t)top, (uint32_t)stride, &JniBufferCallback::invoke, callback);

//   Texture::PrefilterOptions options;
//   options.sampleCount = sampleCount;
//   options.mirror = mirror;
//   texture->generatePrefilterMipmap(*engine, std::move(desc), faceOffsets,
//                                    &options);

//   return 0;
// }
