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
use std::vec::with_capacity;
use std::hashmap::HashMap;
use uuid::Uuid;
use serialize::{Decodable, Encodable};

use super::sprite::SpriteTile;
use super::world::{TraversalDirection, GlobalCoord, Payloadable};

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
    portal_id: Option<uint>
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
    id: uint,
    size: uint,
    all_tiles: ~[Tile<TPayload>],
    priv payload_coords: HashMap<Uuid, (uint, uint)>,
    priv portal_coords: HashMap<uint, (uint, uint)>
}

impl<TPayload: Send + Payloadable> Zone<TPayload> {
    pub fn new(size: uint, id: uint) -> Zone<TPayload> {
        // size must be a power of 2
        let mut z = Zone {
            id: id,
            size: size,
            all_tiles: with_capacity(size*size),
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
        self.payload_coords.find(plid).expect(format!("Unable to find coords for payload {:?}", plid))
    }
    pub fn get_portal_coords<'a>(&'a self, pid: &uint) -> &'a (uint, uint) {
        self.portal_coords.find(pid).expect(format!("Unable to find coords for portal {:?}", pid))
    }
    pub fn coords_in_bounds(&self, coords: (uint, uint)) -> bool {
        let (x, y) = coords;
        return x >= 0 && x < self.size
            && y >= 0 && y < self.size
    }
    ///////////////////////
    // Tile related
    ///////////////////////
    pub fn tile_at_idx<'a>(&'a self, idx: uint) -> &'a Tile<TPayload> {
        &self.all_tiles[idx]
    }
    pub fn tile_at_idx_mut<'a>(&'a mut self, idx: uint) -> &'a mut Tile<TPayload> {
        &mut self.all_tiles[idx]
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
    pub fn add_portal(&mut self, pid: uint, coords: (uint, uint)) {
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
    /*
    pub fn gone_move_payload(&mut self, coords: (uint, uint), plid: Uuid) -> ZoneTraversalResult {
        let in_bounds = self.coords_in_bounds(coords);
        if !in_bounds {
            DestinationOutsideBounds
        } else {
            // this will all move into some kind "traversal predicate" fn
            // that would be passed to this fn when passable, etc moved into
            // the payload.
            {
                let (is_occupied, is_passable) = {
                    let target_tile = self.get_tile(coords);
                    (target_tile.payload_id, target_tile.passable)
                };
                if is_occupied.is_some() {
                    return DestinationOccupied(is_occupied.unwrap());
                }
                if !is_passable {
                    return DestinationBlocked
                }
            }
            // clear their previous position..
            let agent_in_this_zone = self.payload_coords.contains_key(&plid);
            if agent_in_this_zone {
                self.remove_payload(&plid)
            }
            self.payload_coords.insert(plid, coords);
            let target_tile = self.get_tile_mut(coords);
            target_tile.payload_id = Some(plid);
            Destination(GlobalCoord::new(self.id, coords))
        }
    }
    pub fn remove_payload(&mut self, plid: &Uuid) {
        let coords = self.payload_coords.pop(plid).expect("Tried to remove payload from zone, wasn't there!");
        let t = self.get_tile_mut(coords);
        t.payload_id = None;
    }
    */
}
