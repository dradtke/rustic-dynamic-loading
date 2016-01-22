extern crate allegro;
extern crate allegro_font;
extern crate state;

mod loading;

use allegro::Color;
use state::{Platform, State};

#[no_mangle]
#[allow(unused_mut)]
pub fn update(mut s: State) -> State {
    match s {
        State::Loading{ ref mut num_periods, ref mut timer, delay, .. } => loading::update(num_periods, timer, delay),
    }.unwrap_or(s)
}

#[no_mangle]
#[allow(unused_variables)]
pub fn render(p: &Platform, s: &State) {
    p.core.clear_to_color(Color::from_rgb(0, 0, 0));
    match *s {
        State::Loading{ ref base_text, num_periods, .. } => loading::render(p, base_text, num_periods),
    }
}
