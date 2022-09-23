# Wave Function Collapse Demo

## Dan's update note on Sep 22
- With things in their own file and working
- Rulegen logic is about to be working
    - currently takes a WorldMap to work with
    - should reduce that to an using an array 
    - still need to work on reading from other sources 
- I think our new main function should be like:
```rs
fn main{
    /// 1. read_file 
    /// file contains world size of the input minimap
    /// 2. generate_rules
    /// 3. pipe the rules into tiles
    /// 4. generate the world with "knowledgeable" tiles
    /// 5. while loop that does the collapse
    /// our current collapse efficiency is still 1/10 ish
    ///        /\
    /// awaiting further discussion
}
```
Demonstration of basic use of wave function collapse for explanatory purposes.