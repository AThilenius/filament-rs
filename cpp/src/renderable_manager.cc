#include <filament/RenderableManager.h>

using namespace filament;
using namespace utils;

extern "C" RenderableManager::Builder*
RenderableManager_Builder_Create(uint32_t count) {
  return new RenderableManager::Builder(count);
}

extern "C" void
RenderableManager_Builder_SetBoundingBox(RenderableManager::Builder* builder,
                                         float* bounding_box) {
  Box* box = (Box*)bounding_box;
  builder->boundingBox(*box);
}

extern "C" void
RenderableManager_Builder_SetCulling(RenderableManager::Builder* builder,
                                     bool culling) {
  builder->culling(culling);
}

extern "C" void
RenderableManager_Builder_SetMaterial(RenderableManager::Builder* builder,
                                      uint64_t index,
                                      MaterialInstance* material_instance) {
  builder->material(index, material_instance);
}

extern "C" void RenderableManager_Builder_SetGeometry(
    RenderableManager::Builder* builder, uint64_t index, int32_t primitive_type,
    VertexBuffer* vertex_buffer, IndexBuffer* index_buffer) {
  builder->geometry(index, (RenderableManager::PrimitiveType)primitive_type,
                    vertex_buffer, index_buffer);
}

extern "C" void
RenderableManager_Builder_Build(RenderableManager::Builder* builder,
                                Engine* engine, Entity renderable) {
  builder->build(*engine, renderable);
  delete builder;
}
