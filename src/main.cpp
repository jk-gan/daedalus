#include <cassert>

#define NS_PRIVATE_IMPLEMENTATION
#define MTL_PRIVATE_IMPLEMENTATION
#define MTK_PRIVATE_IMPLEMENTATION
#define CA_PRIVATE_IMPLEMENTATION

#include <AppKit/AppKit.hpp>

#include "app_delegate.h"
#include "game.h"

int main(int argc, char* argv[]) {
    // NS::AutoreleasePool* pAutoreleasePool = NS::AutoreleasePool::alloc()->init();
    //
    // AppDelegate del;
    //
    // NS::Application* pSharedApplication = NS::Application::sharedApplication();
    // pSharedApplication->setDelegate(&del);
    // pSharedApplication->run();
    //
    // pAutoreleasePool->release();

    Game game(800, 600);
    game.init();
    game.run();

    return 0;
}
