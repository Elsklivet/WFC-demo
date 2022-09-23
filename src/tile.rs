use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use rand::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum TileKind {
    Land,
    Coast,
    Sea,
    Void, // This is just to represent "we don't have a tile here yet"
}

/// Tile structure
#[derive(Clone, Debug)]
pub struct Tile {
    /// Type of tile this tile is
    pub kind: TileKind,
    /// The (omnidirectional) set of possibilities for this tile's neighbors
    pub compatibility: HashSet<TileKind>, // L, C, S, (V?)
    /// Set of choices for superposition states<br>
    /// Modified by our neighbors in collapse to shrink our
    /// potential states to ones that fit their compatibilities.
    pub choices: HashSet<TileKind>, // L, C, S, (V?)
    /// The entropy (number of possible states) of this tile
    /// At any given time, must be equal to choices.len()
    pub entropy: usize,
    /// Has this tile been collapsed into a single state
    pub collapsed: bool,
    /// Horizontal coordinate of the tile, left to right
    pub x: usize,
    /// Vertical coordinate of the tile, top to bottom
    pub y: usize,
    /// Width of the world this tile is in
    pub world_width: usize,
    /// Height of the world this tile is in
    pub world_height: usize,
}

/// Get the initial compatibility vector ("choices") for a kind of Tile.
pub fn get_compatibility(kind: TileKind) -> HashSet<TileKind> {
    let mut compatibility: HashSet<TileKind> = HashSet::new();

    // We are generating the hashset AFTER we know what kind of tile this is
    // which in reality every tile starts as a void tile
    // so we collapse, get the compatibility, and then propagate
    match kind {
        TileKind::Land => {
            compatibility.insert(TileKind::Land);
            compatibility.insert(TileKind::Coast);
        },
        TileKind::Coast => {
            compatibility.insert(TileKind::Land);
            compatibility.insert(TileKind::Coast);
            compatibility.insert(TileKind::Sea);
        },
        TileKind::Sea => {
            compatibility.insert(TileKind::Coast);
            compatibility.insert(TileKind::Sea);
        },
        TileKind::Void => {
            compatibility.insert(TileKind::Land);
            compatibility.insert(TileKind::Coast);
            compatibility.insert(TileKind::Sea);
        }
    }

    compatibility
}

pub fn default_choices() -> HashSet<TileKind> {
    HashSet::from([TileKind::Sea, TileKind::Land, TileKind::Coast])
}

impl PartialOrd for Tile {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.entropy == other.entropy
    }
}

impl Eq for Tile {
}

impl Ord for Tile {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entropy.cmp(&other.entropy)
    }
}

impl Tile {
    /// Generate a new Tile
    pub fn new(x: usize, y: usize, kind: TileKind, world_width: usize, world_height: usize) -> Tile {
        let compatibility = get_compatibility(kind);
        let choices = default_choices();
        let entropy = choices.len();
        // let entropy = if kind != TileKind::Void {
        //     choices.len()
        // }
        // else {
        //     usize::MAX
        // };

        Tile {
            kind,
            compatibility,
            choices,
            entropy,
            collapsed: false,
            x,
            y,
            world_width,
            world_height,
        }
    }

    /// Modify the compatibility matrix for the Tile based on the kind of tile
    /// that was just placed.
    /// lets not worry about this for now
    // fn mod_compatibility(&mut self, just_placed_kind: TileKind) {
    //     match &self.kind {
    //         TileKind::Land => {
    //             if just_placed_kind == TileKind::Coast {
    //                 self.compatibility.retain(|kind| *kind != TileKind::Coast);
    //             }
    //         },
    //         TileKind::Coast => {
    //             if just_placed_kind == TileKind::Sea {
    //                 self.compatibility.retain(|kind| *kind != TileKind::Sea);
    //             }
    //         },
    //         TileKind::Sea => {
    //             if just_placed_kind == TileKind::Coast {
    //                 self.compatibility.retain(|kind| *kind != TileKind::Coast);
    //             }
    //         }, 
    //         _ => {}
    //     }
    // }
    // Get coordinates north of me
    pub fn find_north(&self) -> Option<(usize, usize)> {
        if self.y == 0 {
            None
        } else {
            Some((self.x, self.y-1))
        }
    }
    /// Get coordinates south of me
    pub fn find_south(&self) -> Option<(usize, usize)> {
        if self.y == self.world_height - 1 {
            None
        } else {
            Some((self.x, self.y+1))
        }
    }
    /// Get coordinates west of me
    pub fn find_west(&self) -> Option<(usize, usize)> {
        if self.x == 0 {
            None
        } else {
            Some((self.x-1, self.y))
        }
    }
    /// Get coordinates east of me
    pub fn find_east(&self) -> Option<(usize, usize)> {
        if self.x == self.world_width - 1 { 
            None
        } else {
            Some((self.x+1, self.y))
        }
    }
    /// Collapse this tile into a single state.
    /// 
    /// **Warning:** Function assumes that the current selection of `choices` is correct. 
    /// # Steps
    /// 1. Randomly select from `choices` a TileKind for this tile
    /// 2. Mark this tile as collapsed
    /// 3. Modify our own compatibility set based on our new `kind`
    /// 4. Report the TileKind selected back to the iterative `collapse` function to propagate to 
    /// our neighbors
    /// 
    /// ## Returns
    /// - `TileKind` this tile was collapsed to
    pub fn collapse_self(&mut self) -> TileKind  {
        // Step 1
        let choice = thread_rng().gen_range(0..self.choices.len());
        let state = self.choices.iter().nth(choice).unwrap_or(&TileKind::Void).clone();
        self.kind = state;

        // Step 2
        self.collapsed = true;

        // Step 3
        // self.compatibility = get_compatibility(self.kind);
        self.entropy = usize::MAX;

        // Step 4
        self.kind
    }

}
