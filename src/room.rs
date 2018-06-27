#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone, Copy)]
pub struct Room {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub centre: Point
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        println!("{:?}", width as f64 / 2.0);
        println!("floor {:?}", (width as f64 / 2.0).floor());
        println!("round {:?}", (width as f64 / 2.0).round());
        Room {
            x1: x,
            x2: x + width,
            y1: y,
            y2: y + height,
            width,
            height,
            centre: Point{
                x: x + (width as f64 / 2.0).floor() as i32,
                y: y + (height as f64 / 2.0).floor() as i32
            }
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x1 <= other.x2 && self.x2 >= other.x1 && self.y1 <= other.y2 && self.y2 >= other.y1
    }
}