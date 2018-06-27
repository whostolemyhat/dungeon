extern crate rand;
extern crate sha2;
#[macro_use]
extern crate arrayref;

pub mod draw;
mod level;
mod room;

use sha2::{ Sha256, Digest };
use rand::prelude::*;

use draw::draw;
use level::Level;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

fn main() {
    let board_width = 48;
    let board_height = 40;
    let hash = create_hash("ohdearmanuelneuer");

    let mut level = Level::new(board_width, board_height, &hash);
    let seed = array_ref!(hash.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed);

    level.place_rooms(&mut rng);
    level.place_corridors(&mut rng);

    println!("{}", level);
    draw(level, ".").unwrap();
}

// drunkards walk
// bsp
// grid (gen on top + pick random direction)
// cellular automata