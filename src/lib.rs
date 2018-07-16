extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

pub mod draw;
pub mod tile;
pub mod level;
pub mod room;
pub mod roomscorridors;
pub mod bsp;


// // drunkards walk
// // bresenhams line algorithm
// // non-rectangular rooms
// // quadtree
// // grid (gen on top + pick random direction)
// // cellular automata https://gamedevelopment.tutsplus.com/tutorials/generate-random-cave-levels-using-cellular-automata--gamedev-9664
// // bsp https://gamedevelopment.tutsplus.com/tutorials/how-to-use-bsp-trees-to-generate-game-maps--gamedev-12268