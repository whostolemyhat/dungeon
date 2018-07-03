use rand::{ Rng, StdRng };
use serde::{ Serialize, Serializer };
use std::fmt;
use room::Room;
use draw::draw;

#[derive(Clone)]
pub enum Tile {
    Empty,
    Walkable
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Walkable => write!(f, "1")
        }
    }
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match self {
            Tile::Empty => serializer.serialize_i32(0),
            Tile::Walkable => serializer.serialize_i32(1)
        }
    }
}

#[derive(Serialize)]
pub struct Level {
    pub hash: String,
    pub tile_size: i32,
    pub width: i32,
    pub height: i32,
    pub board: Vec<Tile>,
    pub rooms: Vec<Room>,
}

impl Level {
    pub fn new(width: i32, height: i32, hash: &String) -> Self {
        Level {
            tile_size: 16,
            width,
            height,
            board: vec![Tile::Empty; (height * width) as usize],
            rooms: Vec::new(),
            hash: hash.clone()
        }
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

    fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
        for col in start_x..end_x + 1 {
            let pos = self.get_tile_coords(col, y);
            self.board[pos] = Tile::Walkable;
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y + 1 {
            let pos = self.get_tile_coords(x, row);
            self.board[pos] = Tile::Walkable;
        }
    }

    pub fn place_rooms(&mut self, rng: &mut StdRng) {
        let max_rooms = 10;

        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 5;
        let max_room_height = 12;

        // TODO fix out of bounds
        for i in 0..max_rooms {
            let mut x = rng.gen_range(0, self.width);
            let mut y = rng.gen_range(0, self.height);
            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            if x + width > self.width {
                x = self.width - width;
            }

            if y + height > self.height {
                y = self.height - height;
            }

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

            draw(&self, "./img", format!("0{}", i + 1).as_str()).unwrap();
        }
    }

    pub fn place_corridors(&mut self, rng: &mut StdRng) {
        // TODO check bounds/len
        for i in 0..(self.rooms.len() - 1) {
            let room = self.rooms[i];
            let other = self.rooms[i + 1];

            // randomly pick vert or horz
            match rng.gen_range(0, 2) {
                0 => {
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y)
                    }
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x)
                    }
                }
                _ => {
                    match room.centre.y <= other.centre.y {
                        true => self.vert_corridor(room.centre.y, other.centre.y, other.centre.x),
                        false => self.vert_corridor(other.centre.y, room.centre.y, other.centre.x)
                    }
                    match room.centre.x <= other.centre.x {
                        true => self.horz_corridor(room.centre.x, other.centre.x, room.centre.y),
                        false => self.horz_corridor(other.centre.x, room.centre.x, room.centre.y)
                    }
                }
            }

            draw(&self, "./img", format!("{}", i + 11).as_str()).unwrap();
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
            write!(f, "{} ", row)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use level::Tile;
    use serde_json;

    #[test]
    fn test_tile_display() {
        assert_eq!(format!("{}", Tile::Empty), " ");
        assert_eq!(format!("{}", Tile::Walkable), "1");
    }

    #[test]
    fn test_tile_serialise() {
        assert_eq!(serde_json::to_string(&Tile::Empty).unwrap(), "0");
        assert_eq!(serde_json::to_string(&Tile::Walkable).unwrap(), "1");
    }
}