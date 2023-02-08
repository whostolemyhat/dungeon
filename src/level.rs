use serde_derive::Serialize;
use std::fmt;

use crate::room::Room;
use crate::tile::Tile;

#[derive(Serialize)]
pub struct Level {
    pub hash: String,
    pub tile_size: i32,
    pub width: i32,
    pub height: i32,
    pub board: Vec<Vec<Tile>>,
    pub rooms: Vec<Room>,
    pub min_room_width: i32,
    pub min_room_height: i32,
}

impl Level {
    pub fn new(
        width: i32,
        height: i32,
        hash: &str,
        min_room_width: i32,
        min_room_height: i32,
    ) -> Self {
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
            hash: hash.to_string(),
            min_room_width,
            min_room_height,
        }
    }

    pub fn add_room(&mut self, room: &Room) {
        for row in 0..room.layout.len() {
            for col in 0..room.layout[row].len() {
                let y = room.y as usize + row;
                let x = room.x as usize + col;

                self.board[y][x] = room.layout[row][col];
            }
        }

        self.rooms.push(room.clone());
    }

    pub fn add_walls(&mut self) {
        // TODO add corners
        for y in 0..self.board.len() {
            for x in 0..self.board[y].len() {
                if self.board[y][x] == Tile::Walkable {
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
                }
            }
        }
    }

    fn add_wall(&mut self, x: usize, y: usize) {
        if x >= self.width as usize || y >= self.height as usize {
            return;
        }

        if self.board[y][x] == Tile::Empty {
            self.board[y][x] = Tile::Wall;
        }
    }

    pub fn board_to_csv(&self) -> String {
        let mut output = Vec::new();
        for row in 0..self.height as usize {
            let mut row_output = Vec::new();
            for col in 0..self.width as usize {
                if self.board[row][col] == Tile::Empty {
                    row_output.push("0".to_string());
                } else {
                    row_output.push(format!("{}", self.board[row][col]));
                }
            }

            output.push(row_output.join(","));
        }

        output.join("\n")
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.hash)?;

        for row in 0..self.height as usize {
            for col in 0..self.width as usize {
                write!(f, "{} ", self.board[row][col])?
            }
            writeln!(f)?
        }

        Ok(())
    }
}
