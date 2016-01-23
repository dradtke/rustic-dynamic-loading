use allegro;
use state::{State, Platform, GameMapDetail};

pub fn update(_: &Platform, _: &mut GameMapDetail) -> Option<State> {
    None
}

pub fn render(p: &Platform, detail: &GameMapDetail) {
    for y in 0..28 {
        for x in 0..37 {
            p.core.draw_bitmap(&detail.tiles.parts[(y * 37) + x], (x * 16) as f32, (y * 16) as f32, allegro::FLIP_NONE);
        }
    }
}
