extern crate allegro;
extern crate allegro_font;
extern crate state;
extern crate tiled;

mod states;
mod util;

use allegro::Color;
use state::{Platform, State};
use std::mem;

#[no_mangle]
pub fn update(p: &Platform, mut s: State) -> State {
    match match s {
        State::Loading(ref mut detail) => states::loading::update(p, detail),
        State::GameMap(ref mut detail) => states::game_map::update(p, detail),
    } {
        // The old state is technically owned by the main binary, so it needs
        // to be forgotten here in order to prevent a segfault.
        Some(x) => { mem::forget(s); x},
        None => s,
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub fn render(p: &Platform, s: &State) {
    p.core.clear_to_color(Color::from_rgb(0, 0, 0));
    match *s {
        State::Loading(ref detail) => states::loading::render(p, detail),
        State::GameMap(ref detail) => states::game_map::render(p, detail),
    }
}

#[no_mangle]
#[allow(unused_variables)]
pub fn handle_event(p: &Platform, s: &State, e: allegro::Event) {
    match *s {
        State::GameMap(ref detail) => states::game_map::handle_event(p, detail, e),
        _ => (),
    }
}

#[no_mangle]
pub fn clean_up(s: State) {
    mem::forget(s);
}
