// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.
use std::io::timer;
use std::comm::channel;
use std::cast::transmute;
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

fn cb_loop(cb: || -> bool) {
    loop {
        match cb() {
            false => break,
            _ => {}
        }
    }
}

pub trait DisplayViewContext {
    fn get_display<'a>(&'a self) -> &'a GameDisplay;
}

// ViewManager API exploration
pub trait ActiveView<TViewCtx: DisplayViewContext, TOut: Send> : PassiveView<TViewCtx> {
    fn active_update<'a>(&'a mut self, ctx: &TViewCtx, events: &[Event], ms_time: u64,
                         passives: & mut Vec<& mut PassiveView<TViewCtx> >)
                         -> Option<TOut>;
    fn yield_to<'a, TOut: Send>(&'a mut self, ctx: &TViewCtx,
                                active: &mut ActiveView<TViewCtx, TOut>,
                                passives: &mut Vec<&mut PassiveView<TViewCtx> >) -> TOut {
        let (sender, receiver) = channel();
        cb_loop(|| {
            match self._yield_inner(ctx, active, passives) {
                Some(out) => { sender.send(out); false },
                None => true
            }
        });
        receiver.recv()
    }
}

pub trait PassiveView<TViewCtx: DisplayViewContext> {
    fn passive_update(&mut self, ctx: &TViewCtx, ms_time: u64);
    fn _yield_inner<TOut: Send>(&mut self, ctx: &TViewCtx,
                         active: & mut ActiveView<TViewCtx, TOut>,
                         passives: & mut Vec<& mut PassiveView<TViewCtx> >) -> Option<TOut> {
        let time = precise_time_ns() / 1000000;

        // eh heh heh heh
        unsafe {
            let yielding = transmute(self as &mut PassiveView<TViewCtx>);
            passives.push(yielding);
        }
        let mut events = Vec::new();
        {
            for view in passives.mut_iter() {
                view.passive_update(ctx, time);
            }
        }
        active.passive_update(ctx, time);
        ctx.get_display().renderer.present();
        loop {
            match poll_event() {
                NoEvent => { break; },
                event => { events.push(event); }
            }
        }
        let result = active.active_update(ctx, events.as_slice(), time, passives);
        passives.pop();
        result
    }
}
