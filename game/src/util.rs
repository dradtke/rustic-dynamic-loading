use allegro::{Core, Bitmap};
use state::TiledMemoryMap;
use std::collections::HashMap;
use std::fs::File;
use std::path::{Path, PathBuf};
use tiled;

// TODO: return a Result rather than panic.
pub fn load_map(core: &Core, filename: &str) -> TiledMemoryMap {
	let f = match File::open(filename) {
		Ok(f) => f,
		Err(e) => panic!("failed to open map file {}: {}", filename, e),
	};
	let m = match tiled::parse(f) {
		Ok(m) => m,
		Err(e) => panic!("failed to parse map: {}", e),
	};

	let dir = Path::new(filename).parent().unwrap();

	// Load all of the backing tilesheet images as Allegro memory bitmaps.
	let mut bitmaps = HashMap::new();
	for tileset in &m.tilesets {
		for image in &tileset.images {
			let mut path = PathBuf::from(dir); path.push(&image.source[..]);
			let bmp = match Bitmap::load(core, &path.to_string_lossy()).unwrap().into_memory_bitmap() {
				Ok(x) => x,
				Err(_) => panic!("failed to convert to memory bitmap"),
			};
			bitmaps.insert(image.source.clone(), bmp);
		}
	}

	TiledMemoryMap{m: m, bitmaps: bitmaps}
}
