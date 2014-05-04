// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_id="p2dux#0.1"]
#![crate_type="rlib"]
#![desc = "All UX/Frontend-specific code in the p2d 2D-graphics library"]
#![license = "MIT"]
#![feature(globs)]

extern crate time = "time";
extern crate serialize = "serialize";
extern crate uuid = "uuid";
extern crate collections = "collections";
extern crate sdl2 = "sdl2";
extern crate p2d = "p2d";
use time::precise_time_ns;

pub mod gfx;
pub mod ui;
pub mod view;

pub struct TimeTracker {
    pub last_time: u64,
    pub now_time: u64,
    pub next_fps_time: u64,
    pub fps_ctr: uint,
    pub curr_fps: uint
}
impl TimeTracker {
    pub fn new() -> TimeTracker {
        let curr_time = precise_time_ns() / 1000000u64;
        let mut tt = TimeTracker {
            last_time: curr_time,
            now_time: curr_time,
            next_fps_time: curr_time + 1000u64,
            fps_ctr: 0,
            curr_fps: 0
        };
        tt.update();
        tt
    }
    pub fn update(&mut self) {
        self.last_time = self.now_time;
        self.now_time = precise_time_ns() / 1000000u64;
        if self.now_time >= self.next_fps_time {
            self.curr_fps = self.fps_ctr;
            self.fps_ctr = 0;
            self.next_fps_time = self.now_time + 1000u64;
        } else {
            self.fps_ctr += 1;
        }
    }
    pub fn get_curr_fps(&self) -> uint { self.curr_fps }
    pub fn get_ms_since(&self) -> uint { (self.now_time - self.last_time) as uint }
}
