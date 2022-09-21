use rand::prelude::*;
use rand::random;
use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;

// wikihow to backtrack
// void traverse(decision, partial_solution):
//   for each choice:
//     if valid choice:
//       apply choice to partial_solution
//     if more decisions:
//       traverse(next decision, updated partial_solution)
//     else:
//       candidate solution
//     undo choice



/// Kind of tile.
/// 
/// - Land can be placed next to land or coast, but coast only once
/// - Coast can be placed next to land, coast, or sea, but sea only once
/// - Sea can be placed next to sea or coast, but coast only once
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum TileKind {
    Land,
    Coast,
    Sea,
    Void, // This is just to represent "we don't have a tile here"
}

/// Tile structure
#[derive(Clone)]
struct Tile {
    kind: TileKind,
    compatibility: HashSet<TileKind>, // N W E S 
    entropy: usize,
    collapsed: bool,
    x: usize,
    y: usize,
}

/// Get the initial compatibility vector ("choices") for a kind of Tile.
fn get_compatibility(kind: TileKind) -> HashSet<TileKind> {
    let mut compatibility: HashSet<TileKind> = HashSet::new();

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
    fn new(x: usize, y: usize, kind: TileKind) -> Tile {
        let compatibility = get_compatibility(kind);
        let entropy = compatibility.len();

        Tile {
            kind,
            compatibility,
            entropy,
            collapsed: false,
            x,
            y,
        }
    }

    /// Modify the compatibility matrix for the Tile based on the kind of tile
    /// that was just placed.
    fn mod_compatibility(&mut self, just_placed_kind: TileKind) {
        match &self.kind {
            TileKind::Land => {
                match just_placed_kind {
                    TileKind::Coast => self.compatibility.retain(|kind| *kind != TileKind::Coast),
                    _ => {}
                }
            },
            TileKind::Coast => {
                match just_placed_kind {
                    TileKind::Sea => self.compatibility.retain(|kind| *kind != TileKind::Sea),
                    _ => {}
                }
            },
            TileKind::Sea => {
                match just_placed_kind {
                    TileKind::Coast => self.compatibility.retain(|kind| *kind != TileKind::Coast),
                    _ => {}
                }
            }, 
            _ => {}
        }
    }

    fn find_north(&self) -> Option<(usize, usize)> {
        if self.y == 0 {
            None
        } else {
            Some((self.x, self.y-1))
        }
    }

    fn find_south(&self) -> Option<(usize, usize)> {
        if self.y == 7 {
            None
        } else {
            Some((self.x, self.y+1))
        }
    }

    fn find_west(&self) -> Option<(usize, usize)> {
        if self.x == 0 {
            None
        } else {
            Some((self.x-1, self.y))
        }
    }

    fn find_east(&self) -> Option<(usize, usize)> {
        if self.x == 15 {
            None
        } else {
            Some((self.x+1, self.y))
        }
    }

}

type WorldMap = Vec<Vec<Tile>>;

/// Simply print the world out as it currently stands.
fn render_map(world: WorldMap) {
    for row in world {
        for tile in row {
            match tile.kind {
                TileKind::Land => print!("L"),
                TileKind::Coast => print!("C"),
                TileKind::Sea => print!("S"),
                TileKind::Void => print!("X"),
            }
        }
        println!();
    }
}

/// Find the min entropy tile
fn min_entropy(world: &WorldMap) -> Option<(usize, usize)> {
    let mut row_lows: Vec<Tile> = vec![];
    for row in world {
        let low = row.iter().min();
        if let Some(tile) = low {
            row_lows.push(tile.clone());
        } else {
            // Something seriously wrong
            return None;
        }
    }

    if let Some(tile) = row_lows.iter().min() {
        Some((tile.x, tile.y))
    } else {
        None
    }
}

/// Collapse a point
fn collapse(x: usize, y: usize, world: &mut WorldMap) {
    let this = &mut world[x][y];

    // We can get the tiles in every direction from the current tile
    // by using the `find` functions. Then we want to take all of the compatibility
    // sets of those tiles and intersect them. What results would be the valid possible options for 
    // this specific point. We then pick one of those options at random, set this tile's Kind to that
    // choice, reset its compatibility to the default for its kind, set its entropy to max (so it is not
    // collapsed again) and set its collapsed value.
    // At that point we can return and loop should move forward.

}

fn main() {
    // World will be a 8x16 vector of vectors of tiles
    let mut world: WorldMap = Vec::new();

    // Create rows
    for i in 0..8usize {
        world.push(Vec::new());

        for j in 0..16usize {
            world[i].push(Tile::new(j,i, TileKind::Void));
        }
    }

    // Start collapsing
    // Choose random spot to start
    let mut collapsed = 0;

    while collapsed < 128 {
        // Get lowest entropy and collapse on that point
        if let Some(lowest_coord) = min_entropy(&world) {
            collapse(lowest_coord.0, lowest_coord.1, &mut world);
            collapsed += 1;
        } else {
            println!("Something went wrong finding minimum entropy.");
            break;
        }
    }
    
    render_map(world);
}
