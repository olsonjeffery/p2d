// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

use std::cmp::{PartialEq, Eq};
use std::hash::Hash;
use std::collections::HashMap;
use uuid::Uuid;

use super::zone::{Zone, ZoneTraversalResult, Destination,
                  DestinationOutsideBounds, DestinationBlocked};
use super::portal::Portal;

#[deriving(Decodable, Encodable, Eq, PartialEq, Hash, Show)]
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
pub struct World<TWorldPayload, TZonePayload, TTilePayload> {
    pub data: TWorldPayload,
    pub zones: HashMap<Uuid, Zone<TZonePayload, TTilePayload>>,
    pub portals: HashMap<Uuid, Portal>,
}

#[deriving(Eq, PartialEq, Hash, Clone, Encodable, Decodable)]
pub struct GlobalCoord {
    pub zone_id: Uuid,
    pub coords: (uint, uint)
}

impl GlobalCoord {
    pub fn new(zone_id: Uuid, coords: (uint, uint)) -> GlobalCoord {
        GlobalCoord {
            zone_id: zone_id,
            coords: coords
        }
    }
}

#[deriving(Hash, Eq, PartialEq)]
pub struct RelativeCoord {
    pub zone_id: Uuid,
    pub lx: uint,
    pub ly: uint,
    pub gx: int,
    pub gy: int
}

impl RelativeCoord {
    pub fn new(zone_id: Uuid, coords: (uint, uint), g_coords: (int, int)) -> RelativeCoord {
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

impl<TWorldPayload, TZonePayload, TTilePayload: Send + Payloadable>
        World<TWorldPayload, TZonePayload, TTilePayload> {
    pub fn new(data: TWorldPayload) -> World<TWorldPayload, TZonePayload, TTilePayload> {
        World {
            data: data,
            zones: HashMap::new(),
            portals: HashMap::new(),
        }
    }

    pub fn new_zone(&mut self, size: uint, data: TZonePayload,
                    cb: |&mut Zone<TZonePayload, TTilePayload>|) -> Uuid {
        let zone_id = Uuid::new_v4();
        let z = Zone::<TZonePayload, TTilePayload>::new(size, zone_id, data);
        self.zones.insert(zone_id, z);
        cb(self.zones.get_mut(&zone_id).unwrap());
        zone_id
    }

    pub fn new_portal(&mut self, a: (Uuid, (uint, uint), TraversalDirection),
                      b: (Uuid, (uint, uint), TraversalDirection)) -> Uuid {
        let next_id = Uuid::new_v4();
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
        next_id
    }

    // Entity lookup
    pub fn get_payload<'a>(&'a self, gc: &GlobalCoord) -> &'a TTilePayload {
        let zone = self.get_zone(&gc.zone_id);
        &zone.get_tile(gc.coords).payload
    }
    pub fn get_payload_mut<'a>(&'a mut self, gc: &GlobalCoord) -> &'a mut TTilePayload {
        let zone = self.get_zone_mut(&gc.zone_id);
        &mut zone.get_tile_mut(gc.coords).payload
    }
    pub fn get_zone<'a>(&'a self, id: &Uuid) -> &'a Zone<TZonePayload, TTilePayload> {
        self.zones.find(id).expect(format!("Cannot find zone with id {}", id).as_slice())
    }
    pub fn get_zone_mut<'a>(&'a mut self, id: &Uuid) -> &'a mut Zone<TZonePayload, TTilePayload> {
        self.zones.find_mut(id).expect(format!("Cannot find_mut zone with id {}", id).as_slice())
    }

    pub fn get_portal<'a>(&'a self, id: Uuid) -> &'a Portal {
        self.portals.find(&id).expect(format!("Cannot find portal with id {}", id).as_slice())

    }

    /// Try traversing from one `GlobalCoord` to another.
    pub fn try_traversal(&self, src: GlobalCoord, dir: TraversalDirection) -> ZoneTraversalResult {
        let delta: (int, int) = match dir {
            North => (0, -1),
            West => (-1, 0),
            South => (0, 1),
            East => (1, 0),
            NoDirection => panic!("NoDirection not allowed in traverse()")
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
                        println!("portal dir: {} traversing dir: {}", td, dir);
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
                    NoDirection => panic!("NoDirection not allowed in traverse()")
                };
                println!("other zone: {}, this zone: {}", ozid, curr_zone_id);
                (ozid, oc)
            } else {
                let dest_coords = (curr_x as int + d_x, curr_y as int + d_y);
                (curr_zone_id, dest_coords)
            }
        };
        let dest_zone = self.get_zone(&dest_zone_id);
        let (dx, dy) = dest_coords;
        println!("Dir {} Delta {} src: {} dest: {}",dir,delta,src.coords, dest_coords);
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
}
