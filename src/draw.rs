extern crate cairo;

use super::Room;
use std::fs::File;
use self::cairo::{ Context, Format, ImageSurface };

const IMAGE_WIDTH: i32 = 768;
const IMAGE_HEIGHT: i32 = 640;
const SCALE: f64 = 16.0;

fn draw_room(context: &Context, room: &Room) {
    // context.set_source_rgb(room.colour.r, room.colour.g, room.colour.b);
    context.set_source_rgb(1.0, 0.4, 0.2);
    context.new_path();
    context.move_to(room.x1 as f64 * SCALE, room.y1 as f64 * SCALE);
    context.line_to(room.x2 as f64 * SCALE, room.y1 as f64 * SCALE);
    context.line_to(room.x1 as f64 * SCALE, room.y2 as f64 * SCALE);
    context.move_to(room.x2 as f64 * SCALE, room.y2 as f64 * SCALE);
    context.line_to(room.x2 as f64 * SCALE, room.y1 as f64 * SCALE);
    context.line_to(room.x1 as f64 * SCALE, room.y2 as f64 * SCALE);
    context.close_path();
    context.fill();
}

pub fn draw(rooms: Vec<Room>, path: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, "rooms");
    let surface = ImageSurface::create(Format::ARgb32, IMAGE_WIDTH, IMAGE_HEIGHT).unwrap();
    let ctx = Context::new(&surface);
    for room in rooms {
        draw_room(&ctx, &room);
    }
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}
