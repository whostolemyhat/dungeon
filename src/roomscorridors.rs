use rand::{ Rng, StdRng };

use level::Level;
use room::Room;
use tile::Tile;

pub struct RoomsCorridors {
    level: Level
}

impl RoomsCorridors {
    pub fn new(width: i32, height: i32, hash: &String, rng: &mut StdRng, add_walls: bool) -> Level {
        let level = Level::new(width, height, hash);

        let mut map = RoomsCorridors {
            level
        };

        map.place_rooms(rng);
        map.place_corridors(rng);

        if add_walls {
            map.level.add_walls();
        }

        map.level
    }

    fn horz_corridor(&mut self, start_x: i32, end_x: i32, y: i32) {
        for col in start_x..end_x + 1 {
            self.level.board[y as usize][col as usize] = Tile::Walkable;
        }
    }

    fn vert_corridor(&mut self, start_y: i32, end_y: i32, x: i32) {
        for row in start_y..end_y + 1 {
            self.level.board[row as usize][x as usize] = Tile::Walkable;
        }
    }

    fn place_rooms(&mut self, rng: &mut StdRng) {
        let max_rooms = 10;

        let min_room_width = 4;
        let max_room_width = 8;
        let min_room_height = 5;
        let max_room_height = 12;

        // TODO fix out of bounds
        for _ in 0..max_rooms {
            let mut x = rng.gen_range(0, self.level.width);
            let mut y = rng.gen_range(0, self.level.height);
            let width = rng.gen_range(min_room_width, max_room_width);
            let height = rng.gen_range(min_room_height, max_room_height);

            if x + width > self.level.width {
                x = self.level.width - width;
            }

            if y + height > self.level.height {
                y = self.level.height - height;
            }

            let mut collides = false;
            let room = Room::new(x, y, width, height);

            for other_room in &self.level.rooms {
                if room.intersects(&other_room) {
                    collides = true;
                    break;
                }
            }

            if !collides {
                self.level.add_room(&room);
            }

            // draw(&self, "./img", format!("0{}", i + 1).as_str()).unwrap();
        }
    }

    // fn add_walls(&mut self) {
    //     let mut y = 0;
    //     for line in self.level.board {
    //         for (x, tile) in line.iter().enumerate() {
    //             match tile {
    //                 Tile::Walkable => {
    //                     self.add_wall(x - 1, y - 1);
    //                     self.add_wall(x, y - 1);
    //                     self.add_wall(x + 1, y - 1);

    //                     self.add_wall(x - 1, y);
    //                     self.add_wall(x + 1, y);

    //                     self.add_wall(x - 1, y + 1);
    //                     self.add_wall(x, y + 1);
    //                     self.add_wall(x + 1, y + 1);
    //                 },
    //                 _ => ()
    //             }
    //         }

    //         y = y + 1;
    //     }
    // }

    // fn add_wall(&mut self, x: usize, y: usize) {
    //     if self.level.board[y][x] == Tile::Empty {
    //         self.level.board[y][x] = Tile::Wall;
    //     }

    // }

    pub fn place_corridors(&mut self, rng: &mut StdRng) {
        // TODO check bounds/len
        for i in 0..(self.level.rooms.len() - 1) {
            let room = self.level.rooms[i];
            let other = self.level.rooms[i + 1];

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

            // draw(&self, "./img", format!("{}", i + 11).as_str()).unwrap();
        }
    }
}