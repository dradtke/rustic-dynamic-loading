extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate tiled;

use allegro::{Bitmap, MemoryBitmap, SharedBitmap, SubBitmap};
use std::collections::HashMap;
use std::collections::hash_map::{Entry};
use std::rc::Rc;
use std::sync::mpsc::Receiver;

#[no_mangle]
pub enum State {
    Loading(LoadingDetail),
    GameMap(GameMapDetail),
}

impl Default for State {
    fn default() -> State {
        State::Loading(LoadingDetail::default())
    }
}

pub struct LoadingDetail {
    pub initialized: bool,

    pub base_text: String,
    /// The number of dots to display.
    pub dot_count: i8,
    /// The number of dots after which they will be reset to 0.
    pub dot_max: i8,
    /// Number of frames before adding the next dot.
    pub dot_delay: i8,
    /// Incremented once each frame.
    pub dot_timer: i8,

    pub map_recv: Option<Receiver<TiledMemoryMap>>,
    pub map: Option<TiledMap>,
}

impl Default for LoadingDetail {
    fn default() -> LoadingDetail {
        LoadingDetail{
            initialized: false,
            base_text: String::from("Loading"),
            dot_count: 0,
            dot_max: 3,
            dot_delay: 15,
            dot_timer: 0,
            map_recv: None,
            map: None,
        }
    }
}

#[no_mangle]
pub struct GameMapDetail {
	pub map: TiledMap,
}

/// Representation of an in-memory Tiled map. This is
/// usually created in another thread, then sent to
/// the main one.
pub struct TiledMemoryMap {
	pub m: tiled::Map,
	pub bitmaps: HashMap<String, MemoryBitmap>,
}

impl TiledMemoryMap {
	pub fn into_map(self) -> TiledMap {
		let mut bitmaps = HashMap::new();
		for (filename, mbmp) in self.bitmaps {
			bitmaps.insert(filename, Rc::new(mbmp.into_bitmap().clone()));
		}

		let mut tiles = HashMap::new();
        for layer in &self.m.layers {
            for y in 0..layer.tiles.len() {
                for x in 0..layer.tiles[y].len() {
                    let gid = layer.tiles[y][x];
                    if let Entry::Vacant(entry) = tiles.entry(gid) {
                        if let Some(tileset) = self.m.get_tileset_by_gid(gid) {
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

		TiledMap{ m: self.m, bitmaps: bitmaps, tiles: tiles }
	}
}

pub struct TiledMap {
	pub m: tiled::Map,
	pub bitmaps: HashMap<String, Rc<Bitmap>>,
    pub tiles: HashMap<u32, SubBitmap>,
}

impl TiledMap {
	pub fn render(&self, p: &Platform) {
        for layer in &self.m.layers {
            if !layer.visible {
                continue;
            }
            // TODO: opacity?

            for y in 0..layer.tiles.len() {
                for x in 0..layer.tiles[y].len() {
                    let gid = layer.tiles[y][x];
                    if let Some(tileset) = self.m.get_tileset_by_gid(gid) {
                        if let Some(bmp) = self.tiles.get(&gid) {
                            p.core.draw_bitmap(bmp, (x as u32 * tileset.tile_width) as f32, (y as u32 * tileset.tile_height) as f32, allegro::FLIP_NONE);
                        }
                    }
                }
            }
        }
    }
}

/// Platform is a collection of the Allegro core and any initialized addons.
#[allow(dead_code)]
pub struct Platform {
    pub core: allegro::Core,
    pub font_addon: allegro_font::FontAddon,
    pub image_addon: allegro_image::ImageAddon,
}
