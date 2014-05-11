// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use sdl2::{rect, pixels};

use p2d::sprite::SpriteTile;
use gfx::GameDisplay;

pub mod menu;

pub trait UiFont {
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
    fn compute_len(&self, text: &str, gap: uint) -> uint {
        let mut total_len = 0;
        let text_slice = text.slice_from(0);
        for c in text_slice.chars() {
            let font_sprite = self.sprite_for(&c).expect(
                format!("Sprite not found for {:?}! Shouldn't happen...", c));
            let (fsx, _) = font_sprite.size;
            total_len += fsx + gap;
        }
        total_len
    }
}

pub trait UiBox {
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
        match display.renderer.fill_rect(&bg_rect) {
            Ok(()) => {},
            Err(e) => fail!("draw_box: failure in fill_rect(): {}", e)
        }
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

pub fn draw_text_box<TFont: UiFont, TBox: UiBox>(
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

pub fn compute_text_box_bounds<TFont: UiFont, TBox: UiBox>(
        lines: &[~str], ui_font: &TFont, ui_box: &TBox,
        text_gap: uint) -> (uint, uint) {
    // figure out width, in pixels, of the text (based on longest entry line)
    let mut longest_len = 0;
    for line in lines.iter() {
        let flen = ui_font.compute_len(*line, text_gap);
        if flen > longest_len {
            longest_len = flen;
        }
    }
    // figure out height, in pixels, of the text
    let (_, fy) = ui_font.sprite_for(&' ')
        .expect("compute_text_box_bounds(): expected a spritetile..").size;
    let font_height = (fy * lines.len()) +
        ((fy >> 2) * (lines.len() - 1));
    let box_unit_size = ui_box.unit_size();
    // compute menu box size from width/height info
    let font_h_units = font_height / box_unit_size;
    let padding_h = 2 + if (font_height % box_unit_size) > 0 { 1 } else { 0 };
    let box_h = font_h_units + padding_h;
    let font_w_units = longest_len / box_unit_size;
    let padding_w = 2 + if (longest_len % box_unit_size) > 0 { 1 } else { 0 };
    let box_w = font_w_units + padding_w;
    (box_w, box_h)
}
