extern crate cairo;

use super::{ Room, Level };
use std::fs::File;
use self::cairo::{ Context, Format, ImageSurface };

// const IMAGE_WIDTH: i32 = 768;
// const IMAGE_HEIGHT: i32 = 640;
// const SCALE: f64 = 16.0;

fn draw_room(context: &Context, room: &Room, scale: f64) {
    // context.set_source_rgb(room.colour.r, room.colour.g, room.colour.b);
    context.set_source_rgb(1.0, 0.4, 0.2);
    context.new_path();
    context.move_to(room.x1 as f64 * scale, room.y1 as f64 * scale);
    context.line_to(room.x2 as f64 * scale, room.y1 as f64 * scale);
    context.line_to(room.x1 as f64 * scale, room.y2 as f64 * scale);
    context.move_to(room.x2 as f64 * scale, room.y2 as f64 * scale);
    context.line_to(room.x2 as f64 * scale, room.y1 as f64 * scale);
    context.line_to(room.x1 as f64 * scale, room.y2 as f64 * scale);
    context.close_path();
    context.fill();
}

pub fn draw(level: Level, path: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, "rooms");
    let surface = ImageSurface::create(Format::ARgb32, level.width * level.tile_size, level.height * level.tile_size).unwrap();
    let ctx = Context::new(&surface);
    for room in level.rooms {
        draw_room(&ctx, &room, level.tile_size as f64);
    }
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}
