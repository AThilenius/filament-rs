#ifndef TRANSFORM_MANAGER
#define TRANSFORM_MANAGER

#include "opaque_types.h"

void TransformManager_SetTransform(Engine* engine, Entity entity,
                                   float* matrix4);

#endif
