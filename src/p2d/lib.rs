// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#[crate_id="p2d#0.1"];
#[crate_type="rlib"];
#[desc = "Backend/graphics-agnostic code for the p2d 2D graphics library"];
#[license = "MIT"];

#[feature(globs)];

extern mod extra = "extra#0.10-pre";
extern mod serialize = "serialize#0.10-pre";
extern mod uuid = "uuid#0.10-pre";

pub mod world;
pub mod zone;
pub mod portal;
pub mod sprite;
pub mod fov;
