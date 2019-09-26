#ifndef ENGINE
#define ENGINE

#include "opaque_types.h"

Engine* Engine_nCreateEngine();
void Engine_nDestroyEngine(Engine* engine);

// SwapChain (Use 0ULL for flags).
SwapChain* Engine_nCreateSwapChain(Engine* engine, void* surface, int flags);
void Engine_nDestroySwapChain(Engine* engine, SwapChain* swapChain);

// View
struct View* Engine_nCreateView(struct Engine* engine);
void Engine_nDestroyView(struct Engine* engine, struct View* view);

// Renderer
struct Renderer* Engine_nCreateRenderer(struct Engine* engine);
void Engine_nDestroyRenderer(struct Engine* engine, Renderer* renderer);

// Camera
struct Camera* Engine_nCreateCamera(struct Engine* engine);
struct Camera* Engine_nCreateCameraWithEntity(struct Engine* engine,
                                              struct Entity entity);
void Engine_nDestroyCamera(struct Engine* engine, struct Camera* camera);

// Scene
struct Scene* Engine_nCreateScene(struct Engine* engine);
void Engine_nDestroyScene(struct Engine* engine, Scene* scene);

// Fence
struct Fence* Engine_nCreateFence(struct Engine* engine);
void Engine_nDestroyFence(struct Engine* engine, Fence* fence);

// Others...
void Engine_nDestroyIndexBuffer(struct Engine* engine,
                                struct IndexBuffer* indexBuffer);
void Engine_nDestroyVertexBuffer(struct Engine* engine,
                                 struct VertexBuffer* vertexBuffer);
void Engine_nDestroyIndirectLight(struct Engine* engine,
                                  struct IndirectLight* indirectLight);
void Engine_nDestroyMaterial(struct Engine* engine, struct Material* material);
void Engine_nDestroyMaterialInstance(struct Engine* engine,
                                     struct MaterialInstance* materialInstance);
void Engine_nDestroySkybox(struct Engine* engine, struct Skybox* skybox);
void Engine_nDestroyTexture(struct Engine* engine, struct Texture* texture);
void Engine_nDestroyRenderTarget(struct Engine* engine,
                                 struct RenderTarget* target);
void Engine_nDestroyEntity(struct Engine* engine, struct Entity entity);

// Managers...
struct TransformManager* Engine_nGetTransformManager(struct Engine* engine);
struct LightManager* Engine_nGetLightManager(struct Engine* engine);
struct RenderableManager* Engine_nGetRenderableManager(struct Engine* engine);

#endif
