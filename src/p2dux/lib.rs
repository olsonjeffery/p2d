// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#[crate_id="p2dux#0.1"];
#[crate_type="rlib"];
#[desc = "All UX/Frontend-specific code in the p2d 2D-graphics library"];
#[license = "MIT"];

#[feature(globs)];

extern crate time = "time#0.10-pre";
extern crate serialize = "serialize#0.10-pre";
extern crate uuid = "uuid#0.10-pre";
extern crate collections = "collections#0.10-pre";
extern crate sdl2 = "sdl2";
extern crate p2d = "p2d";
use std::option::{Option, Some, None};
use std::io::timer;
use std::comm;

pub mod gfx;
pub mod ux;

pub enum UxEvent {
    Continue,
    Quit
}
pub trait UxManager<TOut: Send> {
    fn handle(&mut self, ms_since: u64, fps: uint) -> Option<TOut>;
    fn throttle(&mut self, fps: uint) -> TOut {
        let target_fps = (1000 / fps) as u64;

        let mut last_time = time::precise_time_ns() / 1000000;
        let mut fps_ctr = 0;
        let mut next_fps = last_time + 1000;
        let mut curr_fps = 0;
        let (sender, receiver) = comm::channel();
        loop {
            let now_time = time::precise_time_ns() / 1000000;
            let ms_since = now_time - last_time;
            last_time = now_time;
            // fps tracking
            fps_ctr = fps_ctr + 1;
            if now_time >= next_fps {
                next_fps = now_time + 1000;
                curr_fps = fps_ctr;
                fps_ctr = 0;
            }

            match self.handle(ms_since, curr_fps) {
                None => {
                    // main loop throttle to provided fps
                    let now_time = time::precise_time_ns() / 1000000;
                    let next_frame = last_time + target_fps;
                    if now_time < next_frame {
                        let sleep_gap = next_frame - now_time;
                        timer::sleep(sleep_gap);
                    }
                },
                Some(s) => { sender.send(Some(s)); break }
            }
        }
        receiver.recv().expect("Exited handler loop with None value.. shouldn't happen")
    }
}
