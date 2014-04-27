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
pub struct Tile<TTilePayload> {
    passable: bool,
    payload: TTilePayload,
    portal_id: Option<Uuid>
}

impl<TTilePayload: Send + Payloadable> Tile<TTilePayload> {
    pub fn stub() -> Tile<TTilePayload> {
        Tile {
            passable: false,
            payload: Tile::stub_payload(),
            portal_id: None
        }
    }

    pub fn stub_payload() -> TTilePayload {
        Payloadable::stub()
    }

    pub fn get_payload<'a>(&'a self) -> &'a TTilePayload {
        &self.payload
    }
}

#[deriving(Encodable, Decodable)]
pub struct Zone<TZonePayload, TTilePayload> {
    id: Uuid,
    data: TZonePayload,
    size: uint,
    all_tiles: Vec<Tile<TTilePayload>>,
    priv payload_coords: HashMap<Uuid, (uint, uint)>,
    priv portal_coords: HashMap<Uuid, (uint, uint)>
}

impl<TZonePayload, TTilePayload: Send + Payloadable> Zone<TZonePayload, TTilePayload> {
    pub fn new(size: uint, id: Uuid, data: TZonePayload) -> Zone<TZonePayload, TTilePayload> {
        // size must be a power of 2
        let mut z = Zone {
            id: id,
            data: data,
            size: size,
            all_tiles: Vec::with_capacity(size*size),
            payload_coords: HashMap::new(),
            portal_coords: HashMap::new()
        };
        let limit = size*size;
        let mut ctr = 0;
        while ctr < limit {
            let tile: Tile<TTilePayload> = Tile::<TTilePayload>::stub();
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
    pub fn tile_at_idx<'a>(&'a self, idx: uint) -> &'a Tile<TTilePayload> {
        self.all_tiles.get(idx)
    }
    pub fn tile_at_idx_mut<'a>(&'a mut self, idx: uint) -> &'a mut Tile<TTilePayload> {
        self.all_tiles.get_mut(idx)
    }
    pub fn get_tile<'a>(&'a self, coords: (uint, uint)) -> &'a Tile<TTilePayload> {
        let idx = coords_to_idx(coords, self.size);
        self.tile_at_idx(idx)
    }
    pub fn get_tile_mut<'a>(&'a mut self, coords: (uint, uint)) -> &'a mut Tile<TTilePayload> {
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
