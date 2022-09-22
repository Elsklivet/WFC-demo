

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
    options: HashSet<TileKind>, // L, C, S, (V?)
    // options is what self can be
    // compatibility is what self can be next to
    // self modifies options of neighbors based on compatibility
    entropy: usize,
    collapsed: bool,
    x: usize,
    y: usize,
}

/// Get the initial compatibility vector ("choices") for a kind of Tile.
fn get_compatibility(kind: TileKind) -> HashSet<TileKind> {
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
        let options = compatibility.clone();

        Tile {
            kind,
            compatibility,
            options,
            entropy,
            collapsed: false,
            x,
            y,
        }
    }

    /// Modify the compatibility matrix for the Tile based on the kind of tile
    /// that was just placed.
    /// lets not worry about this for now
    fn mod_compatibility(&mut self, just_placed_kind: TileKind) {
        match &self.kind {
            TileKind::Land => {
                if just_placed_kind == TileKind::Coast {
                    self.compatibility.retain(|kind| *kind != TileKind::Coast);
                }
            },
            TileKind::Coast => {
                if just_placed_kind == TileKind::Sea {
                    self.compatibility.retain(|kind| *kind != TileKind::Sea);
                }
            },
            TileKind::Sea => {
                if just_placed_kind == TileKind::Coast {
                    self.compatibility.retain(|kind| *kind != TileKind::Coast);
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

    row_lows.iter().min().map(|tile| (tile.x, tile.y))
}

// fn get_possibilities(x: usize, y: usize, world: &WorldMap) -> HashSet<TileKind> {
//    ...
// }

// fn calc_entropy(x: usize, y: usize) -> usize {
//    get_possibilities(x,y).len()
// }

/// Collapse a point
fn collapse(x: usize, y: usize, world: &mut WorldMap) {
    let _this = &mut world[x][y];

    // We can get the tiles in every direction from the current tile
    // by using the `find` functions. Then we want to take all of the compatibility
    // sets of those tiles and intersect them. What results would be the valid possible options for 
    // this specific point. We then pick one of those options at random, set this tile's Kind to that
    // choice, reset its compatibility to the default for its kind, set its entropy to max (so it is not
    // collapsed again) and set its collapsed value.
    // At that point we can return and loop should move forward.
    
    // Get the tiles in every direction
    let _east = _this.find_east();
    let _west = _this.find_west();
    let _north = _this.find_north();
    let _south = _this.find_south();

    // Get the compatibility sets for each tile
    
    // Cannot do this though:
    // choices = choices.intersection(&other.compatibilities).collect()
    //                                                       ^--------- Cannot collect based on &TileKind to &HashSet<TileKind>
    
    // I dk how exactly /\ is failing, maybe try this:
    // choices = choices.intersection(&other.compatibilities).map(|kind| kind.clone()).collect();


    /*
    
    def collapse(x,y):
        global world

        get left.choices, right.choices, up.choices, down.choices
        collapse_choices = intersect this.choices with above 
        if not collapse_choices:
            backtrack
        pick random of collapse_choices
        this.kind = above
        this.compatibility.modify()
    
    def main():
        ...

        while not fully collapsed:
            lowest = get_lowest_entropy()
            collapse(lowest.x, lowest.y)
    
    */

    // SCRATCH THAT, THIS MAKES MORE SENSE \/

    /*

    class Tile:
        kind,
        entropy,
        possible_states,
        adjacency_rules # north south east and west

    def calc_entropy(x,y):
        global world

        return len(world[x][y].possible_states)

    def collapse(x,y):
        global world
        
        check my choices (possible_states)
        pick a choice from my possibilities
        collapse to this choice
        modify my adjacency rules based on collapsed state
        for each neighbor:
            notify neighbor of my state change by propagating my new adjacency rules to modify their own:
                1. entropy
                2. possible_states

    def main():
        while not fully collapsed:
            to_collapse = get_tile_with_lowest_entropy()
            collapse(to_collapse)
    */

    // let choices = get_possibilities(x,y,&world);

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
