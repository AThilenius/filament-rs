#ifndef ENGINE
#define ENGINE

#include "opaque_types.h"

Engine* Engine_CreateEngine();
void Engine_DestroyEngine(Engine* engine);

// SwapChain (Use 0ULL for flags).
SwapChain* Engine_CreateSwapChain(Engine* engine, void* surface, int32_t flags);
void Engine_DestroySwapChain(Engine* engine, SwapChain* swapChain);

// View
struct View* Engine_CreateView(struct Engine* engine);
void Engine_DestroyView(struct Engine* engine, struct View* view);

// Renderer
struct Renderer* Engine_CreateRenderer(struct Engine* engine);
void Engine_DestroyRenderer(struct Engine* engine, Renderer* renderer);

// Camera
struct Camera* Engine_CreateCamera(struct Engine* engine);
struct Camera* Engine_CreateCameraWithEntity(struct Engine* engine,
                                             struct Entity entity);
void Engine_DestroyCamera(struct Engine* engine, struct Camera* camera);

// Scene
struct Scene* Engine_CreateScene(struct Engine* engine);
void Engine_DestroyScene(struct Engine* engine, Scene* scene);

// Fence
struct Fence* Engine_CreateFence(struct Engine* engine);
void Engine_DestroyFence(struct Engine* engine, Fence* fence);

// Others...
void Engine_DestroyIndexBuffer(struct Engine* engine,
                               struct IndexBuffer* indexBuffer);
void Engine_DestroyVertexBuffer(struct Engine* engine,
                                struct VertexBuffer* vertexBuffer);
void Engine_DestroyIndirectLight(struct Engine* engine,
                                 struct IndirectLight* indirectLight);
void Engine_DestroyMaterial(struct Engine* engine, struct Material* material);
void Engine_DestroyMaterialInstance(struct Engine* engine,
                                    struct MaterialInstance* materialInstance);
void Engine_DestroySkybox(struct Engine* engine, struct Skybox* skybox);
void Engine_DestroyTexture(struct Engine* engine, struct Texture* texture);
void Engine_DestroyRenderTarget(struct Engine* engine,
                                struct RenderTarget* target);
void Engine_DestroyEntity(struct Engine* engine, struct Entity entity);

// Managers...
struct TransformManager* Engine_GetTransformManager(struct Engine* engine);
struct LightManager* Engine_GetLightManager(struct Engine* engine);
struct RenderableManager* Engine_GetRenderableManager(struct Engine* engine);

#endif
