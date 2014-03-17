.PHONY: sdl2

UNAME=$(shell uname)

ifeq ($(UNAME),Darwin)
  # If the user wasn't explicit, see if SDL2 library exists
  ifeq ($(strip $(SDL_MODE)),"")
    SDL_CHECK=$(shell pkg-config --exists sdl2)
    ifeq ($(SDL_CHECK),0)
      SDL_MODE = dylib
    else
      SDL_MODE = framework
    endif
  endif

  ifeq ($(SDL_MODE),framework)
    RUSTFLAGS+=--cfg mac_framework
  else
    RUSTFLAGS+=--cfg mac_dylib
  endif
else
endif

sdl2:
	./build.sh
