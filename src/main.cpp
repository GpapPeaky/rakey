#include "auxf/includes.aux"

int main(int argc, char* argv[]){

    SDL2_InitWin(); /* Initialise */

    bool SDL2_Quit = false;

    while(!SDL2_Quit){
        SDL2_HandleEvents(SDL2_Quit); /* Creates a new event to poll per call (Might need to be optimised) */

        /* Updates to assets / sprites */

        SDL_SetRenderDrawColor(SDL2_Rnd, 0, 0, 255, 255);

        SDL_RenderClear(SDL2_Rnd);

        /* Render functions */

        SDL_RenderPresent(SDL2_Rnd);
    }

    SDL_DestroyRenderer(SDL2_Rnd);
    SDL_DestroyWindow(SDL2_Win);
    SDL_Quit();

    return 0;
}
