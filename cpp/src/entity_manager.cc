#include <utils/Entity.h>
#include <utils/EntityManager.h>

#include <iostream>

using namespace utils;

extern "C" uint32_t EntityManager_create() {
  return EntityManager::get().create().getId();
}

extern "C" void EntityManager_destroy(Entity entity) {
  EntityManager::get().destroy(entity);
}

extern "C" bool EntityManager_isAlive(Entity entity) {
  return EntityManager::get().isAlive(entity);
}
