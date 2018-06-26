use rand::{ StdRng };
use rand::distributions::{ IndependentSample, Range };

use std::fmt;
use room::Room;

pub struct Level {
    pub hash: String,
    pub tile_size: i32,
    pub width: i32,
    pub height: i32,
    pub board: Vec<i32>,
    pub rooms: Vec<Room>,
}

impl Level {
    pub fn new(width: i32, height: i32, hash: &String) -> Self {
        Level {
            tile_size: 16,
            width,
            height,
            board: vec![0; (height * width) as usize],
            rooms: Vec::new(),
            hash: hash.clone()
        }
    }

    fn get_tile_coords(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn add_room(&mut self, room: &Room) {
        // TODO check bounds
        for row in 0..room.height {
            for col in 0..room.width {
                let y = room.y1 + row;
                let x = room.x1 + col;
                let coord = self.get_tile_coords(x, y);

                self.board[coord] = 1;
            }
        }

        self.rooms.push(*room);
    }

    fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
        for col in start_x..end_x {
            let pos = self.get_tile_coords(col, y);
            self.board[pos] = 1;
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y {
            let pos = self.get_tile_coords(x, row);
            self.board[pos] = 1;
        }
    }

    pub fn place_rooms(&mut self, mut rng: StdRng) {
        let max_rooms = 10;

        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 5;
        let max_room_height = 12;

        // TODO fix out of bounds
        let between = Range::new(0, self.width - max_room_width);
        let between_y = Range::new(0, self.height - max_room_height);
        let height_range = Range::new(min_room_height, max_room_height);
        let width_range = Range::new(min_room_width, max_room_width);

        for _ in 0..max_rooms {
            let x = between.ind_sample(&mut rng);
            let y = between_y.ind_sample(&mut rng);
            let width = width_range.ind_sample(&mut rng);
            let height = height_range.ind_sample(&mut rng);

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            for other_room in &self.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.add_room(&room);
            }
        }
    }

    pub fn place_corridors(&mut self, mut rng: StdRng) {
        // TODO check bounds/len
        for i in 0..(self.rooms.len() - 1) {
            let room = self.rooms[i];
            let other = self.rooms[i + 1];

            // randomly pick vert or horz
            match Range::new(0, 2).ind_sample(&mut rng) {
                0 => {
                    match room.centre.x < other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y)
                    }
                    match room.centre.y < other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x)
                    }
                }
                _ => {
                    match room.centre.y < other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x)
                    }
                    match room.centre.x < other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y)
                    }
                }
            }
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.hash)?;
        for (i, row) in self.board.iter().enumerate() {
            if i > 0 && i % self.width as usize == 0 {
                write!(f, "\n")?;
            }
            write!(f, " {:?} ", row)?;
        }

        Ok(())
    }
}