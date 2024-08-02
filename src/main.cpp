#include <cassert>

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

    Game game(1280, 720);
    game.init();
    game.run();

    return 0;
}
