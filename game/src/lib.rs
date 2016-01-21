extern crate allegro;
extern crate allegro_font;
extern crate state;

use allegro::Color;
use allegro_font::{Font, FontAlign, FontDrawing};
use state::State;
use std::u8;

#[no_mangle]
#[allow(unused_mut)]
pub fn update(mut s: State) -> State {
    let new_state = match s {
        State::Loading{ ref mut num_periods, ref mut timer, delay, .. } => {
            if *timer >= delay {
                *num_periods = if *num_periods < 3 { *num_periods + 1 } else { 0 };
                *timer = 0;
            } else {
                *timer += 1;
            }
            None
        },
    };
    new_state.unwrap_or(s)
}

#[no_mangle]
#[allow(unused_variables)]
pub fn render(core: &allegro::Core, font_addon: &allegro_font::FontAddon, s: &State) {
    core.clear_to_color(Color::from_rgb(0, 0, 0));
    match *s {
        State::Loading{ ref base_text, num_periods, .. } => {
            let font = Font::new_builtin(font_addon).unwrap();
            let color = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
            let pos = (10.0, 10.0); // (x, y)
            let align = FontAlign::Left;

            let dots = (0..num_periods).map(|_| '.').collect::<String>();
            core.draw_text(&font, color, pos.0, pos.1, align, &(base_text.clone() + &dots)[..]);
        },
    }
}
