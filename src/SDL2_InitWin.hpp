#pragma once

#include "../ThirdParty/SDL2/include/SDL2/SDL.h"
#include "../ThirdParty/SDL2/include/SDL2/SDL_image.h"
#include <iostream>

extern SDL_Window* SDL2_Win;        /* Window */
extern SDL_Renderer* SDL2_Rnd;      /* Renderer */

/**
 * @brief Initialises the renderer, the window, and the SDL/IMG modes required
*/
int SDL2_InitWin(void);
