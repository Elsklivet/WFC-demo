use std::collections::HashSet;
use std::cmp::Ordering;

use crate::TileKind::TileKind;


#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Clone)]
#[allow(non_camel_case_types)]
pub struct Tile {
    pub coords: (usize, usize),
    pub content: char,
    //tile only has a type once it has been fully collapsed
    pub kind: Option<TileKind>,
    pub options: HashSet<TileKind>,
}

impl Tile{

    pub fn new(coords: (usize, usize), world_width: usize, world_height: usize) -> Tile {
        let mut options = HashSet::new();
        options.insert(TileKind::HORIZ_BAR);
        options.insert(TileKind::VERT_BAR);
        options.insert(TileKind::DOWN_RIGHT);
        options.insert(TileKind::DOWN_LEFT);
        options.insert(TileKind::UP_LEFT);
        options.insert(TileKind::UP_RIGHT);
        options.insert(TileKind::VERT_RIGHT);
        options.insert(TileKind::VERT_LEFT);
        options.insert(TileKind::HORIZ_DOWN);
        options.insert(TileKind::HORIZ_UP);
        options.insert(TileKind::CROSS);
        options.insert(TileKind::DOWN_LEFT_ROUND);
        options.insert(TileKind::DOWN_RIGHT_ROUND);
        options.insert(TileKind::UP_LEFT_ROUND);
        options.insert(TileKind::UP_RIGHT_ROUND);
        
        return Tile {
            coords,
            content: 'X',
            kind: None,
            options,
        };
    }

    pub fn entropy(&self) -> usize {
        if self.kind.is_some() {
            return usize::MAX;
        } else {
            return self.options.len();
        }
    }

}