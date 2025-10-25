# SDL2 Window main

CXXC = g++

SRCP = src/

SRC = $(SRCP)main.cpp\
$(SRCP)SDL2_Event.cpp\
$(SRCP)SDL2_InitWin.cpp

SDL2I = -IThirdParty/SDL2/include/SDL2
SDL2L = -LThirdParty/SDL2/lib
SDL2F = -lmingw32 -lSDL2main -lSDL2 -lSDL2_image

SDL2WIN:
	$(CXXC) $(SRC) -Iinclude/SDL2 $(SDL2I) $(SDL2L) $(SDL2F) -o bin/win
