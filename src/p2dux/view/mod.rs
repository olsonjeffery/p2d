// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use std::io::timer;
use std::comm::channel;
use time::precise_time_ns;

use sdl2::event::{Event, NoEvent, poll_event};

use gfx::GameDisplay;

pub mod prefab;

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

// ViewManager API exploration
pub trait ActiveView<TOut> : PassiveView {
    fn handle(& mut self, display: &GameDisplay, events: &[Event], time: u64,
              passives: & mut Vec<& mut PassiveView>, yielder: & mut ViewManager)
        -> Option<TOut>;
}
pub trait PassiveView {
    fn update(&mut self, display: &GameDisplay, time: u64);
}

pub struct ViewManager;

impl ViewManager {
    pub fn enter<'a, TOut: Send>(&mut self, display: &GameDisplay,
                           yielding: Option<&'a mut PassiveView>,
                           active: &mut ActiveView<TOut>,
                           passives: &mut Vec<&'a mut PassiveView>) -> TOut {
        let yielding_added = match yielding {
            Some(y) => { passives.push(y); true}, None => false
        };
        let (sender, receiver) = channel();
        throttle(1000, || {
            match self.enter_once(display, active, passives) {
                Some(out) => { sender.send(out); false }, None => true
            }
        });
        if yielding_added { passives.pop(); }
        receiver.recv()
    }
    pub fn enter_once<TOut: Send>(&mut self, display: &GameDisplay,
                           active: & mut ActiveView<TOut>,
                           passives: & mut Vec<& mut PassiveView>) -> Option<TOut> {
            let time = 0;
            let mut events = Vec::new();
            {
                for view in passives.mut_iter() {
                    view.update(display, time);
                }
            }
            active.update(display, time);
            loop {
                match poll_event() {
                    NoEvent => { break; },
                    event => { events.push(event); }
                }
            }
            active.handle(display, events.as_slice(), time, passives, self)
    }
}
