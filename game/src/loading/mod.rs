use allegro::Color;
use allegro_font::{Font, FontAlign, FontDrawing};
use state::{State, Platform};
use std::u8;

pub fn update(num_periods: &mut i8, timer: &mut i8, delay: i8) -> Option<State> {
    if *timer >= delay {
        *num_periods = if *num_periods < 3 { *num_periods + 1 } else { 0 };
        *timer = 0;
    } else {
        *timer += 1;
    }
    None
}

pub fn render(p: &Platform, base_text: &String, num_periods: i8) {
    let font = Font::new_builtin(&p.font_addon).unwrap();
    let color = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
    let pos = (10.0, 10.0); // (x, y)
    let align = FontAlign::Left;

    let dots = (0..num_periods).map(|_| '.').collect::<String>();
    p.core.draw_text(&font, color, pos.0, pos.1, align, &(base_text.clone() + &dots)[..]);
}
