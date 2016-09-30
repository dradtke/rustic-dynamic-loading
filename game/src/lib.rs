extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate tiled;

mod states;

use allegro::{Bitmap, Color, Core, SharedBitmap, SubBitmap};
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::fs::File;
use std::mem;
use std::rc::Rc;
use std::path::{Path, PathBuf};

#[no_mangle]
pub fn update(p: &Platform, s: State) -> State {
    s.update(p)
}

#[no_mangle]
#[allow(unused_variables)]
pub fn render(p: &Platform, s: &State) {
    p.core.clear_to_color(Color::from_rgb(0, 0, 0));
    s.render(p);
}

#[no_mangle]
#[allow(unused_variables)]
pub fn handle_event(p: &Platform, s: State, e: allegro::Event) -> State {
    s.handle_event(p, e)
}

#[no_mangle]
pub fn clean_up(mut s: State) {
    s.clean_up();
    mem::forget(s);
}

#[allow(dead_code)]
pub struct Platform {
    pub core: allegro::Core,
    pub font_addon: allegro_font::FontAddon,
    pub image_addon: allegro_image::ImageAddon,
}

pub enum State {
    Loading(states::loading::Loading),
    Game(states::game::Game),
}

impl State {
    pub fn new(p: &Platform) -> State {
        State::Loading(states::loading::Loading::new(p))
    }

    fn update(self, p: &Platform) -> State {
        match self {
            State::Loading(x) => x.update(p),
            State::Game(x) => x.update(p),
        }
    }

    fn render(&self, p: &Platform) {
        match *self {
            State::Loading(ref x) => x.render(p),
            State::Game(ref x) => x.render(p),
        }
    }

    fn handle_event(self, p: &Platform, e: allegro::Event) -> State {
        match self {
            State::Loading(x) => State::Loading(x),
            State::Game(x) => x.handle_event(p, e),
        }
    }

    fn clean_up(&mut self) {
        match *self {
            State::Loading(_) => (),
            State::Game(ref mut x) => { x.clean_up(); mem::forget(x) },
        }
    }
}

pub struct TiledMap {
	pub m: tiled::Map,
	pub bitmaps: HashMap<String, Rc<Bitmap>>,
    pub tiles: HashMap<u32, SubBitmap>,
}

impl TiledMap {
    pub fn load<P: AsRef<Path>>(core: &Core, filename: P) -> TiledMap {
        let f = match File::open(filename.as_ref()) {
            Ok(f) => f,
            Err(e) => panic!("failed to open map file: {}", e),
        };
        let m = match tiled::parse(f) {
            Ok(m) => m,
            Err(e) => panic!("failed to parse map: {}", e),
        };

        let dir = filename.as_ref().parent().unwrap();

        // Load all of the backing tilesheet images as Allegro memory bitmaps.
        let mut bitmaps = HashMap::new();
        for tileset in &m.tilesets {
            for image in &tileset.images {
                let mut path = PathBuf::from(dir); path.push(&image.source[..]);
                let bmp = match Bitmap::load(core, &path.to_string_lossy()) {
                    Ok(x) => x,
                    Err(e) => panic!("failed to load bitmap: {:?}", e),
                };
                bitmaps.insert(image.source.clone(), Rc::new(bmp));
            }
        }

		let mut tiles = HashMap::new();
        for layer in &m.layers {
            for y in 0..layer.tiles.len() {
                for x in 0..layer.tiles[y].len() {
                    let gid = layer.tiles[y][x];
                    if let Entry::Vacant(entry) = tiles.entry(gid) {
                        if let Some(tileset) = m.get_tileset_by_gid(gid) {
                            let image = &tileset.images[0];
                            let relative_gid = gid - tileset.first_gid;
                            let tiles_per_row = ((image.width as u32) + tileset.spacing) / (tileset.tile_width + tileset.spacing);
                            let ty = relative_gid / tiles_per_row;
                            let tx = relative_gid - (ty * tiles_per_row);
                            entry.insert(bitmaps.get(&image.source).unwrap()
                                         .create_sub_bitmap(
                                             ((tx * (tileset.tile_width + tileset.spacing)) + tileset.margin) as i32,
                                             ((ty * (tileset.tile_height + tileset.spacing)) + tileset.margin) as i32,
                                             tileset.tile_width as i32,
                                             tileset.tile_height as i32,
                                         ).unwrap());
                        }
                    }
                }
            }
        }

		TiledMap{ m: m, bitmaps: bitmaps, tiles: tiles }
    }

	pub fn render(&self, p: &Platform, dx: i32, dy: i32) {
        for layer in &self.m.layers {
            if !layer.visible {
                continue;
            }

            for ty in 0..layer.tiles.len() {
                for tx in 0..layer.tiles[ty].len() {
                    let gid = layer.tiles[ty][tx];
                    if let Some(tileset) = self.m.get_tileset_by_gid(gid) {
                        if let Some(bmp) = self.tiles.get(&gid) {
                            let x = ((tx as i32 * tileset.tile_width as i32) - dx) as f32;
                            let y = ((ty as i32 * tileset.tile_height as i32) - dy) as f32;
                            if layer.opacity == 1.00 {
                                p.core.draw_bitmap(bmp, x, y, allegro::FLIP_NONE);
                            } else {
                                p.core.draw_tinted_bitmap(bmp, Color::from_rgba_f(layer.opacity, layer.opacity, layer.opacity, layer.opacity), x, y, allegro::FLIP_NONE);
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Drop for TiledMap {
    fn drop(&mut self) {
        println!("Dropping a map");
    }
}
