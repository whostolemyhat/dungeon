use arrayref::array_ref;
use dungeon::{bsp, draw, roomscorridors};

use clap::{Arg, Command};
use rand::distributions::Alphanumeric;
use rand::prelude::*;
use sha2::{Digest, Sha256};

use draw::draw;

use bsp::BspLevel;
use roomscorridors::RoomsCorridors;

fn create_hash(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.update(text.as_bytes());
    format!("{:x}", hasher.finalize())
}

enum Algorithm {
    Bsp,
    Rooms,
}

fn main() {
    // config:
    // hash (pass hash directly)
    // text (hashed + used)
    // width
    // height
    // max rooms
    // room size
    let matches = Command::new("Dungeon")
        .version("3.0")
        .author("James Baum <@whostolemyhat>")
        .arg(
            Arg::new("text")
                .short('t')
                .long("text")
                .help("A string to hash and use as a seed"),
        )
        .arg(
            Arg::new("seed")
                .short('s')
                .long("seed")
                .help("An existing seed. Must be 32 characters"),
        )
        .arg(
            Arg::new("algo")
                .short('a')
                .long("algorithm")
                .value_parser(["rooms", "bsp"])
                .default_value("rooms")
                .help("The type of procedural algorithm to use"),
        )
        .arg(
            Arg::new("json")
                .short('j')
                .long("json")
                .num_args(0)
                .help("If set, displays serialised JSON output"),
        )
        .arg(
            Arg::new("draw")
                .short('d')
                .long("draw")
                .num_args(0)
                .help("If set, creates a png representation"),
        )
        .arg(
            Arg::new("csv")
                .short('c')
                .long("csv")
                .num_args(0)
                .help("Output board in CSV format"),
        )
        .arg(
            Arg::new("walls")
                .short('w')
                .long("walls")
                .num_args(0)
                .help("Add wall tile around rooms"),
        )
        .arg(
            Arg::new("height")
                .short('y')
                .default_value("40")
                .long("height")
                .help("Height of the level"),
        )
        .arg(
            Arg::new("width")
                .short('x')
                .long("width")
                .default_value("48")
                .help("Width of the level"),
        )
        .arg(
            Arg::new("minroomwidth")
                .short('m')
                .long("minroomwidth")
                .default_value("4")
                .help("Minimum width of rooms"),
        )
        .arg(
            Arg::new("minroomheight")
                .short('n')
                .long("minroomheight")
                .default_value("5")
                .help("Minimum height of rooms"),
        )
        .get_matches();

    let board_width = matches
        .get_one::<String>("width")
        .expect("Width not set")
        .parse::<i32>()
        .expect("Couldn't parse width");
    let board_height = matches
        .get_one::<String>("height")
        .expect("Height not set")
        .parse::<i32>()
        .expect("Couldn't parse height");

    let seed: String = match matches.get_one::<String>("seed") {
        Some(text) => {
            if text.chars().count() < 32 {
                panic!("Seed must be 32 characters long. Use -t option to create a new seed.")
            }
            text.to_string()
        }
        None => match matches.get_one::<String>("text") {
            Some(text) => create_hash(text),
            None => create_hash(
                &thread_rng()
                    .sample_iter(&Alphanumeric)
                    .take(32)
                    .collect::<String>(),
            ),
        },
    };

    let walls = matches.contains_id("walls");
    let method = match matches
        .get_one::<String>("algo")
        .expect("Default algorithm not set")
        .as_str()
    {
        "bsp" => Algorithm::Bsp,
        "rooms" => Algorithm::Rooms,
        _ => unreachable![],
    };

    let min_room_width: i32 = matches
        .get_one::<String>("minroomwidth")
        .expect("No room width")
        .parse::<i32>()
        .expect("Couldn't parse room width");
    let min_room_height: i32 = matches
        .get_one::<String>("minroomheight")
        .expect("No room height")
        .parse::<i32>()
        .expect("Couldn't parse room height");

    let seed_u8 = array_ref!(seed.as_bytes(), 0, 32);
    let mut rng: StdRng = SeedableRng::from_seed(*seed_u8);

    let level = match method {
        Algorithm::Rooms => RoomsCorridors::create(
            board_width,
            board_height,
            &seed,
            &mut rng,
            walls,
            min_room_width,
            min_room_height,
        ),
        Algorithm::Bsp => BspLevel::create(
            board_width,
            board_height,
            &seed,
            &mut rng,
            walls,
            min_room_width,
            min_room_height,
        ),
    };

    let print_json = matches.contains_id("json");
    let draw_map = matches.contains_id("draw");
    let csv = matches.contains_id("csv");

    println!("{}", level);
    if print_json {
        let serialised = serde_json::to_string(&level).expect("Serialising level failed");
        println!("{}", serialised);
    }

    if draw_map {
        draw(&level, "./img", &seed).expect("Drawing failed");
    }

    if csv {
        println!("{:?}", level.board_to_csv());
    }
}

// include pre-generated rooms
// add detail to rooms
// drunkards walk
// bresenhams line algorithm
// non-rectangular rooms
// quadtree
// grid (gen on top + pick random direction)
// cellular automata https://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664
// bsp https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268

// http://www.gamasutra.com/blogs/AAdonaac/20150903/252889/Procedural_Dungeon_Generation_Algorithm.php
