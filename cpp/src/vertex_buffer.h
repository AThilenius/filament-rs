#ifndef VERTEX_BUFFER
#define VERTEX_BUFFER

#include <stddef.h>

#include "opaque_types.h"

VertexBufferBuilder* VertexBuffer_CreateBuilder();

void VertexBuffer_DestroyBuilder(VertexBufferBuilder* builder);

void VertexBuffer_BuilderVertexCount(VertexBufferBuilder* builder,
                                     uint32_t vertexCount);

void VertexBuffer_BuilderBufferCount(VertexBufferBuilder* builder,
                                     uint8_t bufferCount);

void VertexBuffer_BuilderAttribute(VertexBufferBuilder* builder,
                                   int32_t attribute, uint8_t bufferIndex,
                                   int32_t attributeType, uint32_t byteOffset,
                                   uint8_t byteStride);

void VertexBuffer_BuilderNormalized(VertexBufferBuilder* builder,
                                    int32_t attribute, bool normalized);

VertexBuffer* VertexBuffer_BuilderBuild(VertexBufferBuilder* builder,
                                        Engine* engine);

uint64_t VertexBuffer_GetVertexCount(VertexBuffer* vertexBuffer);

void VertexBuffer_SetBufferAt(VertexBuffer* vertexBuffer, Engine* engine,
                              uint8_t bufferIndex, void* buffer, uint64_t size,
                              void (*callback)(void* buffer, uint64_t size,
                                               void* user));

void VertexBuffer_SetBufferAtCopy(VertexBuffer* vertexBuffer, Engine* engine,
                                  uint8_t bufferIndex, void* buffer,
                                  uint64_t size);

#endif
