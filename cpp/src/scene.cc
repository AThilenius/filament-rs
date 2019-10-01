#include <filament/Scene.h>

using namespace filament;
using namespace utils;

extern "C" void Scene_SetSkybox(Scene* scene, Skybox* skybox) {
  scene->setSkybox(skybox);
}

extern "C" void Scene_SetIndirectLight(Scene* scene,
                                       IndirectLight* indirectLight) {

  scene->setIndirectLight(indirectLight);
}

extern "C" void Scene_AddEntity(Scene* scene, Entity entity) {

  scene->addEntity(entity);
}

extern "C" void Scene_AddEntities(Scene* scene, Entity* entities,
                                  uint64_t entities_len) {
  scene->addEntities(entities, entities_len);
}

extern "C" void Scene_RemoveEntity(Scene* scene, Entity entity) {
  scene->remove(entity);
}

extern "C" int Scene_GetRenderableCount(Scene* scene) {
  return (int)scene->getRenderableCount();
}

extern "C" int Scene_GetLightCount(Scene* scene) {
  return (int)scene->getLightCount();
}
