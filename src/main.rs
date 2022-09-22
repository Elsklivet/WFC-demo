

use std::cmp::Ordering;
use std::collections::HashSet;
use std::hash::Hash;
use rand::prelude::*;
use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};



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
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum TileKind {
    Land,
    Coast,
    Sea,
    Void, // This is just to represent "we don't have a tile here yet"
}

/// Tile structure
#[derive(Clone)]
struct Tile {
    /// Type of tile this tile is
    kind: TileKind,
    /// The (omnidirectional) set of possibilities for this tile's neighbors
    compatibility: HashSet<TileKind>, // L, C, S, (V?)
    /// Set of choices for superposition states<br>
    /// Modified by our neighbors in collapse to shrink our
    /// potential states to ones that fit their compatibilities.
    choices: HashSet<TileKind>, // L, C, S, (V?)
    /// The entropy (number of possible states) of this tile
    /// At any given time, must be equal to choices.len()
    entropy: usize,
    /// Has this tile been collapsed into a single state
    collapsed: bool,
    /// Horizontal coordinate of the tile, left to right
    x: usize,
    /// Vertical coordinate of the tile, top to bottom
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

fn default_choices() -> HashSet<TileKind> {
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
    fn new(x: usize, y: usize, kind: TileKind) -> Tile {
        let compatibility = get_compatibility(kind);
        let choices = default_choices();
        let entropy = if kind != TileKind::Void {
            choices.len()
        } else {
            0
        };

        Tile {
            kind,
            compatibility,
            choices,
            entropy,
            collapsed: false,
            x,
            y,
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
    fn find_north(&self) -> Option<(usize, usize)> {
        if self.y == 0 {
            None
        } else {
            Some((self.x, self.y-1))
        }
    }
    /// Get coordinates south of me
    fn find_south(&self) -> Option<(usize, usize)> {
        if self.y == 3 {
            None
        } else {
            Some((self.x, self.y+1))
        }
    }
    /// Get coordinates west of me
    fn find_west(&self) -> Option<(usize, usize)> {
        if self.x == 0 {
            None
        } else {
            Some((self.x-1, self.y))
        }
    }
    /// Get coordinates east of me
    fn find_east(&self) -> Option<(usize, usize)> {
        if self.x == 7 {
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
    fn collapse_self(&mut self) -> TileKind  {
        // Step 1
        let choice = thread_rng().gen_range(0..self.choices.len());
        let state = self.choices.iter().nth(choice).unwrap_or(&TileKind::Void).clone();
        self.kind = state;

        // Step 2
        self.collapsed = true;

        // Step 3
        // self.compatibility = get_compatibility(self.kind);

        // Step 4
        self.kind
    }

}

type WorldMap = Vec<Vec<Tile>>;

/// Simply print the world out as it currently stands.
fn render_map(world: WorldMap) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    for row in world {
        for tile in row {
            match tile.kind {
                TileKind::Land => {
                    match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Green))) {
                        Ok(()) => write!(&mut stdout, "L").expect("ability to write to term"),
                        Err(_) => print!("L"),
                    };
                },
                TileKind::Coast => match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))) {
                    Ok(()) => write!(&mut stdout, "C").expect("ability to write to term"),
                    Err(_) => print!("C"),
                },
                TileKind::Sea => match stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue))) {
                    Ok(()) => write!(&mut stdout, "S").expect("ability to write to term"),
                    Err(_) => print!("S"),
                },
                TileKind::Void => print!("X"),
            }
            stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
        }
        println!();
    }
}

/// Find the min entropy tile
fn min_entropy(world: &WorldMap) -> usize {
    match world.iter().flat_map(|row| row.iter()).min() {
        Some(tile) => tile.entropy,
        None => usize::MAX,
    }
}

fn min_tiles(world: &WorldMap) -> Vec<&Tile> {
    // Get lowest entropy value
    let entropy = min_entropy(world);
    let choices: Vec<&Tile> = Vec::new();

    // Check if it returned a valid value or not
    if entropy == usize::MAX {
        // No minimum was found, return every single tile
        world.iter().flat_map(|row| row.iter()).collect()
    } else {
        world.iter().flat_map(|row| row.iter()).filter(|tile| tile.entropy == entropy).collect()
    }
}

// fn get_possibilities(x: usize, y: usize, world: &WorldMap) -> HashSet<TileKind> {
//    ...
// }

// fn calc_entropy(x: usize, y: usize) -> usize {
//    get_possibilities(x,y).len()
// }

/// Propagate rule changes to neighbors
fn propagate(x: usize, y: usize, kind: TileKind, world: &mut WorldMap) {
    let this = &mut world[y][x];
    let other_comp = get_compatibility(kind);
    
    this.choices = this.choices.intersection(&other_comp).map(|kind| kind.clone()).collect();
    println!("collapsed to: {:?}\nnew compatibility: {:?}", kind, this.compatibility);
}

/// Collapse a point
fn collapse(x: usize, y: usize, world: &mut WorldMap) {
    let this = &mut world[y][x];

    if this.collapsed {
        return;
    }

    // We can get the tiles in every direction from the current tile
    // by using the `find` functions. Then we want to take all of the compatibility
    // sets of those tiles and intersect them. What results would be the valid possible options for 
    // this specific point. We then pick one of those options at random, set this tile's Kind to that
    // choice, reset its compatibility to the default for its kind, set its entropy to max (so it is not
    // collapsed again) and set its collapsed value.
    // At that point we can return and loop should move forward.

    let this_kind = this.collapse_self();
    
    // Get the tiles in every direction
    let wrapped_east = this.find_east();
    let wrapped_west = this.find_west();
    let wrapped_north = this.find_north();
    let wrapped_south = this.find_south();
    
    if let Some(east) = wrapped_east {
        propagate(east.0, east.1, this_kind, world);
    }
    if let Some(west) = wrapped_west {
        propagate(west.0, west.1, this_kind, world);
    }
    if let Some(north) = wrapped_north{
        propagate(north.0, north.1, this_kind, world);
    }
    if let Some(south) = wrapped_south{
        propagate(south.0, south.1, this_kind, world);
    }



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
    for i in 0..4usize {
        world.push(Vec::new());

        for j in 0..8usize {
            world[i].push(Tile::new(j,i, TileKind::Void));
        }
    }

    // Start collapsing
    // Choose random spot to start
    let mut collapsed = 0;

    while collapsed < 64 {
        // Get lowest entropy and collapse on that point
        let collapse_options = min_tiles(&world);
        let rand_index = thread_rng().gen_range(0..collapse_options.len());
        let to_collapse = collapse_options[rand_index];

        println!("collapsing tile at {}, {}, this is the {} collapse", to_collapse.y, to_collapse.x, collapsed);
        collapse(to_collapse.x, to_collapse.y, &mut world);
        collapsed += 1;
    }

    // for row in world.clone() {
    //     for tile in row {
    //         collapse(tile.x, tile.y, &mut world);
    //     }
    // }
    
    render_map(world);
}
