// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use std::io::timer;
use time::precise_time_ns;

use gfx::GameDisplay;

pub fn throttle(fps: uint, cb: || -> bool) {
    let target_fps = (1000 / fps) as u64;
    loop {
        let next_frame = (precise_time_ns() / 1000000) + target_fps;
        match cb() {
            false => break,
            _ => {}
        }
        let now_time = precise_time_ns() / 1000000;
        if  now_time < next_frame {
            let sleep_gap = next_frame - now_time;
            timer::sleep(sleep_gap);
        }
    }
}

pub trait View<TOut: Send> {
    fn draw(&self, display: &GameDisplay, parent_draw: |&GameDisplay|);
    fn update(&mut self) -> Option<TOut>;
}
