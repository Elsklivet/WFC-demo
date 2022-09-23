// def generate_rules(world, N):
//     rule = {}
//     for i in range(len(world.contents)):
//         for j in range(len(world.contents[i])):
//             this_content = world.contents[i][j]
//             if world.contents[i][j] not in rule:
//                 rule[this_content] = {}
//                 for direction in range(4):
//                     rule[this_content][direction] = []

//     for i in range(len(world.contents)):
//         for j in range(len(world.contents[i])):
//             this_content = world.contents[i][j]
//             if i > 0: # left
//                 rule[this_content][0].append(world.contents[i-1][j])
//                 rule[this_content][0] = list(set(rule[this_content][0]))
//             if i < world.width-1: # right
//                 rule[this_content][1].append(world.contents[i+1][j])
//                 rule[this_content][1] = list(set(rule[this_content][1]))
//             if j > 0: # up
//                 rule[this_content][2].append(world.contents[i][j-1])
//                 rule[this_content][2] = list(set(rule[this_content][2]))
//             if j < world.height-1: # down
//                 rule[this_content][3].append(world.contents[i][j+1])
//                 rule[this_content][3] = list(set(rule[this_content][3]))
//     return rule
//
// {END OF PYTHON SNIPPET}

use std::collections::HashMap;

/// world: a 2d array of TileKind
/// n: the window size
pub fn generate_rules(world: WorldMap, n: usize) -> HashMap<TileKind, HashMap<usize, Vec<TileKind>>> {
    let mut rule = HashMap::new();
    for i in 0..world.height {
        for j in 0..world.width {
            let this_kind = world.world[i][j].kind;
            if !rule.contains_key(&this_kind) {
                rule.insert(this_kind, HashMap::new());
                for direction in 0..4 {
                    rule.get_mut(&this_kind).unwrap().insert(direction, Vec::new());
                }
            }
        }
    }

    // WARNING: pay attention to the x y directions and loop orders if things go wrong
    for i in 0..world.height {
        for j in 0..world.width {
            let this_kind = world.world[i][j].kind;
            if i > 0 { // left
                rule.get_mut(&this_kind).unwrap().get_mut(&0).unwrap().push(world.world[i-1][j].kind);
                rule.get_mut(&this_kind).unwrap().get_mut(&0).unwrap().sort();
                rule.get_mut(&this_kind).unwrap().get_mut(&0).unwrap().dedup();
            }
            if i < world.width-1 { // right
                rule.get_mut(&this_kind).unwrap().get_mut(&1).unwrap().push(world.world[i+1][j].kind);
                rule.get_mut(&this_kind).unwrap().get_mut(&1).unwrap().sort();
                rule.get_mut(&this_kind).unwrap().get_mut(&1).unwrap().dedup();
            }
            if j > 0 { // up
                rule.get_mut(&this_kind).unwrap().get_mut(&2).unwrap().push(world.world[i][j-1].kind);
                rule.get_mut(&this_kind).unwrap().get_mut(&2).unwrap().sort();
                rule.get_mut(&this_kind).unwrap().get_mut(&2).unwrap().dedup();
            }
            if j < world.height-1 { // down
                rule.get_mut(&this_kind).unwrap().get_mut(&3).unwrap().push(world.world[i][j+1].kind);
                rule.get_mut(&this_kind).unwrap().get_mut(&3).unwrap().sort();
                rule.get_mut(&this_kind).unwrap().get_mut(&3).unwrap().dedup();
            }
        }
    }
    rule
}