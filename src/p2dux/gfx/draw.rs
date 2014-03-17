// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use collections::hashmap::HashMap;
use std::option::{Some, None};
use uuid::Uuid;
use serialize::{Encoder, Decoder, Encodable, Decodable};

use super::GameDisplay;
use super::texture::TextureSheets;
use p2d::world::{GlobalCoord, Payloadable, World, RelativeCoord};
use p2d::sprite::{SpriteTile, SpriteSheet};
use p2d::zone::{Zone, Tile};
use ux::SpriteFontSheet;

pub trait DrawableItem {
    fn get_sprites<'a>(&'a self) -> &'a [SpriteTile];

    fn draw(&self, display: &GameDisplay,
                          base: (int, int), offset: (int, int)) {
        let (base_x, base_y) = base;
        let (offset_x, offset_y) = offset;
        let sprites = self.get_sprites();
        let sheets = &display.sheets;
        for st in sprites.iter() {
            let sheet = sheets.get(&st.sheet);
            let (tile_size_x, tile_size_y) = st.size;
            // should only need to get screen_x once this whole thing..
            // .. this implies getting rid of offset..
            let screen_x = base_x + (offset_x * tile_size_x as int) as int;
            let screen_y = base_y + (offset_y * tile_size_y as int) as int;
            sheet.draw_tile(display.renderer, st,
                            (screen_x, screen_y), st.size);
        }
    }
}
