// #include <filament/Camera.h>
// #include <filament/Color.h>
#include <filament/Engine.h>
// #include <filament/FilamentAPI.h>
// #include <filament/Renderer.h>
// #include <filament/Scene.h>
// #include <filament/View.h>
// #include <math/vec4.h>

using namespace filament;
// using namespace filament::math;

#include <iostream>

// extern "C" void init(void *window_handle)
// {
//   std::cout << "Hello, world! Got pointer: " << window_handle << std::endl;

//   Engine *engine = Engine::create();
//   SwapChain *swapChain = engine->createSwapChain(window_handle);
//   Renderer *renderer = engine->createRenderer();

//   Camera *camera = engine->createCamera();
//   View *view = engine->createView();
//   view->setClearTargets(true, true, false);
//   view->setRenderTarget(View::TargetBufferFlags::DEPTH_AND_STENCIL);
//   view->setShadowsEnabled(false);
//   view->setClearColor({0.0, 0.25, 0.5, 1.0});
//   Scene *scene = engine->createScene();

//   view->setCamera(camera);
//   view->setScene(scene);

//   if (renderer->beginFrame(swapChain))
//   {
//     renderer->render(view);
//     renderer->endFrame();
//   }

//   engine->destroy(camera);
// }

extern "C" void hello() {
  std::cout << "Hello, from Rust!" << std::endl;
  Engine* engine = Engine::create();
}
