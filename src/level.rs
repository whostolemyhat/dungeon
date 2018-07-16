use std::fmt;
use room::Room;
use tile::Tile;

#[derive(Serialize)]
pub struct Level {
    pub hash: String,
    pub tile_size: i32,
    pub width: i32,
    pub height: i32,
    pub board: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
}

impl Level {
    pub fn new(width: i32, height: i32, hash: &String) -> Self {
        let mut board = Vec::new();
        for _ in 0..height {
            let row = vec![Tile::Empty; width as usize];
            board.push(row);
        }

        Level {
            tile_size: 16,
            width,
            height,
            board,
            rooms: vec![],
            hash: hash.clone()
        }
    }

    pub fn add_room(&mut self, room: &Room) {
        for row in 0..room.height {
            for col in 0..room.width {
                let y = (room.y + row) as usize;
                let x = (room.x + col) as usize;

                self.board[y][x] = Tile::Walkable;
            }
        }

        self.rooms.push(*room);
    }

    pub fn add_walls(&mut self) {
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                match self.board[y][x] {
                    Tile::Walkable => {
                        // ugly code to avoid overflow (ie < 0 in usize)
                        if x >= 1 {
                            if y >= 1 {
                                self.add_wall(x - 1, y - 1);
                            }

                            self.add_wall(x - 1, y);
                            self.add_wall(x - 1, y + 1);
                        }

                        if y >= 1 {
                            self.add_wall(x, y - 1);
                            self.add_wall(x + 1, y - 1);
                        }

                        self.add_wall(x + 1, y);

                        self.add_wall(x, y + 1);
                        self.add_wall(x + 1, y + 1);
                    },
                    _ => ()
                }
            }
        }
    }

    fn add_wall(&mut self, x: usize, y: usize) {
        if x >=self.width as usize || y > self.height as usize {
            return;
        }

        if self.board[y][x] == Tile::Empty {
            self.board[y][x] = Tile::Wall;
        }

    }

    pub fn board_to_csv(&self) -> Vec<String> {
        let mut output = Vec::new();
        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                if self.board[row][col] == Tile::Empty {
                    output.push("0".to_string());
                } else {
                    output.push(format!("{}", self.board[row][col]));
                }
            }

            output.push("\n".to_string());
        }

        output
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}\n", self.hash)?;

        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{} ", self.board[row][col])?
            }
            write!(f, "\n")?
        }

        Ok(())
    }
}
