#[link(name = "p2dux#0.1",
       uuid = "263e35b2-0727-11e3-b3dd-29219a890b3c",
       url = "http://github.com/olsonjeffery/p2d")];
#[crate_id="p2dux#0.1"];
#[desc = "All UX/Frontend-specific code in the p2d 2D-graphics library"];
#[license = "MIT"];

#[feature(globs)];

extern mod extra;
extern mod sdl2;
extern mod p2d;
use std::option::{Option, Some, None};
use std::io::timer;
use extra::time;

use sdl2::event::{QuitEvent, KeyDownEvent, NoEvent, poll_event};
use sdl2::keycode::*;

use p2d::world::{World, Payload};

pub mod gfx;

pub enum UxEvent {
    Continue,
    Quit
}
pub fn default_handler<T: Send + Payload>(world: &mut World<T>, display: &gfx::GameDisplay) -> UxEvent {
    let mut ret_event = Continue;
    let mut stop_loop = false;
    'event : loop {
        // input events
        match poll_event() {
            QuitEvent(_) => {
                ret_event = Quit;
                stop_loop = true;
            },
            KeyDownEvent(_, _, key, _, _) => {
                if key == EscapeKey {
                    ret_event = Quit;
                    stop_loop = true;
                }
            },
            NoEvent => {
                stop_loop = true;
            },
            _ => { }
        }
        if stop_loop {
            break 'event
        }
    }
    ret_event
}
pub trait UxManager<TOut> {
    fn handle(&mut self) -> Option<TOut>;
    fn throttle(&mut self, fps: uint) -> TOut {
        let target_fps = (1000 / fps) as u64;

        let mut last_time = time::precise_time_ns() / 1000000;
        let mut fps_ctr = 0;
        let mut next_fps = last_time + 1000;
        let mut exit_val = None;
        loop {
            let now_time = time::precise_time_ns() / 1000000;
            let ms_since = now_time - last_time;
            last_time = now_time;
            // fps tracking
            fps_ctr = fps_ctr + 1;
            if now_time >= next_fps {
                next_fps = now_time + 1000;
                //println!("{} fps", fps_ctr);
                fps_ctr = 0;
            }

            match self.handle() {
                None => {
                    // main loop throttle to 60 fps
                    let now_time = time::precise_time_ns() / 1000000;
                    let next_frame = last_time + target_fps;
                    if (now_time < next_frame) {
                        let sleep_gap = next_frame - now_time;
                        //println!("sleeping for {}", sleep_gap);
                        timer::sleep(sleep_gap);
                    }
                },
                Some(s) => { exit_val = Some(s); break }
            }
        }
        exit_val.expect("Exited handler loop with None value.. shouldn't happen")
    }
}
