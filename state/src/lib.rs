extern crate allegro;
extern crate allegro_font;

#[no_mangle]
#[derive(Debug)]
pub enum State {
    Loading {
        base_text: String,
        num_periods: i8,
        delay: i8,
        timer: i8,
    },
}

impl Default for State {
    fn default() -> State {
        State::Loading{
            base_text: "Loading".to_string(),
            num_periods: 3,
            delay: 15,
            timer: 0,
        }
    }
}

#[allow(dead_code)]
pub struct Platform {
    pub core: allegro::Core,
    pub font_addon: allegro_font::FontAddon,
}
