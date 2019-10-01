#ifndef RENDERABLE_MANAGER
#define RENDERABLE_MANAGER

#include "opaque_types.h"

RenderableManagerBuilder* RenderableManager_Builder_Create(uint32_t count);

void RenderableManager_Builder_SetBoundingBox(RenderableManagerBuilder* builder,
                                              float* bounding_box);

void RenderableManager_Builder_SetCulling(RenderableManagerBuilder* builder,
                                          bool culling);

void RenderableManager_Builder_SetMaterial(RenderableManagerBuilder* builder,
                                           uint64_t index,
                                           MaterialInstance* material_instance);

void RenderableManager_Builder_SetGeometry(RenderableManagerBuilder* builder,
                                           uint64_t index,
                                           int32_t primitive_type,
                                           VertexBuffer* vertex_buffer,
                                           IndexBuffer* index_buffer);

void RenderableManager_Builder_Build(RenderableManagerBuilder* builder,
                                     Engine* engine, Entity renderable);

#endif
