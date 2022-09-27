use crate::Tile::Tile;
use crate::TileKind::TileKind;

pub struct WorldMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}


    
impl WorldMap{
    pub fn new(width: usize, height: usize) -> WorldMap{
        let mut world = Vec::new();
        for y in 0..height{
            world.push(Vec::new());
    
            for x in 0..width{
                world[y].push(Tile::new((x,y), width, height));
            }
        }

        WorldMap{
            width: width,
            height: height,
            tiles: world,
        }
    }

    pub fn render_map(&self) {
        for row in &self.tiles {
            for tile in row {
                print!("{:?}", tile.content);
            }
            println!();
        }
    }

    pub fn populate_map(&mut self, pattern: &str) {
        for char in pattern.chars() {
            for row in &mut self.tiles {
                for tile in row {
                    tile.content = char;
                }
            }
        }
    }
}
