// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::hashmap::HashMap;
use std::option::{Some, None};
use extra::uuid::Uuid;

use super::GameDisplay;
use super::texture::TextureSheets;
use p2d::world::{GlobalCoord, Payload, World, RelativeCoord, EntityData};
use p2d::sprite::SpriteSheet;
use p2d::zone::{Zone, Tile};

//pub type Payload_Processor<'a, T> = 'b |w: &'a World, display: &'a GameDisplay,
//    base: (int, int), plid: Uuid| -> bool;
pub type Payload_Processor<T> = fn(&World<T>, &GameDisplay, (int, int), (int, int), Uuid);

// so this is assuming uniform tile size amongst a group of tiles in a
// Zone
pub fn draw_grid_tile<T: Send + Payload>(t: &Tile, world: &World<T>,
                                         display: &GameDisplay, base_x: int,
                                         base_y: int, offset_x: int, offset_y: int,
                                         payload_cb: Payload_Processor<T>) {
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
    // draw object sprite
    // ...

    // draw actor sprite
    let base = (base_x, base_y);
    let offset = (offset_x, offset_y);
    match t.payload_id {
        None => {},
        Some(id) => {
            payload_cb(world, display, base, offset, id);
            /*
            let agent = world.get_agent(&id);
            // animation stuff goes here.. curr time/tick fed in as a
            // param.. ?
            let st = &agent.animations[0];
            let sheet = sheets.get(&st.sheet);
            let (tile_size_x, tile_size_y) = st.size;
            // should only need to get screen_x once this whole thing..
            let screen_x = base_x + (zone_x * tile_size_x as int) as int;
            let screen_y = base_y + (zone_y * tile_size_x as int) as int;
            sheet.draw_tile(display.renderer, st,
                            (screen_x, screen_y), st.size);
            */
        }
    }
}

pub fn draw_agent_los<'a, T: Send + Payload>(world: &World<T>, visible_tiles: &~[RelativeCoord], display: &GameDisplay, payload_cb: Payload_Processor<T>) {
    // screen center
    let (base_x, base_y) = (400, 300);
    for tc in visible_tiles.iter() {
        let zone = world.get_zone(tc.zone_id);
        //println!("draw_agent_los: zid:{:} {:?}", tc.zone_id, (tc.lx, tc.ly));
        let tile = zone.get_tile((tc.lx, tc.ly));
        // need to account multiple zones/reference points w/ portalling..
        draw_grid_tile(tile, world, display, base_x, base_y, tc.gx, tc.gy,
                       payload_cb);
    }
}