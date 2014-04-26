// Copyright 2013-2014 Jeffery Olson
//
// Licensed under the 3-Clause BSD License, see LICENSE.txt
// at the top-level of this repository.
// This file may not be copied, modified, or distributed
// except according to those terms.

//  a logical representation of a discrete area (a room, hallway, an entire
// floor of a tower/fortress, etc)
// zones are just game data.. no display/ptr stuff that could cause race
// conditions..

use std::option::{None, Some};
use std::vec_ng::Vec;
use collections::hashmap::HashMap;
use uuid::Uuid;

use super::world::{GlobalCoord, Payloadable};

pub fn coords_to_idx(coords: (uint, uint), size: uint) -> uint {
    let (x, y) = coords;
    x + (y * size)
}

pub enum ZoneTraversalResult {
    Destination(GlobalCoord),
    DestinationBlocked,
    DestinationOccupied(Uuid),
    DestinationOutsideBounds,
}

#[deriving(Encodable, Decodable)]
pub struct Tile<TPayload> {
    passable: bool,
    payload: TPayload,
    portal_id: Option<Uuid>
}

impl<TPayload: Send + Payloadable> Tile<TPayload> {
    pub fn stub() -> Tile<TPayload> {
        Tile {
            passable: false,
            payload: Tile::stub_payload(),
            portal_id: None
        }
    }

    pub fn stub_payload() -> TPayload {
        Payloadable::stub()
    }

    pub fn get_payload<'a>(&'a self) -> &'a TPayload {
        &self.payload
    }
}

#[deriving(Encodable, Decodable)]
pub struct Zone<TPayload> {
    id: Uuid,
    name: ~str,
    size: uint,
    all_tiles: Vec<Tile<TPayload>>,
    priv payload_coords: HashMap<Uuid, (uint, uint)>,
    priv portal_coords: HashMap<Uuid, (uint, uint)>
}

impl<TPayload: Send + Payloadable> Zone<TPayload> {
    pub fn new(size: uint, id: Uuid, name: ~str) -> Zone<TPayload> {
        // size must be a power of 2
        let mut z = Zone {
            id: id,
            name: name,
            size: size,
            all_tiles: Vec::with_capacity(size*size),
            payload_coords: HashMap::new(),
            portal_coords: HashMap::new()
        };
        let limit = size*size;
        let mut ctr = 0;
        while ctr < limit {
            let tile: Tile<TPayload> = Tile::<TPayload>::stub();
            z.all_tiles.push(tile);
            ctr += 1;
        }
        z
    }
    ///////////////////////
    // coordinate information for things within the Zone
    ///////////////////////
    pub fn get_payload_coords<'a>(&'a self, plid: &Uuid) -> &'a (uint, uint) {
        self.payload_coords.find(plid).expect(
            format!("Unable to find coords for payload {:?}", plid))
    }
    pub fn get_portal_coords<'a>(&'a self, pid: &Uuid) -> &'a (uint, uint) {
        self.portal_coords.find(pid).expect(format!("Unable to find coords for portal {:?}", pid))
    }
    pub fn coords_in_bounds(&self, coords: (uint, uint)) -> bool {
        let (x, y) = coords;
        return x < self.size && y < self.size
    }
    ///////////////////////
    // Tile related
    ///////////////////////
    pub fn tile_at_idx<'a>(&'a self, idx: uint) -> &'a Tile<TPayload> {
        self.all_tiles.get(idx)
    }
    pub fn tile_at_idx_mut<'a>(&'a mut self, idx: uint) -> &'a mut Tile<TPayload> {
        self.all_tiles.get_mut(idx)
    }
    pub fn get_tile<'a>(&'a self, coords: (uint, uint)) -> &'a Tile<TPayload> {
        let idx = coords_to_idx(coords, self.size);
        self.tile_at_idx(idx)
    }
    pub fn get_tile_mut<'a>(&'a mut self, coords: (uint, uint)) -> &'a mut Tile<TPayload> {
        let idx = coords_to_idx(coords, self.size);
        self.tile_at_idx_mut(idx)
    }
    ///////////////////////
    // adding/moving entities
    ///////////////////////
    pub fn add_portal(&mut self, pid: Uuid, coords: (uint, uint)) {
        if !self.coords_in_bounds(coords) {
            let (x, y) = coords;
            fail!("add_portal: coords {:?},{:?} aren't in bounds!", x, y);
        }
        // can only add a portal to a zone once..
        if self.portal_coords.find(&pid).is_some() {
            fail!("add_portal: portal {:?} already added to zone {:?}!", pid, self.id);
        }
        {
            let t = self.get_tile_mut(coords);
            t.portal_id = Some(pid);
        }
        self.portal_coords.insert(pid, coords);
    }
}
