#include <functional>
#include <stdlib.h>
#include <string.h>

#include <filament/IndexBuffer.h>

using namespace filament;
using namespace backend;

extern "C" IndexBuffer::Builder* IndexBuffer_CreateBuilder() {
  return new IndexBuffer::Builder();
}

extern "C" void IndexBuffer_DestroyBuilder(IndexBuffer::Builder* builder) {
  delete builder;
}

extern "C" void IndexBuffer_BuilderIndexCount(IndexBuffer::Builder* builder,
                                              uint32_t indexCount) {
  builder->indexCount(indexCount);
}

extern "C" void IndexBuffer_BuilderBufferType(IndexBuffer::Builder* builder,
                                              int32_t indexType) {
  using IndexType = IndexBuffer::IndexType;

  IndexType types[] = {IndexType::USHORT, IndexType::UINT};
  builder->bufferType(types[indexType & 1]);
}

extern "C" IndexBuffer* IndexBuffer_BuilderBuild(IndexBuffer::Builder* builder,
                                                 Engine* engine) {
  return builder->build(*engine);
}

extern "C" uint64_t IndexBuffer_GetIndexCount(IndexBuffer* indexBuffer) {
  return indexBuffer->getIndexCount();
}

extern "C" void IndexBuffer_SetBuffer(IndexBuffer* indexBuffer, Engine* engine,
                                      void* buffer, uint64_t size) {
  BufferDescriptor desc(buffer, size);
  indexBuffer->setBuffer(*engine, std::move(desc));
}
