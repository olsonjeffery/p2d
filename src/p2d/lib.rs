// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#[link(name = "p2d#0.1",
       uuid = "b39753cc-a018-4873-9bbf-38cd4497df96",
       url = "http://github.com/olsonjeffery/p2d")];
#[crate_id="p2d#0.1"];
#[desc = "Backend/graphics-agnostic code for the p2d 2D graphics library"];
#[license = "MIT"];

#[feature(globs)];

extern mod extra;

pub mod world;
pub mod zone;
pub mod portal;
pub mod sprite;
pub mod fov;
