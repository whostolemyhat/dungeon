use crate::level::Level;
use crate::tile::Tile;
use cairo::{Context, Format, ImageSurface};
use std::fs::File;

fn draw_tile(context: &Context, x: f64, y: f64, x2: f64, y2: f64, colour: (f64, f64, f64)) {
    // context.set_source_rgb(0.258, 0.525, 0.956);
    context.set_source_rgb(colour.0, colour.1, colour.2);
    context.new_path();
    context.move_to(x, y);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.move_to(x2, y2);
    context.line_to(x2, y);
    context.line_to(x, y2);
    context.close_path();
    context.fill().expect("Failed to fill context");
}

fn draw_tiles(context: &Context, board: &[Vec<Tile>], scale: f64) {
    for (row, line) in board.iter().enumerate() {
        for (col, tile) in line.iter().enumerate() {
            match tile {
                Tile::Walkable => draw_tile(
                    context,
                    col as f64 * scale,
                    row as f64 * scale,
                    col as f64 * scale + scale,
                    row as f64 * scale + scale,
                    (0.258, 0.525, 0.956),
                ),
                Tile::Wall => draw_tile(
                    context,
                    col as f64 * scale,
                    row as f64 * scale,
                    col as f64 * scale + scale,
                    row as f64 * scale + scale,
                    (0.956, 0.525, 0.258),
                ),
                _ => (),
            }
        }
    }
}

pub fn draw(level: &Level, path: &str, img_name: &str) -> Result<(), ::std::io::Error> {
    let default_output = format!("{}/{}.png", path, img_name);
    let surface = ImageSurface::create(
        Format::ARgb32,
        level.width * level.tile_size,
        level.height * level.tile_size,
    )
    .unwrap();
    let ctx = Context::new(&surface).expect("failed to create Cairo surface");

    draw_tiles(&ctx, &level.board, level.tile_size as f64);
    let mut file = File::create(default_output)?;
    surface.write_to_png(&mut file).unwrap();

    Ok(())
}
