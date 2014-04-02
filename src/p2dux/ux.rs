// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec_ng::Vec;

use sdl2::{rect, pixels};

use p2d::sprite::SpriteTile;
use gfx::GameDisplay;

pub trait SpriteUxFont {
    fn get_sheet(&self) -> ~str;
    fn sprite_for<'a>(&'a self, c: &char) -> Option<&'a SpriteTile>;

    fn draw_line(&self, display: &GameDisplay, coords: (int, int), text: &str, gap: uint) {
        let (mut cx, cy) = coords;
        let sheet = display.sheets.get(&self.get_sheet());
        let text_slice = text.slice_from(0);
        for c in text_slice.chars() {
            let font_sprite = self.sprite_for(&c).expect(
                format!("Sprite not found for {:?}! Shouldn't happen...", c));
            let (fsx, _) = font_sprite.size;
            sheet.draw_tile(display.renderer, font_sprite, (cx, cy), font_sprite.size);
            cx += (fsx+gap) as int;
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

pub fn draw_text_box<TFont: SpriteUxFont, TBox: SpriteUxBox>(
        display: &GameDisplay, coords: (int, int), size_in_units: (uint, uint),
        bg_color: (u8, u8, u8), lines: &[~str], ux_font: &TFont, ux_box: &TBox,
        gap: uint) {
    // draw backing box
    ux_box.draw_box(display, coords, size_in_units, bg_color);
    // info to draw boxed text (note we aren't doing any bounds checking..)
    let box_unit_size = ux_box.unit_size();
    let (start_x, start_y) = coords;
    let start_x = start_x + box_unit_size as int;
    let mut curr_y = start_y + box_unit_size as int;
    for curr_line in lines.iter() {
        let l_coords = (start_x as int, curr_y as int);
        ux_font.draw_line(display, l_coords, *curr_line, gap);
        curr_y += box_unit_size as int + (box_unit_size >> 2) as int;
    }
}

pub trait MenuEntry<TContext> {
    fn get_name(&self) -> ~str;
    fn on_selected<TFont: SpriteUxFont, TBox: SpriteUxBox>(
        &self, menu: &mut SpriteUxMenuBox<TFont, TBox, TContext>,
        ctx: &mut TContext);
}

pub struct SpriteUxMenuBox<TFont, TBox, TContext> {
    entries: Vec<~MenuEntry<TContext>>,
    bg_color: (u8, u8, u8),
    selected_prefix: ~str,
    unselected_prefix: ~str,
    curr_selected: uint,
    bounds: (int, int, uint, uint),
}

impl<TFont: SpriteUxFont, TBox: SpriteUxBox, TContext>
        SpriteUxMenuBox<TFont, TBox, TContext>{
    pub fn new() -> SpriteUxMenuBox<TFont, TBox, TContext> {
        SpriteUxMenuBox {
            entries: Vec::new(),
            bg_color: (0,0,0),
            selected_prefix: ~"",
            unselected_prefix: ~"",
            curr_selected: 0,
            bounds: (0,0,0,0)
        }
    }
}
