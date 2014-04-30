// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec::Vec;
use collections::hashmap::HashMap;
use sdl2::sdl;
use sdl2::render::Renderer;
use p2d::sprite::SpriteSheet;
use sdl2;

pub mod draw;
pub mod texture;

pub struct GameDisplay {
    pub renderer: ~Renderer,
    pub sheets: texture::TextureSheets
}

impl GameDisplay {
    pub fn new(title: &str, screen_size: (int, int, bool), ss: Vec<SpriteSheet>) -> GameDisplay {
        // first thing we do
        sdl::init(sdl2::InitVideo);

        let (width, height, fullscreen) = screen_size;
        let window = sdl2::video::Window::new(
            title, sdl2::video::PosCentered, sdl2::video::PosCentered,
            width, height, sdl2::video::OpenGL);
        let window = match window {
            Ok(window) => window,
            Err(err) => fail!(format!("failed to create window: {}", err))
        };
        if fullscreen {
            window.set_fullscreen(sdl2::video::FTTrue);
        }
        let renderer = Renderer::from_window(
            window, sdl2::render::DriverAuto, sdl2::render::Accelerated);
        let renderer = match renderer {
            Ok(renderer) => renderer,
            Err(err) => fail!(format!("failed to create renderer: {}", err))
        };
        let mut display = GameDisplay {
            renderer: renderer,
            sheets: HashMap::new()
        };
        // build TextureSheets
        for s in ss.iter() {
            display.sheets.find_or_insert(s.name.clone(), texture::TextureSheet::new(
                display.renderer,
                &s.path,
                s.name.clone()));
        }
        display
    }

    pub fn set_draw_color(&self, rgb: (u8, u8, u8)) {
        let (r, g, b) = rgb;
        match self.renderer.set_draw_color(sdl2::pixels::RGB(r, g, b)) {
            Ok(()) => {},
            Err(e) => fail!("set_draw_color(): failure: {}", e)
        }
    }
    pub fn set_draw_sdl2_color(&self, rgb: sdl2::pixels::Color) {
        match self.renderer.set_draw_color(rgb) {
            Ok(()) => {},
            Err(e) => fail!("set_draw_sdl2_color(): failure: {}", e)
        }
    }
}

impl Drop for GameDisplay {
    fn drop(&mut self) {
        // last thing we do
        sdl::quit();
    }
}
