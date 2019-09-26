#include <filament/Engine.h>
#include <iostream>

using namespace filament;
using namespace utils;

extern "C" Engine* Engine_nCreateEngine() { return Engine::create(); }

extern "C" void Engine_nDestroyEngine(Engine* engine) {
  Engine::destroy(&engine);
}

// SwapChain (use 0ULL for default flags)
extern "C" SwapChain* Engine_nCreateSwapChain(Engine* engine, void* surface,
                                              int flags) {
  return engine->createSwapChain(surface, (uint64_t)flags);
}

extern "C" void Engine_nDestroySwapChain(Engine* engine, SwapChain* swapChain) {
  engine->destroy(swapChain);
}

// View
extern "C" View* Engine_nCreateView(Engine* engine) {
  return engine->createView();
}

extern "C" void Engine_nDestroyView(Engine* engine, View* view) {
  engine->destroy(view);
}

// Renderer

extern "C" Renderer* Engine_nCreateRenderer(Engine* engine) {
  return engine->createRenderer();
}

extern "C" void Engine_nDestroyRenderer(Engine* engine, Renderer* renderer) {
  engine->destroy(renderer);
}

// Camera

extern "C" Camera* Engine_nCreateCamera(Engine* engine) {
  return engine->createCamera();
}

extern "C" Camera* Engine_nCreateCameraWithEntity(Engine* engine,
                                                  Entity entity) {
  return engine->createCamera(entity);
}

extern "C" void Engine_nDestroyCamera(Engine* engine, Camera* camera) {
  engine->destroy(camera);
}

// Scene

extern "C" Scene* Engine_nCreateScene(Engine* engine) {
  return engine->createScene();
}

extern "C" void Engine_nDestroyScene(Engine* engine, Scene* scene) {
  engine->destroy(scene);
}

// Fence

extern "C" Fence* Engine_nCreateFence(Engine* engine) {
  return engine->createFence();
}

extern "C" void Engine_nDestroyFence(Engine* engine, Fence* fence) {
  engine->destroy(fence);
}

// Others...

extern "C" void Engine_nDestroyIndexBuffer(Engine* engine,
                                           IndexBuffer* indexBuffer) {
  engine->destroy(indexBuffer);
}

extern "C" void Engine_nDestroyVertexBuffer(Engine* engine,
                                            VertexBuffer* vertexBuffer) {
  engine->destroy(vertexBuffer);
}

extern "C" void Engine_nDestroyIndirectLight(Engine* engine,
                                             IndirectLight* indirectLight) {
  engine->destroy(indirectLight);
}

extern "C" void Engine_nDestroyMaterial(Engine* engine, Material* material) {
  engine->destroy(material);
}

extern "C" void
Engine_nDestroyMaterialInstance(Engine* engine,
                                MaterialInstance* materialInstance) {
  engine->destroy(materialInstance);
}

extern "C" void Engine_nDestroySkybox(Engine* engine, Skybox* skybox) {
  engine->destroy(skybox);
}

extern "C" void Engine_nDestroyTexture(Engine* engine, Texture* texture) {
  engine->destroy(texture);
}

extern "C" void Engine_nDestroyRenderTarget(Engine* engine,
                                            RenderTarget* target) {
  engine->destroy(target);
}

extern "C" void Engine_nDestroyEntity(Engine* engine, Entity entity) {
  engine->destroy(entity);
}

// Managers...
extern "C" TransformManager* Engine_nGetTransformManager(Engine* engine) {
  return &engine->getTransformManager();
}

extern "C" LightManager* Engine_nGetLightManager(Engine* engine) {
  return &engine->getLightManager();
}

extern "C" RenderableManager* Engine_nGetRenderableManager(Engine* engine) {
  return &engine->getRenderableManager();
}
