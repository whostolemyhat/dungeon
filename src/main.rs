extern crate rand;
extern crate sha2;

pub mod draw;
mod level;
mod room;

use sha2::{ Sha256, Digest };
use rand::{ SeedableRng, StdRng };

use draw::draw;
use level::Level;

fn create_seed(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

fn hash_sum(hash: &str) -> usize {
    hash.as_bytes().into_iter().fold(0, |acc, byte| acc + *byte as usize)
}

fn main() {
    let board_width = 48;
    let board_height = 40;
    let hash = create_seed("brian");
    let mut level = Level::new(board_width, board_height, &hash);

    let seed: &[_] = &[hash_sum(&hash)];
    let rng: StdRng = SeedableRng::from_seed(seed);

    level.place_rooms(rng);
    level.place_corridors(rng);

    println!("{}", level);
    draw(level, ".").unwrap();
}

// drunkards walk
// bsp
// grid (gen on top + pick random direction)
// cellular automata