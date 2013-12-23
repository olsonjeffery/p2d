use super::sprite::SpriteTile;

pub struct Agent {
    id: uint,
    name: ~str,
    animations: ~[SpriteTile],
    zone_id: uint
}

impl Agent {
    pub fn new(id: uint, name: ~str, animation: ~[SpriteTile], zone_id: uint) -> Agent {
        Agent {
            id: id,
            name: name,
            animations: animation,
            zone_id: zone_id
        }
    }
    pub fn stub() -> Agent {
        Agent::new(0, ~"", ~[SpriteTile::stub()], 0)
    }
}