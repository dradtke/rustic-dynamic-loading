use allegro;
use state::{State, Platform, GameMapDetail};
use std::collections::HashSet;

pub fn update(_: &Platform, _: &mut GameMapDetail) -> Option<State> {
    None
}

pub fn render(p: &Platform, detail: &GameMapDetail) {
	let mut used_tiles = HashSet::new();

	for layer in &detail.map.m.layers {
		// if !layer.visible {
		// 	continue;
		// }
		// TODO: opacity?

		for y in 0..layer.tiles.len() {
			for x in 0..layer.tiles[0].len() {
				let gid = layer.tiles[y][x];
				used_tiles.insert(gid);
				let tileset = detail.map.m.get_tileset_by_gid(gid).unwrap();
				match detail.map.tiles.get(&gid) {
					Some(bmp) => {
						p.core.draw_bitmap(bmp, (x as u32 * tileset.tile_width) as f32, (y as u32 * tileset.tile_height) as f32, allegro::FLIP_NONE);
					},
					None => (),
				}
			}
		}
	}
}
