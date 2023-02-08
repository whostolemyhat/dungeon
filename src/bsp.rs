// https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
use rand::{Rng, StdRng};
use serde_json::from_str;
use std::fs;

use crate::level::Level;
use crate::room::Room;
use crate::tile::Tile;

type RoomJson = Vec<Vec<Tile>>;

fn load_rooms() -> std::io::Result<Vec<RoomJson>> {
    let dir = "rooms";
    let mut rooms = vec![];

    for room in fs::read_dir(dir)? {
        let room = room?;
        let path = room.path();
        let dynamic = fs::read_to_string(path)?;
        let json: RoomJson = from_str(&dynamic)?;
        rooms.push(json);
    }

    Ok(rooms)
}

pub struct BspLevel {
    level: Level,
}

impl BspLevel {
    pub fn create(
        width: i32,
        height: i32,
        hash: &str,
        rng: &mut StdRng,
        add_walls: bool,
        min_room_width: i32,
        min_room_height: i32,
    ) -> Level {
        let level = Level::new(width, height, hash, min_room_width, min_room_height);

        let mut map = BspLevel { level };

        map.place_rooms(rng);

        if add_walls {
            map.level.add_walls();
        }

        map.level
    }

    fn place_rooms(&mut self, rng: &mut StdRng) {
        // let prebuilt = vec![
        //     vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
        //     vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Walkable, Tile::Walkable],
        //     vec![Tile::Walkable, Tile::Walkable, Tile::Empty, Tile::Empty, Tile::Empty, Tile::Empty],
        //     vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
        //     vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable]
        // ];

        // let another = room![
        //     [0, 0, 0, 1, 0, 0, 0],
        //     [0, 0, 1, 1, 1, 0, 0],
        //     [0, 1, 1, 1, 1, 1, 0],
        //     [1, 1, 1, 1, 1, 1, 1],
        //     [1, 1, 0, 1, 0, 1, 1],
        //     [1, 1, 1, 1, 1, 1, 1],
        //     [1, 1, 0, 1, 0, 1, 1],
        //     [1, 1, 1, 1, 1, 1, 1],
        //     [0, 1, 1, 1, 1, 1, 0],
        //     [0, 0, 1, 1, 1, 0, 0],
        //     [0, 0, 0, 1, 0, 0, 0]
        // ];

        // let obstacles = room![
        //     [1, 1, 0, 1, 0, 0, 0],
        //     [0, 1, 1, 1, 1, 0, 0],
        //     [0, 1, 1, 1, 1, 1, 0],
        //     [1, 1, 1, 1, 1, 1, 1],
        //     [1, 1, 2, 1, 2, 1, 1],
        //     [1, 1, 1, 1, 1, 1, 1],
        //     [1, 1, 2, 1, 2, 1, 1],
        //     [1, 1, 1, 1, 1, 1, 1],
        //     [0, 1, 1, 1, 1, 1, 0],
        //     [0, 1, 1, 1, 1, 0, 0],
        //     [1, 1, 0, 1, 1, 1, 0]
        // ];

        // let rooms = vec![json];
        let rooms = load_rooms().expect("Error opening room files");

        let min_size = 8;
        let mut root = Leaf::new(
            0,
            0,
            self.level.width,
            self.level.height,
            min_size,
            self.level.min_room_width,
            self.level.min_room_height,
        );
        root.generate(rng);
        root.create_rooms(rng, &mut rooms.iter());

        for leaf in root.iter() {
            if leaf.is_leaf() {
                if let Some(room) = leaf.get_room() {
                    self.level.add_room(&room);
                }
            }

            for corridor in &leaf.corridors {
                self.level.add_room(corridor);
            }
        }
    }
}

struct Leaf {
    min_size: i32,
    min_room_width: i32,
    min_room_height: i32,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    left_child: Option<Box<Leaf>>,
    right_child: Option<Box<Leaf>>,
    room: Option<Room>,
    corridors: Vec<Room>,
}

impl Leaf {
    pub fn new(
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        min_size: i32,
        min_room_width: i32,
        min_room_height: i32,
    ) -> Self {
        Leaf {
            min_size,
            min_room_width,
            min_room_height,
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None,
            corridors: vec![],
        }
    }

    fn is_leaf(&self) -> bool {
        match self.left_child {
            None => self.right_child.is_none(),
            Some(_) => false,
        }
    }

    fn generate(&mut self, rng: &mut StdRng) {
        if self.is_leaf() && self.split(rng) {
            if let Some(ref mut left) = self.left_child {
                left.as_mut().generate(rng);
            };

            if let Some(ref mut right) = self.right_child {
                right.as_mut().generate(rng);
            };
        }
    }

    fn split(&mut self, rng: &mut StdRng) -> bool {
        // if width >25% height, split vertically
        // if height >25% width, split horz
        // otherwise random
        // let mut split_horz = match rng.gen_range(0, 2) {
        //     0 => false,
        //     _ => true,
        // };

        let mut split_horz = !matches!(rng.gen_range(0, 2), 0);

        if self.width > self.height && (self.width as f32 / self.height as f32) >= 1.25 {
            split_horz = false;
        } else if self.height > self.width && (self.height as f32 / self.width as f32) >= 1.25 {
            split_horz = true;
        }

        let max = match split_horz {
            true => self.height - self.min_size,
            false => self.width - self.min_size,
        };

        if max <= self.min_size {
            return false; // too small
        }

        let split_pos = rng.gen_range(self.min_size, max);
        if split_horz {
            self.left_child = Some(Box::new(Leaf::new(
                self.x,
                self.y,
                self.width,
                split_pos,
                self.min_size,
                self.min_room_width,
                self.min_room_height,
            )));
            self.right_child = Some(Box::new(Leaf::new(
                self.x,
                self.y + split_pos,
                self.width,
                self.height - split_pos,
                self.min_size,
                self.min_room_width,
                self.min_room_height,
            )));
        } else {
            self.left_child = Some(Box::new(Leaf::new(
                self.x,
                self.y,
                split_pos,
                self.height,
                self.min_size,
                self.min_room_width,
                self.min_room_height,
            )));
            self.right_child = Some(Box::new(Leaf::new(
                self.x + split_pos,
                self.y,
                self.width - split_pos,
                self.height,
                self.min_size,
                self.min_room_width,
                self.min_room_height,
            )));
        }

        true
    }

    fn create_rooms<'a, I>(&mut self, rng: &mut StdRng, rooms: &mut I)
    where
        I: Iterator<Item = &'a Vec<Vec<Tile>>>,
    {
        if let Some(ref mut room) = self.left_child {
            room.as_mut().create_rooms(rng, rooms);
        };

        if let Some(ref mut room) = self.right_child {
            room.as_mut().create_rooms(rng, rooms);
        };

        // if last level, add a room
        if self.is_leaf() {
            let room = rooms.next();
            let width = rng.gen_range(self.min_room_width, self.width);
            let height = rng.gen_range(self.min_room_height, self.height);
            let x = rng.gen_range(0, self.width - width);
            let y = rng.gen_range(0, self.height - height);

            match room {
                Some(prebuilt) => {
                    self.room = Some(Room::new(
                        x + self.x,
                        y + self.y,
                        prebuilt[0].len() as i32,
                        prebuilt.len() as i32,
                        Some(prebuilt.clone()),
                    ))
                }
                None => self.room = Some(Room::new(x + self.x, y + self.y, width, height, None)),
            };
        }

        // if there's a left and right child, create a corridor between them
        if let (Some(ref mut left), Some(ref mut right)) =
            (&mut self.left_child, &mut self.right_child)
        {
            create_corridors(rng, left, right);
        };
    }

    fn get_room(&self) -> Option<Room> {
        if self.is_leaf() {
            return self.room.clone();
        }

        let mut left_room: Option<Room> = None;
        let mut right_room: Option<Room> = None;

        if let Some(ref room) = self.left_child {
            left_room = room.get_room();
        }

        if let Some(ref room) = self.right_child {
            right_room = room.get_room();
        }

        match (left_room, right_room) {
            (None, None) => None,
            (Some(room), _) => Some(room),
            (_, Some(room)) => Some(room),
        }
    }

    fn iter(&self) -> LeafIterator {
        LeafIterator::new(self)
    }
}

// corridors are just very narrow rooms
fn create_corridors(rng: &mut StdRng, left: &mut Box<Leaf>, right: &mut Box<Leaf>) {
    if let (Some(left_room), Some(right_room)) = (left.get_room(), right.get_room()) {
        // pick point in each room
        let left_point = (
            rng.gen_range(left_room.x, left_room.x + left_room.width),
            rng.gen_range(left_room.y, left_room.y + left_room.height),
        );
        let right_point = (
            rng.gen_range(right_room.x, right_room.x + right_room.width),
            rng.gen_range(right_room.y, right_room.y + right_room.height),
        );

        match rng.gen_range(0, 2) {
            0 => {
                match left_point.0 <= right_point.0 {
                    true => left.corridors.push(horz_corridor(
                        left_point.0,
                        left_point.1,
                        right_point.0,
                    )),
                    false => left.corridors.push(horz_corridor(
                        right_point.0,
                        left_point.1,
                        left_point.0,
                    )),
                }
                match left_point.1 <= right_point.1 {
                    true => left.corridors.push(vert_corridor(
                        right_point.0,
                        left_point.1,
                        right_point.1,
                    )),
                    false => left.corridors.push(vert_corridor(
                        right_point.0,
                        right_point.1,
                        left_point.1,
                    )),
                }
            }
            _ => {
                match left_point.1 <= right_point.1 {
                    true => left.corridors.push(vert_corridor(
                        left_point.0,
                        left_point.1,
                        right_point.1,
                    )),
                    false => left.corridors.push(vert_corridor(
                        left_point.0,
                        right_point.1,
                        left_point.1,
                    )),
                }
                match left_point.0 <= right_point.0 {
                    true => left.corridors.push(horz_corridor(
                        left_point.0,
                        right_point.1,
                        right_point.0,
                    )),
                    false => left.corridors.push(horz_corridor(
                        right_point.0,
                        right_point.1,
                        left_point.0,
                    )),
                }
            }
        }
    };
}

fn horz_corridor(start_x: i32, start_y: i32, end_x: i32) -> Room {
    Room::new(start_x, start_y, (end_x - start_x) + 1, 1, None)
}

fn vert_corridor(start_x: i32, start_y: i32, end_y: i32) -> Room {
    Room::new(start_x, start_y, 1, end_y - start_y, None)
}

struct LeafIterator<'a> {
    current_node: Option<&'a Leaf>,
    right_nodes: Vec<&'a Leaf>,
}

impl<'a> LeafIterator<'a> {
    fn new(root: &'a Leaf) -> LeafIterator<'a> {
        let mut iter = LeafIterator {
            right_nodes: vec![],
            current_node: None,
        };

        iter.add_left_subtree(root);
        iter
    }

    // set the current node to the one provided
    // and add any child leaves to the node vec
    fn add_left_subtree(&mut self, node: &'a Leaf) {
        if let Some(ref left) = node.left_child {
            self.right_nodes.push(left);
        }
        if let Some(ref right) = node.right_child {
            self.right_nodes.push(right);
        }

        self.current_node = Some(node);
    }
}

impl<'a> Iterator for LeafIterator<'a> {
    type Item = &'a Leaf;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current_node.take();
        if let Some(rest) = self.right_nodes.pop() {
            self.add_left_subtree(rest);
        }

        match result {
            Some(leaf) => Some(leaf),
            None => None,
        }
    }
}
