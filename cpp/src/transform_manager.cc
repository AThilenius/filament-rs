#include <filament/Engine.h>
#include <filament/TransformManager.h>

using namespace filament;
using namespace utils;

extern "C" void TransformManager_SetTransform(Engine* engine, Entity entity,
                                              float* matrix4) {
  math::mat4f* matrix_recast = (math::mat4f*)matrix4;
  TransformManager& tm = engine->getTransformManager();
  tm.setTransform(tm.getInstance(entity), *matrix_recast);
}
