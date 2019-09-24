use serde_derive::Serialize;

use crate::tile::Tile;
use serde_derive::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Point {
    pub x: i32,
    pub y: i32
}

macro_rules! room {
    (
        $([$( $x:expr ),*]),*
    ) => (
        vec![$( vec![$(match $x {
            1 => Tile::Walkable,
            2 => Tile::Wall,
            _ => Tile::Empty,
        }),*]),*]
    )
}

#[derive(Debug, Clone, Serialize)]
pub struct Room {
    pub x: i32,
    pub y: i32,
    pub x2: i32,
    pub y2: i32,
    pub width: i32,
    pub height: i32,
    pub centre: Point,
    pub layout: Vec<Vec<Tile>>
}

impl Room {
    pub fn new(x: i32, y: i32, width: i32, height: i32, layout: Option<Vec<Vec<Tile>>>) -> Self {
        let tiles = match layout {
            Some(tiles) => tiles,
            None => {
                let mut board = vec![];
                for _ in 0..height {
                    let row = vec![Tile::Walkable; width as usize];
                    board.push(row);
                }

                board
            }
        };

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
            },
            layout: tiles
        }
    }

    pub fn intersects(&self, other: &Self) -> bool {
        self.x <= other.x2 && self.x2 >= other.x && self.y <= other.y2 && self.y2 >= other.y
    }
}


#[cfg(test)]
mod tests {
    use crate::room::Room;
    use crate::tile::Tile;

    #[test]
    fn test_new_room() {
        let room = Room::new(2, 12, 8, 9, None);
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
        let room = Room::new(2, 12, 8, 9, None);
        let other = Room::new(3, 12, 8, 9, None);
        let third = Room::new(18, 20, 4, 4, None);

        assert!(room.intersects(&other));
        assert!(!room.intersects(&third));
        assert!(other.intersects(&room));
        assert!(!other.intersects(&third));
        assert!(!third.intersects(&other));
        assert!(!third.intersects(&room));
    }

    #[test]
    fn test_macro() {
        let room = room![
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1],
            [1, 1, 1, 1, 1]
        ];
        let another = room![
            [0, 0, 0, 1, 0, 0, 0],
            [0, 0, 1, 1, 1, 0, 0],
            [0, 1, 1, 1, 1, 1, 0],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 0, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [1, 1, 0, 1, 0, 1, 1],
            [1, 1, 1, 1, 1, 1, 1],
            [0, 1, 1, 1, 1, 1, 0],
            [0, 0, 1, 1, 1, 0, 0],
            [0, 0, 0, 1, 0, 0, 0]
        ];

        let walls = room![
            [2, 2, 2, 2, 2],
            [2, 1, 1, 1, 2],
            [2, 1, 0, 1, 2],
            [2, 1, 1, 1, 2],
            [2, 2, 2, 2, 2]
        ];

        assert_eq!(room, vec![
            vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
            vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
            vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable]
        ]);

        assert_eq!(another, vec![
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Walkable, Tile::Empty, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Empty],
            vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
            vec![Tile::Walkable, Tile::Walkable, Tile::Empty, Tile::Walkable, Tile::Empty, Tile::Walkable, Tile::Walkable],
            vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
            vec![Tile::Walkable, Tile::Walkable, Tile::Empty, Tile::Walkable, Tile::Empty, Tile::Walkable, Tile::Walkable],
            vec![Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable],
            vec![Tile::Empty, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Empty, Tile::Empty],
            vec![Tile::Empty, Tile::Empty, Tile::Empty, Tile::Walkable, Tile::Empty, Tile::Empty, Tile::Empty],
        ]);

        assert_eq!(walls, vec![
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
            vec![Tile::Wall, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Wall],
            vec![Tile::Wall, Tile::Walkable, Tile::Empty, Tile::Walkable, Tile::Wall],
            vec![Tile::Wall, Tile::Walkable, Tile::Walkable, Tile::Walkable, Tile::Wall],
            vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall]
        ]);
    }
}
