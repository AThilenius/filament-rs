#include <utils/Entity.h>
#include <utils/EntityManager.h>

using namespace utils;

extern "C" Entity EntityManager_create() {
  return EntityManager::get().create();
}

extern "C" void EntityManager_destroy(Entity entity) {
  EntityManager::get().destroy(entity);
}

extern "C" bool EntityManager_isAlive(Entity entity) {
  EntityManager::get().isAlive(entity);
}
