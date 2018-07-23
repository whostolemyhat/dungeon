// https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
use rand::{ Rng, StdRng };
use room::Room;

use level::Level;

pub struct BspLevel {
    level: Level
}

impl BspLevel {
    pub fn new(width: i32, height: i32, hash: &String, rng: &mut StdRng, add_walls: bool) -> Level {
        let level = Level::new(width, height, hash);

        let mut map = BspLevel {
            level
        };

        map.place_rooms(rng);

        if add_walls {
            map.level.add_walls();
        }

        map.level
    }

    fn place_rooms(&mut self, rng: &mut StdRng) {
        let mut root = Leaf::new(0, 0, self.level.width, self.level.height, 8);
        root.generate(rng);

        // let mut corridors = vec![];
        root.create_rooms(rng);

        // let it: Vec<&Leaf> = root.iter().collect();
        // println!("{:?}", it);

        for leaf in root.iter() {
            if let Some(room) = leaf.get_room() {
                self.level.add_room(&room);
            }

            // println!("corridor {:?}", &leaf.corridors);
            // for corridor in leaf.get_corridors() {
            //     self.level.add_room(&corridor);
            // }
        }

        for corridor in root.get_corridors() {
            self.level.add_room(&corridor);
        }
    }
}

#[derive(Clone, Debug)]
struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    pub width: i32,
    pub height: i32,
    pub left_child: Option<Box<Leaf>>,
    pub right_child: Option<Box<Leaf>>,
    room: Option<Room>,
    corridors: Vec<Room>
}

impl Leaf {
    pub fn new(x: i32, y: i32, width: i32, height: i32, min_size: i32) -> Self {
        Leaf {
            min_size,
            x,
            y,
            width,
            height,
            left_child: None,
            right_child: None,
            room: None,
            corridors: vec![]
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

    fn generate(&mut self, rng: &mut StdRng) {
        if self.is_leaf() {
            if self.split(rng) {
                self.left_child.as_mut().unwrap().generate(rng);
                self.right_child.as_mut().unwrap().generate(rng);
            }
        }
    }

    fn split(&mut self, rng: &mut StdRng) -> bool {
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
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, self.width, split_pos, self.min_size)));
            self.right_child = Some(Box::new(Leaf::new(self.x, self.y + split_pos, self.width, self.height - split_pos, self.min_size)));
        } else {
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, split_pos, self.height, self.min_size)));
            self.right_child = Some(Box::new(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height, self.min_size)));
        }

        true
    }

    fn create_rooms(&mut self, rng: &mut StdRng) {
        match self.left_child {
            Some(_) => self.left_child.as_mut().unwrap().create_rooms(rng),
            None => ()
        };
        match self.right_child {
            Some(_) => self.right_child.as_mut().unwrap().create_rooms(rng),
            None => ()
        };

        let min_room_width = 4;
        let min_room_height = 3;

        // if last level, add a room
        if self.is_leaf() {
            let width = rng.gen_range(min_room_width, self.width);
            let height = rng.gen_range(min_room_height, self.height);
            let x = rng.gen_range(0, self.width - width);
            let y = rng.gen_range(0, self.height - height);

            self.room = Some(Room::new(x + self.x, y + self.y, width, height));
            // rooms.push(self.room.unwrap());
        }

        // if there's a left and right child, create a corridor between them
        match (&mut self.left_child, &mut self.right_child) {
            (Some(ref mut left), Some(ref mut right)) => create_corridors(rng, left, right),
            _ => ()
        };
    }

    fn get_room(&self) -> Option<Room> {
        if self.is_leaf() {
            return self.room;
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
            (None, None) => return None,
            (Some(room), _) => return Some(room),
            (_, Some(room)) => return Some(room),
        };
    }

    // iterate through all children and collect every corridor
    fn get_corridors(&self) -> Vec<Room> {
        let mut corridors = vec![];

        for corridor in &self.corridors {
            corridors.push(*corridor);
        }

        if let Some(ref left) = self.left_child {
            corridors.extend(left.get_corridors());
        }

        if let Some(ref right) = self.right_child {
            corridors.extend(right.get_corridors());
        }

        corridors
    }

    fn iter(&self) -> LeafIterator {
        LeafIterator::new(&self)
    }
}

struct LeafIterator<'a> {
    current_node: Option<&'a Leaf>,
    right_nodes: Vec<&'a Leaf>
}

impl<'a> LeafIterator<'a> {
    fn new(root: &'a Leaf) -> LeafIterator<'a> {
        let mut iter = LeafIterator {
            right_nodes: vec![],
            current_node: None
        };

        iter.add_left_subtree(root);
        iter
    }

    fn add_left_subtree(&mut self, mut node: &'a Leaf) {
        loop {
            if node.is_leaf() {
                self.current_node = Some(node);
                break;
            } else {
                match node.right_child {
                   Some(ref leaf) => self.right_nodes.push(&*leaf),
                   _ => (),
                }

                match node.left_child {
                    Some(ref leaf) => {
                        node = leaf;
                    },
                    None => {}
                };
            }
        }
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
            Some(leaf) => Some(&*leaf),
            None => None
        }
    }
}


// corridors are just very narrow rooms
fn create_corridors(rng: &mut StdRng, left: &mut Box<Leaf>, right: &mut Box<Leaf>) {
    match (left.get_room(), right.get_room()) {
        (Some(left_room), Some(right_room)) => {
            // pick point in each room
            let left_point = (rng.gen_range(left_room.x, left_room.x + left_room.width), rng.gen_range(left_room.y, left_room.y + left_room.height));
            let right_point = (rng.gen_range(right_room.x, right_room.x + right_room.width), rng.gen_range(right_room.y, right_room.y + right_room.height));

            match rng.gen_range(0, 2) {
                0 => {
                    match left_point.0 <= right_point.0 {
                        true => left.corridors.push(horz_corridor(left_point.0, left_point.1, right_point.0)),
                        false => left.corridors.push(horz_corridor(right_point.0, left_point.1, left_point.0))
                    }
                    match left_point.1 <= right_point.1 {
                        true => left.corridors.push(vert_corridor(right_point.0, left_point.1, right_point.1)),
                        false => left.corridors.push(vert_corridor(right_point.0, right_point.1, left_point.1))
                    }
                }
                _ => {
                    match left_point.1 <= right_point.1 {
                        true => left.corridors.push(vert_corridor(left_point.0, left_point.1, right_point.1)),
                        false => left.corridors.push(vert_corridor(left_point.0, right_point.1, left_point.1))
                    }
                    match left_point.0 <= right_point.0 {
                        true => left.corridors.push(horz_corridor(left_point.0, right_point.1, right_point.0)),
                        false => left.corridors.push(horz_corridor(right_point.0, right_point.1, left_point.0))
                    }
                }
            }
        },
        _ => ()
    };
}


fn horz_corridor(start_x: i32, start_y: i32, end_x: i32) -> Room {
    Room::new(start_x, start_y, (end_x - start_x) + 1, 1)
}

fn vert_corridor(start_x: i32, start_y: i32, end_y: i32) -> Room {
    Room::new(start_x, start_y, 1, end_y - start_y)
}