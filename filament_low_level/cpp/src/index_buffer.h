#ifndef INDEX_BUFFER
#define INDEX_BUFFER

#include "opaque_types.h"

IndexBufferBuilder* IndexBuffer_CreateBuilder();

void IndexBuffer_DestroyBuilder(IndexBufferBuilder* builder);

void IndexBuffer_BuilderIndexCount(IndexBufferBuilder* builder,
                                   uint32_t indexCount);

void IndexBuffer_BuilderBufferType(IndexBufferBuilder* builder,
                                   int32_t indexType);

IndexBuffer* IndexBuffer_BuilderBuild(IndexBufferBuilder* builder,
                                      Engine* engine);

uint64_t IndexBuffer_GetIndexCount(IndexBuffer* indexBuffer);

void IndexBuffer_SetBuffer(IndexBuffer* indexBuffer, Engine* engine,
                           void* buffer, uint64_t size,
                           void (*callback)(void* buffer, uint64_t size,
                                            void* user));

#endif
