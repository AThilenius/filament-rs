#ifndef MATERIAL
#define MATERIAL

#include "opaque_types.h"

Material* Material_Create(Engine* engine, void* data, uint64_t len);

MaterialInstance* Material_CreateInstance(Material* material);

#endif
