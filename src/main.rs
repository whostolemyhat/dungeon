extern crate rand;
extern crate sha2;

pub mod draw;

use std::fmt;
use sha2::{ Sha256, Digest };
use rand::{ SeedableRng, StdRng };
use rand::distributions::{ IndependentSample, Range };

use draw::draw;

struct Level {
    width: i32,
    height: i32,
    board: Vec<Vec<i32>>
}

impl Level {
    fn new(width: i32, height: i32) -> Self {
        let mut board = Vec::new();
        for _ in 0..height as usize {
            let row = vec![0; width as usize];
            board.push(row);
        }

        Level {
            width,
            height,
            board
        }
    }

    fn add_room(&mut self, room: &Room) {
        println!("{:?}", &room);
        for row in 0..room.height {
            for col in 0..room.width {
                self.board[(room.y1 + row) as usize][(room.x1 + col) as usize] = 1;
            }
        }

    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::fmt::Result {
        for row in &self.board {
            write!(f, "{:?}\n", row)?
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug)]
pub struct Room {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    width: i32,
    height: i32,
    centre: Point
}

impl Room {
    fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            x1: x,
            x2: x + width,
            y1: y,
            y2: y + height,
            width,
            height,
            centre: Point{ x: (x + (x + width) / 2), y: (y + (y + height) / 2) }
        }
    }

    fn intersects(&self, other: &Room) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}

fn create_seed(text: &str) -> String {
    let mut hasher = Sha256::default();
    hasher.input(text.as_bytes());
    format!("{:x}", hasher.result())
}

fn hash_sum(hash: &str) -> usize {
    hash.as_bytes().into_iter().fold(0, |acc, byte| acc + *byte as usize)
}

fn main() {
    let max_rooms = 10;
    // TODO export these to draw
    let tile_size = 16.0;
    let image_width = 768.0;
    let image_height = 640.0;
    let board_width = image_width / tile_size;
    let board_height = image_height / tile_size;

    let hash = create_seed("brian");
    let seed: &[_] = &[hash_sum(&hash)];
    let mut rng: StdRng = SeedableRng::from_seed(seed);
    let mut rooms = Vec::new();

    // placement
    let min_room_width = 4;
    let max_room_width = 8;
    let min_room_height = 5;
    let max_room_height = 12;

    // TODO fix out of bounds
    let between = Range::new(0, board_width as i32 - max_room_width);
    let between_y = Range::new(0, board_height as i32 - max_room_height);
    let height_range = Range::new(min_room_height, max_room_height);
    let width_range = Range::new(min_room_width, max_room_width);

    for _ in 0..max_rooms {
        let x = between.ind_sample(&mut rng);
        let y = between_y.ind_sample(&mut rng);
        let width = width_range.ind_sample(&mut rng);
        let height = height_range.ind_sample(&mut rng);

        let mut collides = false;
        let room = Room::new(x, y, width, height);

        for other_room in &rooms {
            if room.intersects(&other_room) {
                collides = true;
                break;
            }
        }

        if !collides {
            rooms.push(room);
        }
    }

    // update board
    let mut level = Level::new(board_width as i32, board_height as i32);
    for room in &rooms {
        level.add_room(&room);
    }

    println!("{}", level);
    println!("{:?}", rooms);
    draw(rooms, ".").unwrap();
}
