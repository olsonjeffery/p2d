#!/bin/bash

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cd $SCRIPT_DIR
if [ ! -d "rust-sdl2" ]; then
    git clone git://github.com/olsonjeffery/rust-sdl2
fi
cd rust-sdl2
git fetch
git rebase origin/master
UNAMEVAL=`uname`
echo $UNAMEVAL
if [ $UNAMEVAL = "Darwin" ]; then
    SDL_MODE=framework make
else
    make
fi
cd ..
cd ..
cd .. # now in repo root

echo "cargo-lite: artifacts"
find -f ./src/rust-sdl2/rust-sdl2/build/lib/*rlib
