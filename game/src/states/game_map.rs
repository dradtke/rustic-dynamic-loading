use allegro;
use state::{State, Platform, GameMapDetail};

pub fn update(_: &Platform, _: &mut GameMapDetail) -> Option<State> {
    None
}

pub fn render(p: &Platform, detail: &GameMapDetail) {
	for layer in &detail.map.m.layers {
		// if !layer.visible {
		// 	continue;
		// }
		// TODO: opacity?

		for y in 0..layer.tiles.len() {
			for x in 0..layer.tiles[y].len() {
				let gid = layer.tiles[y][x];
				if let Some(tileset) = detail.map.m.get_tileset_by_gid(gid) {
				    if let Some(bmp) = detail.map.tiles.get(&gid) {
                        p.core.draw_bitmap(bmp, (x as u32 * tileset.tile_width) as f32, (y as u32 * tileset.tile_height) as f32, allegro::FLIP_NONE);
                    }
                }
			}
		}
	}
}
