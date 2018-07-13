extern crate rand;
extern crate sha2;
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate clap;

mod draw;
mod tile;
mod level;
mod room;
mod roomscorridors;
mod bsp;

use sha2::{ Sha256, Digest };
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use clap::{ App, Arg };

use draw::{ draw };

use roomscorridors::{ RoomsCorridors };
use bsp::{ BspLevel };

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

enum Algorithm {
    Bsp,
    Rooms
}

fn main() {
    // config:
    // hash (pass hash directly)
    // text (hashed + used)
    // width
    // height
    // max rooms
    // room size
    let matches = App::new("Dungeon")
                    .version("1.0")
                    .author("James Baum <@whostolemyhat>")
                    .arg(Arg::with_name("text")
                        .short("t")
                        .long("text")
                        .takes_value(true)
                        .help("A string to hash and use as a seed"))
                    .arg(Arg::with_name("seed")
                        .short("s")
                        .long("seed")
                        .takes_value(true)
                        .help("An existing seed. Must be 32 characters"))
                    .arg(Arg::with_name("algo")
                        .short("a")
                        .long("algorithm")
                        .takes_value(true)
                        .possible_values(&["rooms", "bsp"])
                        .help("The type of procedural algorithm to use"))
                    .get_matches();

    let board_width = 48;
    let board_height = 40;

    let seed: String = match matches.value_of("seed") {
        Some(text) => {
            if text.chars().count() < 32 {
                panic!("Seed must be 32 characters long. Use -t option to create a new seed.")
            }
            text.to_string()
        },
        None => {
            match matches.value_of("text") {
               Some(text) => create_hash(&text),
               None => create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>())
           }
        }
    };

    let procgen_method = match matches.value_of("algo") {
        Some(algo) => match algo {
            "bsp" => Algorithm::Bsp,
            "rooms" => Algorithm::Rooms,
            _ => Algorithm::Rooms
        },
        None => Algorithm::Rooms
    };

    let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);

    let level = match procgen_method {
        Algorithm::Rooms => RoomsCorridors::new(board_width, board_height, &seed, &mut rng),
        Algorithm::Bsp => BspLevel::new(board_width, board_height, &seed, &mut rng)
    };

    println!("{}", level);
    let serialised = serde_json::to_string(&level).unwrap();
    println!("{}", serialised);
    draw(&level, "./img", "rooms").unwrap();
}

// drunkards walk
// bresenhams line algorithm
// quadtree
// grid-based http://roguebasin.roguelikedevelopment.org/index.php?title=Grid_Based_Dungeon_Generator
// bsp https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
// grid (gen on top + pick random direction)
// cellular automata https://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664