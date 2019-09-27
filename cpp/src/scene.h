#ifndef SCENE
#define SCENE

#include "opaque_types.h"

void Scene_SetSkybox(Scene* scene, Skybox* skybox);

void Scene_SetIndirectLight(Scene* scene, IndirectLight* indirectLight);

void Scene_AddEntity(Scene* scene, int32_t entity);

void Scene_AddEntities(Scene* scene, Entity* entities, size_t entities_len);

void Scene_Remove(Scene* scene, int32_t entity);

int32_t Scene_GetRenderableCount(Scene* scene);

int32_t Scene_GetLightCount(Scene* scene);

#endif
