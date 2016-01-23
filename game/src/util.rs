use allegro::{BitmapLike, MemoryBitmap, SharedBitmap};
use state::Spritesheet;
use std::rc::Rc;

pub fn parse_spritesheet(bmp: MemoryBitmap, tile_width: usize, tile_height: usize, margin: usize)
    -> Result<Spritesheet, ()>
{
    let image = Rc::new(bmp.into_bitmap().clone());
    let map_width = (image.get_width() as usize + margin) / (tile_width + 1);
    let map_height = (image.get_height() as usize + margin) / (tile_height + 1);
    let mut parts = Vec::with_capacity(map_width * map_height);
    for y in 0..map_height {
        for x in 0..map_width {
            let start_x = (x * (tile_width + margin)) as i32;
            let start_y = (y * (tile_height + margin)) as i32;
            let tile = try!(image.create_sub_bitmap(start_x, start_y, tile_width as i32, tile_height as i32));
            parts.push(tile);
        }
    }
    Ok(Spritesheet{
        image: image,
        parts: parts,
    })
}
