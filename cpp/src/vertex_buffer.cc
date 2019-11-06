#include <stdlib.h>
#include <string.h>

#include <filament/VertexBuffer.h>

#include <math/vec3.h>
#include <math/vec4.h>

using namespace filament;
using namespace filament::math;
using namespace backend;

namespace {
void free_buffer(void* buffer, size_t _size, void* _user) { free(buffer); }
} // namespace

extern "C" VertexBuffer::Builder* VertexBuffer_CreateBuilder() {
  return new VertexBuffer::Builder();
}

extern "C" void VertexBuffer_DestroyBuilder(VertexBuffer::Builder* builder) {
  delete builder;
}

extern "C" void VertexBuffer_BuilderVertexCount(VertexBuffer::Builder* builder,
                                                uint32_t vertexCount) {
  builder->vertexCount(vertexCount);
}

extern "C" void VertexBuffer_BuilderBufferCount(VertexBuffer::Builder* builder,
                                                uint8_t bufferCount) {
  builder->bufferCount(bufferCount);
}

extern "C" void
VertexBuffer_BuilderAttribute(VertexBuffer::Builder* builder, int32_t attribute,
                              uint8_t bufferIndex, int32_t attributeType,
                              uint32_t byteOffset, uint8_t byteStride) {
  builder->attribute((VertexAttribute)attribute, bufferIndex,
                     (VertexBuffer::AttributeType)attributeType, byteOffset,
                     byteStride);
}

extern "C" void VertexBuffer_BuilderNormalized(VertexBuffer::Builder* builder,
                                               int32_t attribute,
                                               bool normalized) {
  builder->normalized((VertexAttribute)attribute, normalized);
}

extern "C" VertexBuffer*
VertexBuffer_BuilderBuild(VertexBuffer::Builder* builder, Engine* engine) {
  return builder->build(*engine);
}

extern "C" uint64_t VertexBuffer_GetVertexCount(VertexBuffer* vertexBuffer) {
  return vertexBuffer->getVertexCount();
}

extern "C" void VertexBuffer_SetBufferAt(
    VertexBuffer* vertexBuffer, Engine* engine, uint8_t bufferIndex,
    void* buffer, uint64_t size,
    void (*callback)(void* buffer, uint64_t size, void* user)) {
  BufferDescriptor desc(buffer, size, (void (*)(void*, size_t, void*))callback);
  vertexBuffer->setBufferAt(*engine, bufferIndex, std::move(desc));
}

extern "C" void VertexBuffer_SetBufferAtCopy(VertexBuffer* vertexBuffer,
                                             Engine* engine,
                                             uint8_t bufferIndex, void* buffer,
                                             uint64_t size) {
  void* buffer_duplicate = malloc(size);
  memcpy(buffer_duplicate, buffer, size);
  BufferDescriptor desc(buffer_duplicate, size, &free_buffer);
  vertexBuffer->setBufferAt(*engine, bufferIndex, std::move(desc));
}

// extern "C" void VertexBuffer_PopulateTangentQuaternions(
//     int32_t quatType, int32_t quatCount, jobject outBuffer, int32_t
//     outRemaining, int32_t outStride, jobject normals, int32_t
//     normalsRemaining, int32_t normalsStride, jobject tangents, int32_t
//     tangentsRemaining, int32_t tangentsStride) {

//   AutoBuffer outNioBuffer(env, outBuffer, outRemaining, true);
//   void* outData = outNioBuffer.getData();

//   AutoBuffer normalsNioBuffer(env, normals, normalsRemaining);
//   auto normalsData = (const float3*)normalsNioBuffer.getData();

//   if (tangents) {
//     AutoBuffer tangentsNioBuffer(env, tangents, tangentsRemaining);
//     auto tangentsData = (const float4*)tangentsNioBuffer.getData();
//     VertexBuffer::populateTangentQuaternions(
//         {.quatType = (VertexBuffer::QuatType)quatType,
//          .quatCount = (size_t)quatCount,
//          .outBuffer = outData,
//          .outStride = (size_t)outStride,
//          .normals = normalsData,
//          .normalsStride = (size_t)normalsStride,
//          .tangents = tangentsData,
//          .tangentsStride = (size_t)tangentsStride});
//     return;
//   }

//   VertexBuffer::populateTangentQuaternions(
//       {.quatType = (VertexBuffer::QuatType)quatType,
//        .quatCount = (size_t)quatCount,
//        .outBuffer = outData,
//        .outStride = (size_t)outStride,
//        .normals = normalsData,
//        .normalsStride = (size_t)normalsStride,
//        .tangents = nullptr,
//        .tangentsStride = 0});
// }
