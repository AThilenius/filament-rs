#ifndef ENTITY_MANAGER
#define ENTITY_MANAGER

#include "opaque_types.h"

Entity EntityManager_create();

void EntityManager_destroy(Entity entity);

bool EntityManager_isAlive(Entity entity);

#endif
