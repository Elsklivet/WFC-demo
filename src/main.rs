// mod Tile;
// mod TileKind;
// mod rulegen;
// mod WorldMap;
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

    // println!("{:?}", contents);
    // go into the string once to give each unique char an id
    let mut charmap = HashMap::new();
    let mut id = 1; // 0 is reserved for superposition
    for c in contents.chars() {
        if !charmap.contains_key(&c) {
            charmap.insert(c, id);
            id += 1;
        }
    }
    println!("{:?}", charmap);

    // now represent the input as numbers
    for i in 0..10{
        for j in 0..10{
            let c = contents.chars().nth(i*10+j).unwrap();
            print!("{:?}-", charmap.get(&c).unwrap());
        }
        println!("");
    }

    // make a 2d int array
    let mut map = [[0; 10]; 10];
    for i in 0..10{
        for j in 0..10{
            let c = contents.chars().nth(i*10+j).unwrap();
            map[i][j] = *charmap.get(&c).unwrap();
        }
    }
    // println!("{:?}", map);

    // dir enum
    #[derive(Debug, PartialEq, Eq, Hash, Clone)]
    enum Dir {
        WEST,
        NORTH,
        EAST,
        SOUTH,
    }

    // go into the int array and make a rule set
    let mut rules: HashMap<usize, HashMap<Dir, HashSet<usize>>> = HashMap::new();
    for i in 0..10{
        for j in 0..10{
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
    for i in 0..10{
        for j in 0..10{
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

    println!("{:?}", world);






}


fn propagate(){

}


fn collapse(){

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

        // Step 3
        // self.compatibility = get_compatibility(self.kind);

        // Step 4
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

