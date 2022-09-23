mod rulegen;
mod world;
mod tile;
use rand::prelude::*;
use tile::{Tile, TileKind, get_compatibility};
use world::WorldMap;
use std::time::SystemTime;
// use rulegen::generate_rules;

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

/// Find the min entropy tile
fn min_entropy(world: &WorldMap) -> usize {
    // loop over the world
    let mut min_entropy = usize::MAX;
    for row in world.world.iter() {
        for tile in row {
            if tile.entropy < min_entropy && !tile.collapsed {
                min_entropy = tile.entropy;
            }
        }
    }
    min_entropy
    // match world.world.iter().flat_map(|row| row.iter()).min() {
    //     Some(tile) => tile.entropy,
    //     None => usize::MAX,
    // }
}

fn min_tiles(world: &WorldMap) -> Vec<&Tile> {
    // Get lowest entropy value
    let entropy = min_entropy(world);
    // println!("Entropy: {}", entropy);
    // ^ This line says entrypy 0, that should not happen.
    let _choices: Vec<&Tile> = Vec::new();

    // Check if it returned a valid value or not
    if entropy == usize::MAX {
        // No minimum was found, return every single tile
        world.world.iter().flat_map(|row| row.iter()).collect()
    } else {
        world.world.iter().flat_map(|row| row.iter()).filter(|tile| tile.entropy == entropy).collect()
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
    let this = &mut world.world[y][x];
    let other_comp = get_compatibility(kind);
    
    this.choices = this.choices.intersection(&other_comp).map(|kind| kind.clone()).collect();
    // println!("collapsed to: {:?}\nnew compatibility: {:?}", kind, this.compatibility);
}

/// Collapse a point
fn collapse(x: usize, y: usize, world: &mut WorldMap) {
    let this = &mut world.world[y][x];

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
    let width = 80;
    let height = 45;
    let mut world = WorldMap::new(width, height);

    // Start collapsing
    // Choose random spot to start
    let mut collapsed = 0;

    let start = SystemTime::now();
    while collapsed < width * height {
        // Get lowest entropy and collapse on that point
        let collapse_options = min_tiles(&world);
        // println!("Collapse options: {:?}", collapse_options);
        let rand_index = thread_rng().gen_range(0..collapse_options.len());
        // println!("Collapse_options len: {}", collapse_options.len());
        // println!("Random index: {}", rand_index);
        let to_collapse = collapse_options[rand_index];
        // println!("To collapse: {}, {}", to_collapse.x, to_collapse.y);

        // println!("collapsing tile at {}, {}, this is the {} collapse", to_collapse.y, to_collapse.x, collapsed);
        collapse(to_collapse.x, to_collapse.y, &mut world);
        collapsed += 1;
    }

    // for row in world.clone() {
    //     for tile in row {
    //         collapse(tile.x, tile.y, &mut world);
    //     }
    // }
    
    world.render_map();
    let end = SystemTime::now();
    let duration = end.duration_since(start).unwrap();
    println!("Generated a {} * {} map, it took {} seconds", width, height, duration.as_secs());

}
