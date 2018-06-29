extern crate rand;
extern crate sha2;
#[macro_use]
extern crate arrayref;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate clap;

pub mod draw;
mod level;
mod room;

use sha2::{ Sha256, Digest };
use rand::prelude::*;
use rand::distributions::Alphanumeric;
use clap::{ App, Arg };

use draw::draw;
use level::Level;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
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
                        .help("An existing seed"))
                    .get_matches();

    // let text = matches.value_of("text").unwrap_or(thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>().as_str());
    // let hash = create_hash(&text);
    // let hash = match matches.value_of("text") {
    //     Some(text) => create_hash(&text),
    //     None => create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>())
    // };

    let board_width = 48;
    let board_height = 40;

    // let seed = array_ref!(hash.as_bytes(), 0, 32);
    let seed: String = match matches.value_of("seed") {
        Some(text) => text.to_string(),
        None => {
            match matches.value_of("text") {
               Some(text) => create_hash(&text),
               None => create_hash(&thread_rng().sample_iter(&Alphanumeric).take(32).collect::<String>())
           }
        }
    };

    let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);
    let mut level = Level::new(board_width, board_height, &seed);

    level.place_rooms(&mut rng);
    level.place_corridors(&mut rng);

    println!("{}", level);

    let serialised = serde_json::to_string(&level).unwrap();
    println!("{:?}", serialised);
    draw(&level, ".").unwrap();
}

// drunkards walk
// bsp
// grid (gen on top + pick random direction)
// cellular automata