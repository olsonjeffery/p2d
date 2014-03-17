# p2d - a tile-based 2D graphics library

A simplistic library for drawing sprites to a window, built atop `rust-sdl`.

As is, Jeff Olson is the sole author. The code in this repository is released under the terms of the [3-Clause, "New BSD License"](https://en.wikipedia.org/wiki/BSD_licenses#3-clause_license_.28.22Revised_BSD_License.22.2C_.22New_BSD_License.22.2C_or_.22Modified_BSD_License.22.29).

## Philosophy & Goals

The Author says: I created this repo to share the (generalizable) work that I've been doing with 2D graphics in Rust. With this in mind, I want to keep it focused on the task of providing a useful, simplified & opinionated set of APIs for doing UX in the context of a 2D, tile/grid/sprite-based application.

`p2d` also provides some data-structures and API around a portal-based Field-of-View discovery and drawing scheme, based on a modified version of [Mingo's Restrictive Precise Angle Shadowcasting (MRPAS)](http://roguebasin.roguelikedevelopment.org/index.php?title=Restrictive_Precise_Angle_Shadowcasting). An early demo of this system is available in a [December 20th, 2013 video on youtube](https://www.youtube.com/watch?v=6WPm2mOZuQI).

#### Current Capability

- Load and keep a `p2dux::gfx::TextureSheet`, whose input is defined by `p2d::sprite::SpriteSheet`, in memory from a `.bmp` file on the filesystem
- A `TextureSheet` can draw a portion of itself, as defined in `p2d::sprite::SpriteSheet`, to the screen
- Some utility API around interacting with the above items for the purpose of field-of-view discovery and drawing-to-the-screen
- A rough pattern about UX/input management that also forms the basis of the main loop
- data-structures representing variable-width sprite fonts and basic API for drawing a line of text, in a given `p2d::sprite::SpriteFontSheet` at a given screen location
- A `World<T>` data structure, consisting of a number of `Zone`s, `Tile`s, `Portal`s and `Payload`s
    - A `World<T>` is the top level type in the above-described graph. It keeps `HashMap`-based collections of `Zone`s (which in turn contain `Tile`s which have a `Payload`) and `Portal`s, which connect two `Tile`s in one-or-two `Zone`s together.
    - A `World<T>` impls `Encodable`/`Decodable` via deriving and can be used to represent a snapshot of world state (although there are lots of things you'll keep outside the `World<TPayload>`, which the `TPayload` can contain weak-refernces to.
    - Most of the functionality built atop the `World<TPayload>` data structure depend upon `TPayload` also implementing some traits (e.g. `DrawableItem`, `FovItem`, etc) as a way to allow users to take an à la carte to the library.

#### Future Work

- Data structures simpler than `World<T>` for representing non-fancy tile/grid structures like UX menus, etc and API for drawing them
- Embedding and loading `TextureSheet`s, etc, from the application bin
- Refine the render loop strategy
  - Currently each tile is drawn directly, via `RenderCopy()`, to the `sdl2::render::Renderer` that represents the screen
  - This should be replaced with having tiles drawn to an off-screen render target, which can then have shaders/post-processing applied, and then drawn to the window `Renderer`

## Workspace Structure

- `p2dux`: A crate, with dependencies on `rust-sdl2` and `p2d` that handles graphics interaction and input management. It provides different, specialized strategies for drawing based on non-SDL2-coupled data structures defined in `p2d`
- `p2d`: Provides a set of non-SDL2-coupled data structures for defining graphics constructs to be drawn to the screen in `p2dux`. It also contains some algorithms & utilities for harnessing those structures in novel ways (such as `p2d::fov`)
- A `rust-sdl2` repo is checked out and rebased via the `Makefile`/`build.sh` scripts kept in `/src/rust-sdl2`. The repo is pointing at [olsonjeffery/rust-sdl2](http://github.com/olsonjeffery/rust-sdl2), which lags [AngryLawyer/rust-sdl2](http://github.com/AngryLawyer/rust-sdl2) as needed

## Build

It is currently building w/ `rustc` 0.9 (57db916).

Since `p2dux` relies on `rust-sdl2`, you have to keep it's build-time configuration requirements in mind when building crates in the `p2d` repo.

Assuming a typical *nix-like shell and a working install of the Rust toolchain avaiable in your `PATH`:

~~~~
$ git clone git://github.com/olsonjeffery/p2d <repo_name>
$ cd <repo_name>
$ git submodule init && git submodule update
$ RUST_PATH=`pwd`/rust-sdl2 rustpkg install p2dux <+ whatever rustpkg config params needed to build rust-sdl2>
~~~~

Note that, in the last command, we make the `rust-sdl2` submodule repo available to `rustpkg`, so that it can locate `rust-sdl2`, which is a dependency of the `p2dux` crate. Also, if you have to pass any config params to `rustpkg` in order to build `rust-sdl2`, you'll do that there, as well (e.g. you must pass `--cfg mac_framework` on OSX if you've installed the official SDL2 dev framework package).
