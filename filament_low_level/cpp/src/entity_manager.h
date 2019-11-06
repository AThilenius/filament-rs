#ifndef ENTITY_MANAGER
#define ENTITY_MANAGER

#include "opaque_types.h"

EntityManager* EntityManager_get();

uint32_t EntityManager_create(EntityManager* entity_manager);

void EntityManager_destroy(EntityManager* entity_manager, Entity entity);

void EntityManager_create_n(EntityManager* entity_manager, uint32_t n,
                            Entity* entities);

void EntityManager_destroy_n(EntityManager* entity_manager, uint32_t n,
                             Entity* entities);

bool EntityManager_isAlive(EntityManager* entity_manager, Entity entity);

#endif
