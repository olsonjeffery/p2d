// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::libc::{c_int};
use std::libc;
use std::cmp::{Eq, TotalEq};
use std::hash::Hash;
use collections::hashmap::{HashMap, HashSet};
use uuid::Uuid;
use serialize::{Decodable, Encodable};

use super::zone::{Zone, ZoneTraversalResult, Destination, DestinationOutsideBounds, DestinationBlocked};
use super::portal::Portal;
use super::fov;

#[deriving(Decodable, Encodable, Eq,Hash)]
pub enum TraversalDirection {
    North,
    East,
    South,
    West,
    NoDirection
}
impl TraversalDirection {
    pub fn opposite(&self) -> TraversalDirection {
        match self {
            &North => South,
            &South => North,
            &West => East,
            &East => West,
            &NoDirection => NoDirection
        }
    }
}

pub trait Payloadable {
    fn stub() -> Self;
}

#[deriving(Decodable, Encodable)]
pub struct World<TPayload> {
    zones: HashMap<uint, Zone<TPayload>>,
    portals: HashMap<uint, Portal>,
    latest_zone_id: uint,
    latest_portal_id: uint,
}

#[deriving(Eq, TotalEq, Hash, Clone, Encodable, Decodable)]
pub struct GlobalCoord {
    zone_id: uint,
    coords: (uint, uint)
}

impl GlobalCoord {
    pub fn new(zone_id: uint, coords: (uint, uint)) -> GlobalCoord {
        GlobalCoord {
            zone_id: zone_id,
            coords: coords
        }
    }
}

#[deriving(Hash)]
pub struct RelativeCoord {
    zone_id: uint,
    lx: uint,
    ly: uint,
    gx: int,
    gy: int
}

impl RelativeCoord {
    pub fn new(zone_id: uint, coords: (uint, uint), g_coords: (int, int)) -> RelativeCoord {
        let (x, y) = coords;
        let (gx, gy) = g_coords;
        RelativeCoord {
            zone_id: zone_id,
            lx: x,
            ly: y,
            gx: gx,
            gy: gy
        }
    }
}
impl Eq for RelativeCoord {
    fn eq(&self, other: &RelativeCoord) -> bool {
        return self.zone_id == other.zone_id
            && self.gx == other.gx
            && self.gy == other.gy
            && self.lx == other.lx
            && self.ly == other.ly
    }
}
impl TotalEq for RelativeCoord {
    fn equals(&self, other: &RelativeCoord) -> bool {
        return self.zone_id == other.zone_id
            && self.gx == other.gx
            && self.gy == other.gy
            && self.lx == other.lx
            && self.ly == other.ly
    }
}

impl<TPayload: Send + Payloadable> World<TPayload> {
    pub fn new() -> World<TPayload> {
        let mut w = World {
            zones: HashMap::new(),
            portals: HashMap::new(),
            latest_zone_id: 0,
            latest_portal_id: 0,
        };
        w
    }

    // Entity creation
    /*
    pub fn new_payload(&mut self, zone_id: uint, coords: (uint, uint), payload: TPayload) {
        let next_id = payload.get_id();
        if self.payloads.contains_key(&next_id) {
            fail!("A payload with the id:{:?} is already placed within the World!", next_id);
        }
        {
            let mut zone = self.get_zone_mut(&zone_id);
            zone.move_payload(coords, next_id);
        }
        let ed = EntityData {
            payload: payload,
            zone_id: zone_id
        };
        self.payloads.insert(next_id, ed);
    }
    */

    pub fn new_zone(&mut self, size: uint, cb: |&mut Zone<TPayload>|) -> uint {
        let next_id = self.latest_zone_id + 1;
        let mut z = Zone::<TPayload>::new(size, next_id);
        self.zones.insert(next_id, z);
        self.latest_zone_id = next_id;
        cb(self.zones.get_mut(&next_id));
        next_id
    }

    pub fn new_portal(&mut self, a: (uint, (uint, uint), TraversalDirection),
                      b: (uint, (uint, uint), TraversalDirection)) -> uint {
        let next_id = self.latest_portal_id + 1;
        let (az, ac, ax) = a;
        let (bz, bc, bx) = b;
        let portal = Portal::new(next_id, az, ax, bz, bx);
        self.portals.insert(next_id, portal);
        {
            let zone_a = self.get_zone_mut(&az);
            zone_a.add_portal(next_id, ac);
        }
        {
            let zone_b = self.get_zone_mut(&bz);
            zone_b.add_portal(next_id, bc);
        }
        self.latest_portal_id = next_id;
        next_id
    }

    // Entity lookup
    pub fn get_payload<'a>(&'a self, gc: &GlobalCoord) -> &'a TPayload {
        let zone = self.get_zone(&gc.zone_id);
        &zone.get_tile(gc.coords).payload
    }
    pub fn get_payload_mut<'a>(&'a mut self, gc: &GlobalCoord) -> &'a mut TPayload {
        let zone = self.get_zone_mut(&gc.zone_id);
        &mut zone.get_tile_mut(gc.coords).payload
    }
    pub fn get_zone<'a>(&'a self, id: &uint) -> &'a Zone<TPayload> {
        self.zones.find(id).expect(format!("Cannot find zone with id {:?}", id))
    }
    pub fn get_zone_mut<'a>(&'a mut self, id: &uint) -> &'a mut Zone<TPayload> {
        self.zones.find_mut(id).expect(format!("Cannot find_mut zone with id {:?}", id))
    }

    pub fn get_portal<'a>(&'a self, id: uint) -> &'a Portal {
        self.portals.find(&id).expect(format!("Cannot find portal with id {:?}", id))

    }

    /// Try traversing from one `GlobalCoord` to another.
    pub fn try_traversal(&self, src: GlobalCoord, dir: TraversalDirection) -> ZoneTraversalResult {
        let delta: (int, int) = match dir {
            North => (0, -1),
            West => (-1, 0),
            South => (0, 1),
            East => (1, 0),
            NoDirection => fail!("NoDirection not allowed in traverse()")
        };
        let curr_zone_id = src.zone_id;
        let (dest_zone_id, dest_coords) = {
            let curr_zone = self.get_zone(&curr_zone_id);
            let (d_x, d_y) = delta;
            let curr_coords = src.coords;
            let (curr_x, curr_y) = curr_coords;
            let traversing_portal = {
                let curr_tile = curr_zone.get_tile(curr_coords);
                match curr_tile.portal_id {
                    Some(pid) => {
                        let portal = self.get_portal(pid);
                        let (_, td) = portal.info_from(curr_zone.id);
                        println!("portal dir: {:?} traversing dir: {:?}", td, dir);
                        td == dir
                    },
                    None => false
                }
            };
            if traversing_portal {
                let curr_tile = curr_zone.get_tile(curr_coords);
                let pid = curr_tile.portal_id.expect("None for portal_id.. shouldn't happen.");
                let portal = self.get_portal(pid);
                let (ozid, td) = portal.info_from(curr_zone.id);
                let other_zone = self.get_zone(&ozid);
                let (ocx, ocy) = *other_zone.get_portal_coords(&pid);
                let oc = match td {
                    North => (ocx as int, ocy as int-1 as int),
                    East => (ocx as int+1 as int, ocy as int),
                    South => (ocx as int, ocy as int+1 as int),
                    West => (ocx as int-1 as int, ocy as int),
                    NoDirection => fail!("NoDirection not allowed in traverse()")
                };
                println!("other zone: {:?}, this zone: {:?}", ozid, curr_zone_id);
                (ozid, oc)
            } else {
                let dest_coords = (curr_x as int + d_x, curr_y as int + d_y);
                (curr_zone_id, dest_coords)
            }
        };
        let dest_zone = self.get_zone(&dest_zone_id);
        let (dx, dy) = dest_coords;
        println!("Dir {:?} Delta {:?} src: {:?} dest: {:?}",dir,delta,src.coords, dest_coords);
        if dx < 0 || dy < 0 || dx >= dest_zone.size as int|| dy >= dest_zone.size as int{
            DestinationOutsideBounds
        } else {
            let clean_dc = (dx as uint, dy as uint);
            let dest_tile = dest_zone.get_tile(clean_dc);
            if !dest_tile.passable {
                DestinationBlocked
            }
            else {
                Destination(GlobalCoord::new(dest_zone_id, clean_dc))
            }
        }
            
    }
/*
    pub fn move_agent(&mut self, aid: uint, zid: uint, dst: (uint, uint)) -> ZoneTraversalResult {
        let mut curr_zone_id = {
            let agent = self.get_agent(&aid);
            agent.zone_id
        };
        if zid != curr_zone_id {
            let old_zone = self.get_zone_mut(&curr_zone_id);
            old_zone.remove_agent(aid);
        }
        {
            let agent = self.get_agent_mut(&aid);
            agent.zone_id = zid;
        }
        let mut zone = self.get_zone_mut(&zid);
        zone.move_agent(aid, dst)
    }
*/
}
