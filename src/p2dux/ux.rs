// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use sdl2::{rect, pixels};

use p2d::sprite::{SpriteSheet, SpriteTile};
use gfx::GameDisplay;

pub trait SpriteFontSheet {
    fn get_sheet(&self) -> ~str;
    fn sprite_for<'a>(&'a self, c: &char) -> Option<&'a SpriteTile>;

    fn draw_line(&self, display: &GameDisplay, coords: (int, int), text: ~str) {
        let (mut cx, cy) = coords;
        let sheet = display.sheets.get(&self.get_sheet());
        let text_slice = text.slice_from(0);
        for c in text_slice.chars() {
            let font_sprite = self.sprite_for(&c).expect(format!("Sprite not found for {:?}! Shouldn't happen...", c));
            let (fsx, fsy) = font_sprite.size;
            sheet.draw_tile(display.renderer, font_sprite, (cx, cy), font_sprite.size);
            cx += (fsx+2) as int;
        }
    }
}

pub trait SpriteUxBox {
    fn unit_size(&self) -> uint;
    fn get_sheet(&self) -> ~str;
    fn get_ul_corner<'a>(&'a self) -> &'a SpriteTile;
    fn get_ur_corner<'a>(&'a self) -> &'a SpriteTile;
    fn get_ll_corner<'a>(&'a self) -> &'a SpriteTile;
    fn get_lr_corner<'a>(&'a self) -> &'a SpriteTile;
    fn get_top<'a>(&'a self) -> &'a SpriteTile;
    fn get_bottom<'a>(&'a self) -> &'a SpriteTile;
    fn get_left<'a>(&'a self) -> &'a SpriteTile;
    fn get_right<'a>(&'a self) -> &'a SpriteTile;
    fn draw_box(&self, display: &GameDisplay, coords: (int, int),
                size_in_units: (uint, uint), bg_color: (u8, u8, u8)) {
        let (start_x, start_y) = coords;
        let unit_size = self.unit_size() as int;
        let (w, h) = size_in_units;
        let (w, h) = (w as int, h as int);
        let sheet = display.sheets.get(&self.get_sheet());
        let tile_size = (unit_size as uint, unit_size as uint);
        // draw background
        let (r, g, b) = bg_color;
        let bgc = pixels::RGB(r, g, b);
        display.set_draw_sdl2_color(bgc);
        let (rect_w, rect_h) = (w*unit_size, h*unit_size);
        let bg_rect = rect::Rect::new(
            start_x as i32, start_y as i32, rect_w as i32, rect_h as i32);
        display.renderer.fill_rect(&bg_rect);
        // draw corners
        let (ul_x, ul_y) = coords;
        sheet.draw_tile(display.renderer, self.get_ul_corner(),
                        (ul_x, ul_y), tile_size);
        let (ur_x, ur_y) = (start_x + (unit_size * (w-1)) as int,
                        start_y);
        sheet.draw_tile(display.renderer, self.get_ur_corner(),
                        (ur_x, ur_y), tile_size);
        let (ll_x, ll_y) = (start_x,
                        start_y + (unit_size * (h-1)) as int);
        sheet.draw_tile(display.renderer, self.get_ll_corner(),
                        (ll_x, ll_y), tile_size);
        let (lr_x, lr_y) = (start_x + (unit_size * (w-1) as int),
                        start_y + (unit_size * (h-1)) as int);
        sheet.draw_tile(display.renderer, self.get_lr_corner(),
                        (lr_x, lr_y), tile_size);
        //top/bottom
        let (top_y, bottom_y) = (ul_y, ll_y);
        let mut tb_x = ul_x + unit_size;
        while tb_x < ur_x {
            let top_coords = (tb_x, top_y);
            let bottom_coords = (tb_x, bottom_y);
            sheet.draw_tile(display.renderer, self.get_top(),
                            top_coords, tile_size);
            sheet.draw_tile(display.renderer, self.get_bottom(),
                            bottom_coords, tile_size);
            tb_x += unit_size;
        }
        // left/right
        let (left_x, right_x) = (ul_x, ur_x);
        let mut left_right_y = ul_y + unit_size;
        while left_right_y < ll_y {
            let left_coords = (left_x, left_right_y);
            let right_coords = (right_x, left_right_y);
            sheet.draw_tile(display.renderer, self.get_left(),
                            left_coords, tile_size);
            sheet.draw_tile(display.renderer, self.get_right(),
                            right_coords, tile_size);
            left_right_y += unit_size;
        }
    }
}