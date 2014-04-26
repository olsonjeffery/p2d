// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use uuid::Uuid;

use super::world::{TraversalDirection, North, East, South, West};

#[deriving(Encodable, Decodable)]
pub struct Portal {
    id: Uuid,
    a_zid: Uuid,
    a_exit: TraversalDirection,
    b_zid: Uuid,
    b_exit: TraversalDirection
}

impl Portal {
    pub fn new(id: Uuid, a_zid: Uuid, ae: TraversalDirection,
               b_zid: Uuid, bx: TraversalDirection) -> Portal {
        if ae == North && bx != South { fail!("bad portal dirs a:{:?} b:{:?}", ae, bx); }
        if ae == South && bx != North { fail!("bad portal dirs a:{:?} b:{:?}", ae, bx); }
        if ae == West && bx != East { fail!("bad portal dirs a:{:?} b:{:?}", ae, bx); }
        if ae == East && bx != West { fail!("bad portal dirs a:{:?} b:{:?}", ae, bx); }
        Portal { id: id, a_zid: a_zid, a_exit: ae, b_zid: b_zid, b_exit: bx }
    }
    pub fn info_from(&self, zid: Uuid) -> (Uuid, TraversalDirection) {
        if self.a_zid == zid { (self.b_zid, self.a_exit) }
        else if self.b_zid == zid { (self.a_zid, self.b_exit) }
        else { fail!("zid:{:?} isn't in this portal!", zid) }
    }
}
