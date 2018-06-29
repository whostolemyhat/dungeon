#[derive(Debug, Clone, Copy, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub centre: Point
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        Room {
            x,
            x2: x + width,
            y,
            y2: y + height,
            width,
            height,
            centre: Point {
                x: x + width / 2 as i32,
                y: y + height / 2 as i32
            }
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}

#[cfg(test)]
mod tests {
    use room::Room;

    #[test]
    fn test_new_room() {
        let room = Room::new(2, 12, 8, 9);
        assert_eq!(room.x, 2);
        assert_eq!(room.x2, 10);
        assert_eq!(room.y, 12);
        assert_eq!(room.y2, 21);
        assert_eq!(room.width, 8);
        assert_eq!(room.height, 9);
        assert_eq!(room.centre.x, 6);
        assert_eq!(room.centre.y, 16);
    }

    #[test]
    fn test_intersects() {
        let room = Room::new(2, 12, 8, 9);
        let other = Room::new(3, 12, 8, 9);
        let third = Room::new(18, 20, 4, 4);

        assert!(room.intersects(&other));
        assert!(!room.intersects(&third));
        assert!(other.intersects(&room));
        assert!(!other.intersects(&third));
        assert!(!third.intersects(&other));
        assert!(!third.intersects(&room));
    }
}