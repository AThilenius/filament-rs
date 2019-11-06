#include <filament/Engine.h>
#include <filament/Material.h>

using namespace filament;
using namespace utils;

extern "C" Material* Material_Create(Engine* engine, void* data, uint64_t len) {
  return Material::Builder().package(data, len).build(*engine);
}

extern "C" MaterialInstance* Material_CreateInstance(Material* material) {
  return material->createInstance();
}

extern "C" void MaterialInstance_SetParameterTexture(MaterialInstance* instance,
                                                     const char* name,
                                                     Texture* texture,
                                                     uint32_t sampler_u) {
  TextureSampler& sampler = reinterpret_cast<TextureSampler&>(sampler_u);
  instance->setParameter(name, texture, sampler);
}
