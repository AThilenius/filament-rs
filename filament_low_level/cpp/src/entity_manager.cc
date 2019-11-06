#include <utils/Entity.h>
#include <utils/EntityManager.h>

#include <iostream>

using namespace utils;

extern "C" EntityManager* EntityManager_get() {
  // TODO: Unsure if this is always safe (converting a ref to a pointer).
  return &EntityManager::get();
}

extern "C" uint32_t EntityManager_create(EntityManager* entity_manager) {
  return entity_manager->create().getId();
}

extern "C" void EntityManager_destroy(EntityManager* entity_manager,
                                      Entity entity) {
  entity_manager->destroy(entity);
}

extern "C" void EntityManager_create_n(EntityManager* entity_manager,
                                       uint32_t n, uint32_t* entities_out) {
  // Safe only as long as Entity is internally a u32.
  Entity* entities_recast = (Entity*)entities_out;
  entity_manager->create(n, entities_recast);
}

extern "C" void EntityManager_destroy_n(EntityManager* entity_manager,
                                        uint32_t n, Entity* entities) {
  entity_manager->destroy(n, entities);
}

extern "C" bool EntityManager_isAlive(EntityManager* entity_manager,
                                      Entity entity) {
  return entity_manager->isAlive(entity);
}
