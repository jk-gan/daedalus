#include <cassert>

#include "game.h"

int main(int argc, char* argv[]) {
    Game game(800, 600);
    game.init();
    game.run();

    return 0;
}
