// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use gfx::GameDisplay;
use super::super::ui::{UiFont, UiBox};
use super::{PassiveView, View};

pub struct TextInputDialogView<'a, TFont, TBox> {
    input_state: ~str,
    preface: Vec<~str>,
    prompt: ~str,
    cursor: ~str,
    bg_color: (u8, u8, u8),
    coords: (int, int),
    box_size: (uint, uint),
    text_gap: uint,
    ui_font: &'a TFont,
    ui_box: &'a TBox
}

pub struct DisplayClearerPassiveView {
    bg_color: (u8, u8, u8)
}

impl<'a, TFont: UiFont, TBox: UiBox> TextInputDialogView<'a, TFont, TBox> {
    pub fn new(ui_font: &'a TFont, ui_box: &'a TBox, seed_state: Option<~str>,
           preface: Vec<~str>, prompt: ~str, cursor: ~str, bg_color: (u8,u8,u8),
           coords: (int, int), text_gap: uint)
    -> TextInputDialogView<'a, TFont, TBox> {
        TextInputDialogView {
            input_state: match seed_state { Some(i) => i, None => ~"" },
            preface: preface,
            prompt: prompt,
            cursor: cursor,
            bg_color: bg_color,
            coords: coords,
            box_size: (0,0),
            text_gap: text_gap,
            ui_font: ui_font,
            ui_box: ui_box
        }
    }
}
impl<'a, TFont: UiFont, TBox: UiBox> View<~str> for TextInputDialogView<'a, TFont, TBox> {
    fn draw(&self, _display: &GameDisplay, _parent_draw: |&GameDisplay|) {
    }
    fn update(&mut self) -> Option<~str> {
       Some(~"#YOLO")
    }
}

impl DisplayClearerPassiveView {
    pub fn new(bgc: (u8, u8, u8)) -> DisplayClearerPassiveView {
        DisplayClearerPassiveView { bg_color: bgc }
    }
}
impl PassiveView for DisplayClearerPassiveView {
    fn update(&mut self, display: &GameDisplay, _time: u64) {
        display.set_draw_color(self.bg_color);
        match display.renderer.clear() {
            Err(e) => fail!("Display Clearer.update(): failed to clear display: {}", e),
            _ => {}
        }
    }
}
