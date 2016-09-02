use allegro::{Color};
use allegro_font::{Font, FontAlign, FontDrawing};
use std::u8;

#[no_mangle]
pub struct Loading {
    pub base_text: String,
    /// The number of dots to display.
    pub dot_count: i8,
    /// The number of dots after which they will be reset to 0.
    pub dot_max: i8,
    /// Number of frames before adding the next dot.
    pub dot_delay: i8,
    /// Incremented once each frame.
    pub dot_timer: i8,

    pub map: Option<::TiledMap>,
}

impl Loading {
    pub fn new(_: &::Platform) -> Loading{
        Loading{
            base_text: String::from("Loading"),
            dot_count: 0,
            dot_max: 3,
            dot_delay: 15,
            dot_timer: 0,
            map: None,
        }
    }

    pub fn render(&self, p: &::Platform) {
        let font = Font::new_builtin(&p.font_addon).unwrap();
        let color = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
        let pos = (10.0, 10.0); // (x, y)
        let align = FontAlign::Left;

        let dots = (0..self.dot_count).map(|_| '.').collect::<String>();
        p.core.draw_text(&font, color, pos.0, pos.1, align, &(String::from("Loading") + &dots)[..]);
    }

    pub fn update(mut self, p: &::Platform) -> ::State {
        if self.map.is_none() {
            self.map = Some(::TiledMap::load(&p.core, "../assets/maps/city.tmx"));
        }

        // Handle the dot animation.
        if self.dot_timer >= self.dot_delay {
            self.dot_count = if self.dot_count < self.dot_max { self.dot_count + 1 } else { 0 };
            self.dot_timer = 0;
            if self.dot_count == 0 && self.map.is_some() {
                // Done loading.
                return ::State::Game(::states::game::Game::new(self.map.unwrap()));
            }
        } else {
            self.dot_timer += 1;
        }

        ::State::Loading(self)
    }
}
