#include <filament/Engine.h>
#include <iostream>

using namespace filament;
using namespace utils;

extern "C" Engine* Engine_CreateEngine() { return Engine::create(); }

extern "C" void Engine_DestroyEngine(Engine* engine) {
  Engine::destroy(&engine);
}

// SwapChain (use 0ULL for default flags)
extern "C" SwapChain* Engine_CreateSwapChain(Engine* engine, void* surface,
                                             int flags) {
  return engine->createSwapChain(surface, (uint64_t)flags);
}

extern "C" void Engine_DestroySwapChain(Engine* engine, SwapChain* swapChain) {
  engine->destroy(swapChain);
}

// View
extern "C" View* Engine_CreateView(Engine* engine) {
  return engine->createView();
}

extern "C" void Engine_DestroyView(Engine* engine, View* view) {
  engine->destroy(view);
}

// Renderer

extern "C" Renderer* Engine_CreateRenderer(Engine* engine) {
  return engine->createRenderer();
}

extern "C" void Engine_DestroyRenderer(Engine* engine, Renderer* renderer) {
  engine->destroy(renderer);
}

// Camera

extern "C" Camera* Engine_CreateCamera(Engine* engine) {
  return engine->createCamera();
}

extern "C" Camera* Engine_CreateCameraWithEntity(Engine* engine,
                                                 Entity entity) {
  return engine->createCamera(entity);
}

extern "C" void Engine_DestroyCamera(Engine* engine, Camera* camera) {
  engine->destroy(camera);
}

// Scene

extern "C" Scene* Engine_CreateScene(Engine* engine) {
  return engine->createScene();
}

extern "C" void Engine_DestroyScene(Engine* engine, Scene* scene) {
  engine->destroy(scene);
}

// Fence

extern "C" Fence* Engine_CreateFence(Engine* engine) {
  return engine->createFence();
}

extern "C" void Engine_DestroyFence(Engine* engine, Fence* fence) {
  engine->destroy(fence);
}

// Others...

extern "C" void Engine_DestroyIndexBuffer(Engine* engine,
                                          IndexBuffer* indexBuffer) {
  engine->destroy(indexBuffer);
}

extern "C" void Engine_DestroyVertexBuffer(Engine* engine,
                                           VertexBuffer* vertexBuffer) {
  engine->destroy(vertexBuffer);
}

extern "C" void Engine_DestroyIndirectLight(Engine* engine,
                                            IndirectLight* indirectLight) {
  engine->destroy(indirectLight);
}

extern "C" void Engine_DestroyMaterial(Engine* engine, Material* material) {
  engine->destroy(material);
}

extern "C" void
Engine_DestroyMaterialInstance(Engine* engine,
                               MaterialInstance* materialInstance) {
  engine->destroy(materialInstance);
}

extern "C" void Engine_DestroySkybox(Engine* engine, Skybox* skybox) {
  engine->destroy(skybox);
}

extern "C" void Engine_DestroyTexture(Engine* engine, Texture* texture) {
  engine->destroy(texture);
}

extern "C" void Engine_DestroyRenderTarget(Engine* engine,
                                           RenderTarget* target) {
  engine->destroy(target);
}

extern "C" void Engine_DestroyEntity(Engine* engine, Entity entity) {
  engine->destroy(entity);
}

// Managers...
extern "C" TransformManager* Engine_GetTransformManager(Engine* engine) {
  return &engine->getTransformManager();
}

extern "C" LightManager* Engine_GetLightManager(Engine* engine) {
  return &engine->getLightManager();
}

extern "C" RenderableManager* Engine_GetRenderableManager(Engine* engine) {
  return &engine->getRenderableManager();
}
