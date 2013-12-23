use std::hashmap::HashMap;
use std::option::{Some, None};

use super::GameDisplay;
use super::texture::TextureSheets;
use p2d::sim::world::{World, GlobalCoord};
use p2d::sim::sprite::SpriteSheet;
use p2d::sim::zone::{Zone, Tile};

// so this is assuming uniform tile size amongst a group of tiles in a
// Zone
pub fn draw_grid_tile(t: &Tile, world: &World, display: &GameDisplay,
                base_x: int, base_y: int, zone_x: int, zone_y: int) {
    let sheets = &display.sheets;
    // draw tile sprites
    for st in t.sprites.iter() {
        let sheet = sheets.get(&st.sheet);
        let (tile_size_x, tile_size_y) = st.size;
        // should only need to get screen_x once this whole thing..
        let screen_x = base_x + (zone_x * tile_size_x as int) as int;
        let screen_y = base_y + (zone_y * tile_size_y as int) as int;
        sheet.draw_tile(display.renderer, st,
                        (screen_x, screen_y), st.size);
    }
    // draw object sprite
    // ...

    // draw actor sprite
    match t.agent_id {
        None => {},
        Some(id) => {
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
        }
    }
}

pub fn draw_agent_los(world: &World, visible_tiles: &~[GlobalCoord], display: &GameDisplay) {
    // screen center
    let (base_x, base_y) = (400, 300);
    for tc in visible_tiles.iter() {
        let zone = world.get_zone(tc.zone_id);
        //println!("draw_agent_los: zid:{:} {:?}", tc.zone_id, (tc.lx, tc.ly));
        let tile = zone.get_tile((tc.lx, tc.ly));
        // need to account multiple zones/reference points w/ portalling..
        draw_grid_tile(tile, world, display, base_x, base_y, tc.gx, tc.gy);
    }
}