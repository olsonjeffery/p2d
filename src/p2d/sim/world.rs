use std::libc::{c_int};
use std::libc;
use std::cmp::{Eq, TotalEq};
use std::to_bytes::{Cb, IterBytes};
use std::hashmap::{HashMap, HashSet};
use super::zone::{Zone, ZoneTraversalResult, EntityMoved};
use super::agent::Agent;
use super::portal::Portal;
use super::fov;

#[deriving(Eq)]
#[deriving(IterBytes)]
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

pub struct World {
    zones: HashMap<uint, Zone>,
    agents: HashMap<uint, Agent>,
    portals: HashMap<uint, Portal>,
    latest_agent_id: uint,
    latest_zone_id: uint,
    latest_portal_id: uint,
    starting_agent: uint
}

#[deriving(IterBytes)]
pub struct GlobalCoord {
    zone_id: uint,
    lx: uint,
    ly: uint,
    gx: int,
    gy: int
}

impl GlobalCoord {
    pub fn new(zone_id: uint, coords: (uint, uint), g_coords: (int, int)) -> GlobalCoord {
        let (x, y) = coords;
        let (gx, gy) = g_coords;
        GlobalCoord {
            zone_id: zone_id,
            lx: x,
            ly: y,
            gx: gx,
            gy: gy
        }
    }
}
impl Eq for GlobalCoord {
    fn eq(&self, other: &GlobalCoord) -> bool {
        return self.zone_id == other.zone_id
            && self.gx == other.gx
            && self.gy == other.gy
            && self.lx == other.lx
            && self.ly == other.ly
    }
}
impl TotalEq for GlobalCoord {
    fn equals(&self, other: &GlobalCoord) -> bool {
        return self.zone_id == other.zone_id
            && self.gx == other.gx
            && self.gy == other.gy
            && self.lx == other.lx
            && self.ly == other.ly
    }
}

impl World {
    pub fn new() -> World {
        let mut w = World {
            zones: HashMap::new(),
            agents: HashMap::new(),
            portals: HashMap::new(),
            latest_agent_id: 0,
            latest_zone_id: 0,
            latest_portal_id: 0,
            starting_agent: 0
        };
        w
    }

    // Entity creation
    pub fn new_agent(&mut self, zone_id: uint, coords: (uint, uint), cb: |&mut Agent|) -> uint {
        let next_id = self.latest_agent_id + 1;
        let mut agent = Agent::stub();
        agent.id = next_id;
        self.latest_agent_id = next_id;
        self.agents.find_or_insert(next_id, agent);
        {
            let new_agent = self.agents.find_mut(&next_id).expect(format!("Couldn't get newly added agent {:u}", next_id));
            new_agent.zone_id = zone_id;
            cb(new_agent);
        }
        let mut zone = self.get_zone_mut(&zone_id);
        let move_result = zone.move_agent(next_id, coords);
        match move_result {
            EntityMoved => {},
             _ => fail!("Agent wasn't moved into zone! Result: {:?}", move_result)
        }
        next_id
    }

    pub fn new_zone(&mut self, size: uint, cb: |&mut Zone|) -> uint {
        let next_id = self.latest_zone_id + 1;
        let mut z = Zone::new(size, next_id);
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
    pub fn get_agent<'a>(&'a self, id: &uint) -> &'a Agent {
        self.agents.find(id).expect(format!("Cannot find agent with id {:?}", id))
    }
    pub fn get_agent_mut<'a>(&'a mut self, id: &uint) -> &'a mut Agent {
        self.agents.find_mut(id).expect(format!("Cannot find_mut agent with id {:?}", id))
    }
    pub fn get_zone<'a>(&'a self, id: uint) -> &'a Zone {
        self.zones.find(&id).expect(format!("Cannot find zone with id {:?}", id))

    }
    pub fn get_zone_mut<'a>(&'a mut self, id: &uint) -> &'a mut Zone {
        self.zones.find_mut(id).expect(format!("Cannot find_mut zone with id {:?}", id))
    }

    pub fn get_portal<'a>(&'a self, id: uint) -> &'a Portal {
        self.portals.find(&id).expect(format!("Cannot find portal with id {:?}", id))

    }

    // Entity Traversal
    pub fn traverse_agent(&mut self, aid: uint, dir: TraversalDirection) -> ZoneTraversalResult {
        let delta = match dir {
            North => (0, -1),
            West => (-1, 0),
            South => (0, 1),
            East => (1, 0),
            NoDirection => fail!("NoDirection not allowed in traverse_agent()")
        };
        let curr_zone_id = {
            let agent = self.get_agent(&aid);
            let curr_zone = self.get_zone(agent.zone_id);
            curr_zone.id
        };
        let (dest_zone_id, dest_coords) = {
            let agent = self.get_agent(&aid);
            let curr_zone = self.get_zone(agent.zone_id);
            let (d_x, d_y) = delta;
            let curr_coords = *curr_zone.get_agent_coords(&aid);
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
                let other_zone = self.get_zone(ozid);
                let (ocx, ocy) = *other_zone.get_portal_coords(&pid);
                let oc = match td {
                    North => (ocx, ocy-1),
                    East => (ocx+1, ocy),
                    South => (ocx, ocy+1),
                    West => (ocx-1, ocy),
                    NoDirection => fail!("NoDirection not allowed in traverse_agent()")
                };
                println!("other zone: {:?}, this zone: {:?}", ozid, curr_zone_id);
                (ozid, oc)
            } else {
                let dest_coords = (curr_x + d_x, curr_y + d_y);
                (curr_zone_id, dest_coords)
            }
        };
        if dest_zone_id != curr_zone_id {
            let old_zone = self.get_zone_mut(&curr_zone_id);
            old_zone.remove_agent(aid);
        }
        self.move_agent(aid, dest_zone_id, dest_coords)
    }
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
}