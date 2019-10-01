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

void RenderableManager_Builder_SetLayerMask(RenderableManagerBuilder* builder,
                                            uint8_t select, uint8_t value);

void RenderableManager_Builder_SetPriority(RenderableManagerBuilder* builder,
                                           uint8_t priority);

void RenderableManager_Builder_SetCastShadowns(
    RenderableManagerBuilder* builder, bool cast_shadows);

void RenderableManager_Builder_SetReceiveShadows(
    RenderableManagerBuilder* builder, bool receive_shadows);

#endif
