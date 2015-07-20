extern crate allegro;
extern crate state;

use allegro as al;
use state::State;
use std::u8::MAX;

#[no_mangle]
#[allow(unused_mut)]
pub fn update(mut s: State) -> State {
    if s.r < MAX {
        s.r += 1;
    }
    s
}

#[no_mangle]
#[allow(unused_variables)]
pub fn render(core: &al::Core, s: &State) {
    core.clear_to_color(core.map_rgb(s.r, s.g, s.b));
}
