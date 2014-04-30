// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

#[deriving(Clone, Encodable, Decodable)]
pub struct SpriteSheet {
    pub path: ~str,
    pub name: ~str
}

#[deriving(Encodable, Decodable)]
pub struct SpriteTile {
    pub sheet: ~str,
    pub coords: (uint, uint),
    pub size: (uint, uint)
}

impl Clone for SpriteTile {
    fn clone(&self) -> SpriteTile {
        SpriteTile {
            sheet: self.sheet.clone(),
            coords: (self.coords),
            size: (self.size)
        }
    }
}
impl SpriteTile {
    pub fn stub() -> SpriteTile { SpriteTile { sheet: ~"", coords: (0,0), size:(0,0) } }
}
