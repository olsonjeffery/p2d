// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::vec::Vec;

use gfx::GameDisplay;

use super::{UiBox, UiFont, draw_text_box, compute_text_box_bounds};

pub struct VertTextMenu<TFont, TBox> {
    pub entries: Vec<~str>,
    pub formatted_entries: Vec<~str>,
    pub bg_color: (u8, u8, u8),
    pub selected_prefix: ~str,
    pub unselected_prefix: ~str,
    pub curr_selected: uint,
    pub coords: (int, int),
    pub box_size: (uint, uint),
    pub text_gap: uint
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
        self.formatted_entries = Vec::new();
        for v in range(0, self.entries.len()) {
            let formatted = self.get_formatted(v);
            self.formatted_entries.push(formatted);
        }
        self.box_size = compute_text_box_bounds(
            self.formatted_entries.as_slice(), ui_font, ui_box, self.text_gap);
        self.coords = coords;
    }

    pub fn draw_menu(&self, display: &GameDisplay, ui_font: &TFont, ui_box: &TBox) {
        draw_text_box(
            display, self.coords, self.box_size, self.bg_color,
            self.formatted_entries.slice_from(0), ui_font, ui_box, self.text_gap);
    }
}
