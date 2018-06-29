extern crate cairo;

use level::{ Level, Tile };
use std::fs::File;
use self::cairo::{ Context, Format, ImageSurface };

fn draw_tile(context: &Context, x: f64, y: f64, x2: f64, y2: f64) {
    context.set_source_rgb(1.0, 0.4, 0.2);
    context.new_path();
    context.move_to(x as f64, y as f64);
    context.line_to(x2 as f64, y as f64);
    context.line_to(x as f64, y2 as f64);
    context.move_to(x2 as f64, y2 as f64);
    context.line_to(x2 as f64, y as f64);
    context.line_to(x as f64, y2 as f64);
    context.close_path();
    context.fill();
}

fn draw_tiles(context: &Context, board: &Vec<Tile>, scale: f64, width: i32) {
    let mut col = 0;
    let mut row = 0;

    for tile in board {
        col = col + 1;
        if col >= width {
            col = 0;
            row = row + 1;
        }

        match tile {
            Tile::Walkable => draw_tile(context, col as f64 * scale, row as f64 * scale, col as f64 * scale + scale, row as f64 * scale + scale),
            _ => ()
        }
    }
}

pub fn draw(level: &Level, path: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, level.hash);
    let surface = ImageSurface::create(Format::ARgb32, level.width * level.tile_size, level.height * level.tile_size).unwrap();
    let ctx = Context::new(&surface);

    draw_tiles(&ctx, &level.board, level.tile_size as f64, level.width);
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}
