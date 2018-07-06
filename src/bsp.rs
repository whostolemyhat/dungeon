// https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268
use rand::{ Rng, StdRng };

#[derive(Debug, Clone, Copy)]
pub struct Leaf {
    min_size: i32,
    x: i32,
    y: i32,
    pub width: i32,
    pub height: i32,
    // pub left_child: Option<&'a Leaf<'a>>,
    // pub right_child: Option<&'a Leaf<'a>>,
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
            // left_child: None,
            // right_child: None,
            room: 0,
            // corridors: Vec::new()
        }
    }

    pub fn split(&mut self, rng: &mut StdRng) -> (Option<Self>, Option<Self>) {
        // match self.left_child {
        //     Some(_) => match self.right_child {
        //         Some(_) => return false, // already set
        //         None => (),
        //     },
        //     None => ()
        // };

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
            // return false;   // too small
            return (None, None);
        }

        let split_pos = rng.gen_range(self.min_size, max); // TODO pick random point(min_size, max)
        if split_horz {
            (Some(Leaf::new(self.x, self.y, self.width, split_pos)), Some(Leaf::new(self.x, self.y + split_pos, self.width, split_pos)))
            // self.left_child = Some(Box::new(Leaf::new(self.x, self.y, self.width, split_pos)));
            // self.right_child = Some(Box::new(Leaf::new(self.x, self.y + split_pos, self.width, self.height - split_pos)));
        } else {
            (Some(Leaf::new(self.x, self.y, split_pos, self.height)), Some(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height)))
            // self.left_child = Some(Box::new(Leaf::new(self.x, self.y, split_pos, self.height)));
            // self.right_child = Some(Box::new(Leaf::new(self.x + split_pos, self.y, self.width - split_pos, self.height)));
        }

        // true
    }
}