// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use sdl2::event::Event;

use gfx::GameDisplay;
use super::super::ui::{UiFont, UiBox};
use super::{ActiveView, PassiveView};

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
impl<'a, TFont: UiFont, TBox: UiBox> ActiveView<~str> for TextInputDialogView<'a, TFont, TBox> {
    fn active_update<'a>(
        &'a mut self,
        _display: &GameDisplay,
        _events: &[Event],
        _ms_time: u64,
        _passives: & mut Vec<& mut PassiveView>) -> Option<~str> {
        //
        Some(~"#YOLO")
    }
}

impl<'a, TFont: UiFont, TBox: UiBox> PassiveView for TextInputDialogView<'a, TFont, TBox> {
    fn passive_update(&mut self, _display: &GameDisplay, _t: u64) {
    }
}

impl DisplayClearerPassiveView {
    pub fn new(bgc: (u8, u8, u8)) -> DisplayClearerPassiveView {
        DisplayClearerPassiveView { bg_color: bgc }
    }
}
impl PassiveView for DisplayClearerPassiveView {
    fn passive_update(&mut self, display: &GameDisplay, _time: u64) {
        display.set_draw_color(self.bg_color);
        match display.renderer.clear() {
            Err(e) => fail!("Display Clearer.update(): failed to clear display: {}", e),
            _ => {}
        }
    }
}
impl ActiveView<()> for DisplayClearerPassiveView {
    fn active_update<'a>(&'a mut self, _d: &GameDisplay, _e: &[Event], _t: u64,
              _p: & mut Vec<& mut PassiveView>)
        -> Option<()> {
            fail!("this should never be called.");
    }
}
