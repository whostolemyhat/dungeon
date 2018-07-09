// https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
use rand::{ Rng, StdRng };
// use std::rc::Rc;
// use std::cell::Cell;

#[derive(Debug, Clone)]
pub struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    pub width: i32,
    pub height: i32,
    pub left_child: Option<Box<Leaf>>,
    pub right_child: Option<Box<Leaf>>,
    room: i32,
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
            room: 0,
            // corridors: Vec::new()
        }
    }

    pub fn print(&self) {
        println!("{:?}", self);
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

    pub fn generate(mut leaf: Leaf, rng: &mut StdRng) {
        if leaf.is_leaf() {
            let max = 10;
            if leaf.width > max {
                if leaf.split(rng) {
                    println!("splitting");
                    Self::generate(*leaf.left_child.unwrap(), rng);
                    Self::generate(*leaf.right_child.unwrap(), rng);
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
            // self.left_child = Some(Leaf::new(self.x, self.y, self.width, split_pos));
            // self.right_child = Some(Leaf::new(self.x, self.y + split_pos, self.width, split_pos));
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, self.width, split_pos)));
            self.right_child = Some(Box::new(Leaf::new(self.x, self.y + split_pos, self.width, self.height - split_pos)));
        } else {
            // self.left_child = Some(Leaf::new(self.x, self.y, split_pos, self.height));
            // self.right_child = Some(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height));
            self.left_child = Some(Box::new(Leaf::new(self.x, self.y, split_pos, self.height)));
            self.right_child = Some(Box::new(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height)));
        }

        true
    }
}