use std::env;
use std::{fs::File, io::Read};
use std::collections::{HashMap, HashSet};
use rand::prelude::*;

fn main() {
    // read command args
    // generate rules
    // feed rules to tiles
    // generate map
    // print map
    // while not complete
    // collapse
    // print map

    let args: Vec<String> = env::args().collect();

    println!("{args:?}");

    //let mut file = File::open("chars.txt").unwrap();
    let mut file = File::open(args[3].clone()).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    println!("content.len() = {}", contents.len());

    let input_board: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect();
    println!("input_board.len() = {}", input_board.len());
    println!("input_board[0].len() = {}", input_board[0].len());
    let input_width = input_board[0].len();
    let input_height = input_board.len();

    // println!("{:?}", contents);
    // go into the string once to give each unique char an id
    let mut charmap = HashMap::new();
    let mut id = 1; // 0 is reserved for superposition
    for c in input_board.iter().flat_map(|l| l.iter()) {
        if !charmap.contains_key(&c) {
            charmap.insert(c, id);
            id += 1;
        }
    }
    // manually add the superposition char
    charmap.insert(&'X', 0);
    println!("{:?}", charmap);

    // make a lookup table for the charmap
    let mut charmap_lookup = HashMap::new();
    for (k, v) in charmap.iter() {
        charmap_lookup.insert(v, k);
    }
    // mannually add the superposition char
    charmap_lookup.insert(&0, &&'X');
    println!("{:?}", charmap_lookup);

    // now represent the input as numbers
    for i in 0..input_board.len() {
        for j in 0..input_board[i].len() {
            let c = input_board[i][j];
            print!("{:?}-", charmap.get(&c).unwrap());
        }
        println!("");
    }

    // make a 2d int array
    let mut map = vec![vec![0; input_width]; input_height];
    for i in 0..input_board.len() {
        for j in 0..input_board[i].len() {
            let c = input_board[i][j];
            map[i][j] = *charmap.get(&c).unwrap();
        }
    }
    // println!("{:?}", map);

    // go into the int array and make a rule set
    let mut rules: HashMap<usize, HashMap<Dir, HashSet<usize>>> = HashMap::new();
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            rules 
            .entry(map[i][j])
            .or_insert(HashMap::from([
                (Dir::WEST, HashSet::new()),
                (Dir::NORTH, HashSet::new()),
                (Dir::EAST, HashSet::new()),
                (Dir::SOUTH, HashSet::new()),
            ]));
        }
    }
    // println!("{:?}", rules);

    // now go through the map and fill in the rules
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            // North
            if i > 0{
                rules.get_mut(&map[i][j]).unwrap().get_mut(&Dir::NORTH).unwrap().insert(map[i-1][j]);
            }
            // South
            if i < 9{
                rules.get_mut(&map[i][j]).unwrap().get_mut(&Dir::SOUTH).unwrap().insert(map[i+1][j]);
            }
            // West
            if j > 0{
                rules.get_mut(&map[i][j]).unwrap().get_mut(&Dir::WEST).unwrap().insert(map[i][j-1]);
            }
            // East
            if j < 9{
                rules.get_mut(&map[i][j]).unwrap().get_mut(&Dir::EAST).unwrap().insert(map[i][j+1]);
            }
        }
    }
    // println!("{:?}", rules.entry(0));
    let world_width = args[1].parse::<usize>().unwrap();
    let world_height = args[2].parse::<usize>().unwrap();

    let mut world: Vec<Vec<Tile>> = Vec::new();
    for i in 0..world_height{
        let mut row: Vec<Tile> = Vec::new();
        for j in 0..world_width{
            row.push(Tile::new(i, j, world_width, world_height));
        }
        world.push(row);
    }

    // println!("{:?}", world);

    // collapse the world
    let mut collapsed = 0;
    while collapsed < world_width * world_height{
        // Get lowest entropy and collapse on that point
        let collapse_options = min_tiles(&world);
        // println!("Collapse options: {:?}", collapse_options);
        let rand_index = thread_rng().gen_range(0..collapse_options.len());
        // println!("Collapse_options len: {}", collapse_options.len());
        // println!("Random index: {}", rand_index);
        let to_collapse = collapse_options[rand_index];
        println!("main loop chose this to collapse: {}, {}", to_collapse.x, to_collapse.y);

        // println!("collapsing tile at {}, {}, this is the {} collapse", to_collapse.y, to_collapse.x, collapsed);
        collapse(to_collapse.x, to_collapse.y, &mut world, &rules);
        collapsed += 1;
    }

    // print the world
    for i in 0..world_height{
        for j in 0..world_width{
            print!("{:}", charmap_lookup.get(&world[i][j].kind).unwrap());
        }
        println!("");
    }




}

    // dir enum
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
enum Dir {
    WEST,
    NORTH,
    EAST,
    SOUTH,
}

/// Find the min entropy tile
fn min_entropy(world: &Vec<Vec<Tile>>) -> usize {
    // loop over the world
    let mut min_entropy = usize::MAX;
    for row in world.iter() {
        for tile in row {
            if tile.entropy() < min_entropy && !tile.collapsed {
                min_entropy = tile.entropy();
            }
        }
    }

    println!("min entropy: {}", min_entropy);

    min_entropy
}

fn min_tiles(world: &Vec<Vec<Tile>>) -> Vec<&Tile> {
    // Get lowest entropy value
    let entropy = min_entropy(world);
    // println!("Entropy: {}", entropy);
    // ^ This line says entrypy 0, that should not happen.
    let _choices: Vec<&Tile> = Vec::new();

    // Check if it returned a valid value or not
    if entropy == usize::MAX {
        // No minimum was found, return every single tile
        world.iter().flat_map(|row| row.iter()).collect()
    } else {
        let choices = world.iter().flat_map(|row| row.iter()).filter(|tile| tile.entropy() == entropy).collect();
        println!("Choices: {:?}", choices);
        choices
    }
}

// before we call propagate
// we already know which neighbor we are prpagating to
// so we index into the rule set
// and use that specific rule set
fn propagate(x: usize, y: usize, rule: &HashSet<usize>, world: &mut Vec<Vec<Tile>>){

    let this = &mut world[y][x];
    println!("this's original choices: {:?}", this.choices);
    let other_comp = rule;
    
    this.choices = this.choices.intersection(&other_comp).map(|kind| kind.clone()).collect();
    println!("Propagating to {}, {}, choices: {:?}", x, y, this.choices);

}


fn collapse(x: usize, y: usize, world: &mut Vec<Vec<Tile>>, 
    rule: &HashMap<usize, HashMap<Dir, HashSet<usize>>>){

    // print intermediate world
    // for i in 0..world.len(){
    //     for j in 0..world[0].len(){
    //         print!("{:}", world[i][j].kind);
    //     }
    //     println!("");
    // }

    let this = &mut world[y][x];

    if this.collapsed {
        return;
    }

    let this_kind = this.collapse_self();

    println!("collapsing tile at {}, {} to kind {:?}", y, x, this_kind);

    let this_rule = rule.get(&this_kind).unwrap();

    println!("this rule: {:?}", this_rule);
    
    // Get the tiles in every direction
    let wrapped_east = this.find_east();
    let wrapped_west = this.find_west();
    let wrapped_north = this.find_north();
    let wrapped_south = this.find_south();
    
    if let Some(east) = wrapped_east {
        let east_rule = this_rule.get(&Dir::EAST).unwrap();
        println!("East:");
        propagate(east.0, east.1, east_rule, world);
    }
    if let Some(west) = wrapped_west {
        let west_rule = this_rule.get(&Dir::WEST).unwrap();
        println!("West:");
        propagate(west.0, west.1, west_rule, world);
    }
    if let Some(north) = wrapped_north{
        let north_rule = this_rule.get(&Dir::NORTH).unwrap();
        println!("North:");
        propagate(north.0, north.1, north_rule, world);
    }
    if let Some(south) = wrapped_south{
        let south_rule = this_rule.get(&Dir::SOUTH).unwrap();
        println!("South:");
        propagate(south.0, south.1, south_rule, world);
    }

}

fn default_choices() -> HashSet<usize>{
    let mut choices = HashSet::new();
    for i in 1..=16{
        choices.insert(i);
    }
    choices
}

#[derive(Debug)]
struct Tile{
    x: usize,
    y: usize,
    kind: usize,
    // rule is global to tile
    // just index into it after it collapes
    choices: HashSet<usize>,
    world_width: usize,
    world_height: usize,
    collapsed: bool,
}

impl Tile{
    fn new(x: usize, y: usize, world_width: usize, world_height: usize) -> Tile{
        Tile{
            x,
            y,
            kind: 0,
            choices: default_choices(),
            world_width,
            world_height,
            collapsed: false,
        }
    }

    fn entropy(&self) -> usize{
        if self.collapsed{
            return usize::MAX;
        }
        self.choices.len()
    }

    fn collapse_self(&mut self) -> usize{
        // Step 1
        let choice = thread_rng().gen_range(0..self.choices.len());
        let state = self.choices.iter().nth(choice).unwrap_or(&0).clone();
        self.kind = state;

        // Step 2
        self.collapsed = true;

        println!("Self-collapsed tile at {}, {} to kind {:?}", self.x, self.y, self.kind);

        // Step 3
        self.kind
    }

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
        if self.y == self.world_height - 1 {
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
        if self.x == self.world_width - 1 { 
            None
        } else {
            Some((self.x+1, self.y))
        }
    }
}