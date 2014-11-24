// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#![crate_id="p2d#0.1"]
#![crate_type="rlib"]
#![desc = "Backend/graphics-agnostic code for the p2d 2D graphics library"]
#![license = "MIT"]
#![feature(phase, globs)]

extern crate serialize;
extern crate uuid;
extern crate collections;
extern crate ecs;
#[phase(plugin, link)]
//extern crate debug;
extern crate log;

pub mod world;
pub mod zone;
pub mod portal;
pub mod sprite;
pub mod fov;
