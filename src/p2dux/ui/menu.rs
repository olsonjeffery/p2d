// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec_ng::Vec;

use gfx::GameDisplay;

use super::{UiBox, UiFont, draw_text_box};

pub struct VertTextMenu<TFont, TBox> {
    entries: Vec<~str>,
    formatted_entries: Vec<~str>,
    bg_color: (u8, u8, u8),
    selected_prefix: ~str,
    unselected_prefix: ~str,
    curr_selected: uint,
    coords: (int, int),
    box_size: (uint, uint),
    text_gap: uint
}

impl<TFont: UiFont, TBox: UiBox>
        VertTextMenu<TFont, TBox> {
    pub fn new() -> VertTextMenu<TFont, TBox> {
        VertTextMenu {
            entries: Vec::new(),
            formatted_entries: Vec::new(),
            bg_color: (0,0,0),
            selected_prefix: ~"",
            unselected_prefix: ~"",
            curr_selected: 0,
            coords: (0,0),
            box_size: (0,0),
            text_gap: 2
        }
    }
    pub fn move_down(&mut self) {
        let last_idx = self.entries.len() - 1;
        if self.curr_selected < last_idx {
            let new_idx = self.curr_selected + 1;
            self.update_selected(new_idx);
        }
    }
    pub fn move_up(&mut self) {
        if self.curr_selected > 0 {
            let new_idx = self.curr_selected - 1;
            self.update_selected(new_idx);
        }
    }
    fn update_selected(&mut self, new_idx: uint) {
        let old_selected = self.curr_selected;
        self.curr_selected = new_idx;
        let selected_formatted = self.get_formatted(self.curr_selected);
        self.formatted_entries.push(selected_formatted);
        self.formatted_entries.swap_remove(self.curr_selected);
        let unselected_formatted = self.get_formatted(old_selected);
        self.formatted_entries.push(unselected_formatted);
        self.formatted_entries.swap_remove(old_selected);
    }
    fn get_formatted(&mut self, v: uint) -> ~str {
        let entry = self.entries.get(v);
        let prefix = if v == self.curr_selected {
            &self.selected_prefix
        } else {
            &self.unselected_prefix
        };
        format!("{} {}", *prefix, *entry)
    }
    pub fn update_bounds(&mut self, coords: (int, int), ui_font: &TFont, ui_box: &TBox) {
        // figure out width, in pixels, of the text (based on longest entry line)
        let mut longest_len = 0;
        self.formatted_entries = Vec::new();
        for v in range(0, self.entries.len()) {
            let formatted = self.get_formatted(v);
            let flen = ui_font.compute_len(formatted, self.text_gap);
            self.formatted_entries.push(formatted);
            if flen > longest_len {
                longest_len = flen;
            }
        }
        // figure out height, in pixels, of the text
        let (_, fy) = ui_font.sprite_for(&' ')
            .expect("update_bounds(): expected a spritetile..").size;
        let font_height = (fy * self.entries.len()) +
            ((fy >> 2) * (self.entries.len() - 1));
        let box_unit_size = ui_box.unit_size();
        // compute menu box size from width/height info
        let font_h_units = font_height / box_unit_size;
        let padding_h = 2 + if (font_height % box_unit_size) > 0 { 1 } else { 0 };
        let box_h = font_h_units + padding_h;
        let font_w_units = longest_len / box_unit_size;
        let padding_w = 2 + if (longest_len % box_unit_size) > 0 { 1 } else { 0 };
        let box_w = font_w_units + padding_w;
        self.box_size = (box_w, box_h);
        self.coords = coords;
    }

    pub fn draw_menu(&self, display: &GameDisplay, ui_font: &TFont, ui_box: &TBox) {
        draw_text_box(
            display, self.coords, self.box_size, self.bg_color,
            self.formatted_entries.slice_from(0), ui_font, ui_box, self.text_gap);
    }
}
