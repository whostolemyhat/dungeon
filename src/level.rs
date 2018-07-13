use std::fmt;
use room::Room;
// use draw::draw;
use tile::Tile;

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
    pub fn get_tile_coords(&self, x: i32, y: i32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn add_room(&mut self, room: &Room) {
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

// trait TileMap {
//     fn new(width: i32, height: i32, hash: &String, rng: &mut StdRng) -> Level;
// }


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
