extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate tiled;

use allegro::{Bitmap, MemoryBitmap, SharedBitmap, SubBitmap};
use std::collections::HashMap;
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

		// TODO: find a more accurate way to make sure all necessary tiles are cached.
		for gid in 0..1035 {
			match self.m.get_tileset_by_gid(gid) {
				Some(tileset) => {
					let (x, y) = tileset.get_orthogonal_tile_coords(gid).unwrap();
					let ref image = tileset.images[0];
					let tile_bmp = bitmaps.get(&image.source).unwrap()
						.create_sub_bitmap(x as i32, y as i32, tileset.tile_width as i32, tileset.tile_height as i32).unwrap();
					tiles.insert(gid, tile_bmp);
					// gid += 1;
				},
				None => (),
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

/// Platform is a collection of the Allegro core and any initialized addons.
#[allow(dead_code)]
pub struct Platform {
    pub core: allegro::Core,
    pub font_addon: allegro_font::FontAddon,
    pub image_addon: allegro_image::ImageAddon,
}
