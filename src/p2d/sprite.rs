use std::path::Path;

pub struct SpriteSheet {
    path: Path,
    name: ~str
}

pub struct SpriteTile {
    sheet: ~str,
    coords: (uint, uint),
    size: (uint, uint)
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