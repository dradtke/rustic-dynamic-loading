use allegro;
use state::{State, Platform, GameMapDetail};

pub fn update(_: &Platform, _: &mut GameMapDetail) -> Option<State> {
    None
}

pub fn render(p: &Platform, detail: &GameMapDetail) {
    detail.map.render(p);
}

pub fn handle_event(p: &Platform, detail: &GameMapDetail, e: allegro::Event) {
    match e {
        allegro::KeyDown{keycode, ..} => {
            println!("Handling keypress for {}!", p.core.keycode_to_name(keycode));
        },
        _ => (),
    }
}
