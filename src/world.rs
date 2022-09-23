use std::io::Write;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use crate::tile::{Tile, TileKind};


pub struct WorldMap{
    pub width: usize,
    pub height: usize,
    pub world: Vec<Vec<Tile>>,
}

impl WorldMap{
    pub fn new(width: usize, height: usize) -> WorldMap{
        let mut world = Vec::new();
        for y in 0..height{
            world.push(Vec::new());
    
            for x in 0..width{
                world[y].push(Tile::new(x,y, TileKind::Void, width, height));
            }
        }

        WorldMap{
            width: width,
            height: height,
            world: world,
        }
    }

    pub fn render_map(&self) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        for row in &self.world {
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
                // Yes this is bad, no I don't care
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::White)));
            }
            println!();
        }
    }


}

