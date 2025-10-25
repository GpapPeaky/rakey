#include "SDL2_InitWin.hpp"

SDL_Window* SDL2_Win;
SDL_Renderer* SDL2_Rnd;

int SDL2_InitWin(void){
    if(SDL_Init(SDL_INIT_EVERYTHING) == -1){
        std::fprintf(stderr, "SDL Could Not Initialise, Error: %s\n",SDL_GetError());
        return -1;
    }

    if(IMG_Init(IMG_INIT_PNG) == 0){
        std::fprintf(stderr, "Failed To Initialise PNGs, Error: %s\n",SDL_GetError());
        return -1;
    }

    /* Initialising window and renderer */

    SDL2_Win = SDL_CreateWindow("SDL Window", SDL_WINDOWPOS_CENTERED, SDL_WINDOWPOS_CENTERED, 900, 800, SDL_WINDOW_SHOWN);
    if(SDL2_Win == NULL){
        std::fprintf(stderr, "Failed To Create Window: %s\n", SDL_GetError());
        return -1;
    }

    SDL2_Rnd = SDL_CreateRenderer(SDL2_Win, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
    if(SDL2_Rnd == NULL){
        std::fprintf(stderr, "Failed To Create Renderer: %s\n", SDL_GetError());
        return -1;
    }

    return 0;
}
