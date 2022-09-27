use std::collections::HashSet;
use std::collections::HashMap;


pub type RuleSet = HashMap<usize, HashMap<usize, HashSet<TileKind>>>;

pub struct TileKind{
    pub id: usize,
    pub content: String,
    pub rule: RuleSet,
}