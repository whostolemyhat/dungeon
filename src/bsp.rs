// https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
use rand::{ Rng, StdRng };
use room::Room;
use std::fmt;

use level::{ Tile };

#[derive(Debug)]
pub struct BspLevel {
    pub hash: String,
    pub tile_size: i32,
    pub width: i32,
    pub height: i32,
    pub board: Vec<Tile>,
    pub leaves: Leaf,
    pub rooms: Vec<Room>,
}

impl BspLevel {
    pub fn new(width: i32, height: i32, hash: &String, rng: &mut StdRng) -> Self {
        let mut level = BspLevel {
            tile_size: 16,
            width,
            height,
            board: vec![Tile::Empty; (height * width) as usize],
            rooms: Vec::new(),
            hash: hash.clone(),
            leaves: Leaf::new(0, 0, width, height)
        };

        level.leaves.generate(rng);

        let mut rooms = vec![];
        level.leaves.create_rooms(rng, &mut rooms);

        // level.rooms = rooms;
        for room in rooms {
            level.add_room(&room);
        }

        level
    }

    fn get_tile_coords(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    fn add_room(&mut self, room: &Room) {
        // TODO check bounds
        for row in 0..room.height {
            for col in 0..room.width {
                let y = room.y + row;
                let x = room.x + col;
                let coord = self.get_tile_coords(x, y);

                self.board[coord] = Tile::Walkable;
            }
        }

        self.rooms.push(*room);
    }
}

impl fmt::Display for BspLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.hash)?;
        for (i, row) in self.board.iter().enumerate() {
            if i > 0 && i % self.width as usize == 0 {
                write!(f, "\n")?;
            }
            write!(f, "{} ", row)?;
        }

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    pub width: i32,
    pub height: i32,
    pub left_child: Option<Box<Leaf>>,
    pub right_child: Option<Box<Leaf>>,
    room: Option<Room>,
    // corridors: Vec<i32>
}

impl Leaf {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Leaf {
            min_size: 8,
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None,
            // corridors: Vec::new()
        }
    }

    fn is_leaf(&self) -> bool {
        match self.left_child {
            None => match self.right_child {
                None => true,
                Some(_) => false,
            },
            Some(_) => false
        }
    }

    pub fn generate(&mut self, rng: &mut StdRng) {
        if self.is_leaf() {
            let max = 10;
            if self.width > max {
                if self.split(rng) {
                    self.left_child.as_mut().unwrap().generate(rng);
                    self.right_child.as_mut().unwrap().generate(rng);
                }
            }
        }
    }

    pub fn split(&mut self, rng: &mut StdRng) -> bool {
        // if width >25% height, split vertically
        // if height >25% width, split horz
        // otherwise random
        let mut split_horz = match rng.gen_range(0, 2) {
            0 => false,
            _ => true
        };

        if self.width > self.height && (self.width as f32 / self.height as f32) >= 1.25 {
            split_horz = false;
        } else if self.height > self.width && (self.height as f32 / self.width as f32) >= 1.25 {
            split_horz = true;
        }

        let max = match split_horz {
            true => self.height - self.min_size,
            false => self.width - self.min_size
        };

        if max <= self.min_size {
            return false;   // too small
        }

        let split_pos = rng.gen_range(self.min_size, max);
        if split_horz {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, self.width, split_pos)));
            self.right_child = Some(Box::new(Leaf::new(self.x, self.y + split_pos, self.width, self.height - split_pos)));
        } else {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, split_pos, self.height)));
            self.right_child = Some(Box::new(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height)));
        }

        true
    }

    pub fn create_rooms(&mut self, rng: &mut StdRng, rooms: &mut Vec<Room>) {
        match self.left_child {
            Some(_) => self.left_child.as_mut().unwrap().create_rooms(rng, rooms),
            None => ()
        };
        match self.right_child {
            Some(_) => self.right_child.as_mut().unwrap().create_rooms(rng, rooms),
            None => ()
        };

        if self.is_leaf() {
            let width = rng.gen_range(3, self.width);
            let height = rng.gen_range(3, self.height);
            let x = rng.gen_range(0, self.width - width);
            let y = rng.gen_range(0, self.height - height);

            self.room = Some(Room::new(x + self.x, y + self.y, width, height));
            rooms.push(self.room.unwrap());
        }
    }
}