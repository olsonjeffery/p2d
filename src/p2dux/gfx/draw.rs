// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::hashmap::HashMap;
use std::option::{Some, None};
use uuid::Uuid;
use serialize::{Encoder, Decoder, Encodable, Decodable};

use super::GameDisplay;
use super::texture::TextureSheets;
use p2d::world::{GlobalCoord, Payloadable, World, RelativeCoord};
use p2d::sprite::{SpriteTile, SpriteSheet};
use p2d::zone::{Zone, Tile};
use ux::SpriteFontSheet;

pub type Payload_Processor<T> = fn(&World<T>, &GameDisplay, (int, int), (int, int), Uuid);

pub trait DrawableItem {
    fn get_sprites<'a>(&'a self) -> &'a [SpriteTile];

    fn draw(&self, world: &World<Self>, display: &GameDisplay,
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

// so this is assuming uniform tile size amongst a group of tiles in a
// Zone
pub fn draw_grid_tile<T:DrawableItem>(t: &Tile<T>, world: &World<T>,
                                         display: &GameDisplay, base_x: int,
                                         base_y: int, offset_x: int, offset_y: int) {
    let sheets = &display.sheets;
    // draw tile sprites
    for st in t.sprites.iter() {
        let sheet = sheets.get(&st.sheet);
        let (tile_size_x, tile_size_y) = st.size;
        // should only need to get screen_x once this whole thing..
        let screen_x = base_x + (offset_x * tile_size_x as int) as int;
        let screen_y = base_y + (offset_y * tile_size_y as int) as int;
        sheet.draw_tile(display.renderer, st,
                        (screen_x, screen_y), st.size);
    }
    // draw actor sprite(s) .. eventually the tile sprite drawing will
    // move into here as well..
    let base = (base_x, base_y);
    let offset = (offset_x, offset_y);
    t.payload.draw(world, display, base, offset);
}

pub fn draw_tiles_from<'a, T: Send + Payloadable + DrawableItem>(world: &World<T>, visible_tiles: &~[RelativeCoord], origin: (int, int), display: &GameDisplay) {
    // screen center
    let (base_x, base_y) = origin;
    for tc in visible_tiles.iter() {
        let zone = world.get_zone(&tc.zone_id);
        //println!("draw_agent_los: zid:{:} {:?}", tc.zone_id, (tc.lx, tc.ly));
        let tile = zone.get_tile((tc.lx, tc.ly));
        // need to account multiple zones/reference points w/ portalling..
        draw_grid_tile(tile, world, display, base_x, base_y, tc.gx, tc.gy);
    }
}
