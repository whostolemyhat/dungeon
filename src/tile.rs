use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tile {
    Empty,
    Walkable,
    Wall,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tile::Empty => write!(f, " "),
            Tile::Walkable => write!(f, "1"),
            Tile::Wall => write!(f, "2"),
        }
    }
}

impl Serialize for Tile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Tile::Empty => serializer.serialize_i32(0),
            Tile::Walkable => serializer.serialize_i32(1),
            Tile::Wall => serializer.serialize_i32(2),
        }
    }
}

impl<'de> Deserialize<'de> for Tile {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = i32::deserialize(deserializer)?;
        let tile = match s {
            0 => Tile::Empty,
            1 => Tile::Walkable,
            2 => Tile::Wall,
            _ => Tile::Empty,
        };
        Ok(tile)
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::Tile;
    use serde_json;

    #[test]
    fn test_tile_display() {
        assert_eq!(format!("{}", Tile::Empty), " ");
        assert_eq!(format!("{}", Tile::Walkable), "1");
    }

    #[test]
    fn test_tile_serialise() {
        assert_eq!(serde_json::to_string(&Tile::Empty).unwrap(), "0");
        assert_eq!(serde_json::to_string(&Tile::Walkable).unwrap(), "1");
    }
}
