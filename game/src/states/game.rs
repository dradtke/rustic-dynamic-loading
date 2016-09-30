use allegro::{Event, KeyCode};
use std::vec::Vec;

#[no_mangle]
pub struct Game {
	map: ::TiledMap,
	reload_map: bool,
	xmap: i32,
	ymap: i32,
	keys_pressed: Vec<KeyCode>,
}

impl Game {
    pub fn new(map: ::TiledMap) -> Game {
        Game{map: map, reload_map: false, xmap: 0, ymap: 0, keys_pressed: vec![]}
    }

    pub fn update(mut self, p: &::Platform) -> ::State {
        if self.reload_map {
            self.do_map_reload(p);
            self.reload_map = false;
        }
        if self.keys_pressed.contains(&KeyCode::Up) {
            self.ymap -= 5;
        } else if self.keys_pressed.contains(&KeyCode::Down) {
            self.ymap += 5;
        } else if self.keys_pressed.contains(&KeyCode::Left) {
            self.xmap -= 5;
        } else if self.keys_pressed.contains(&KeyCode::Right) {
            self.xmap += 5;
        }
        ::State::Game(self)
    }

    pub fn render(&self, p: &::Platform) {
        self.map.render(p, self.xmap, self.ymap);
    }

    pub fn handle_event(mut self, _: &::Platform, e: Event) -> ::State {
        match e {
            Event::KeyDown{keycode, ..} => {
                self.keys_pressed.push(keycode);
                if keycode == KeyCode::Space {
                    self.reload_map = true;
                }
            },
            Event::KeyUp{keycode, ..} => {
                self.keys_pressed.retain(|k| *k != keycode);
            },
            _ => (),
        }
        ::State::Game(self)
    }

    pub fn clean_up(&mut self) {
    }

    fn do_map_reload(&mut self, p: &::Platform) {
        println!("reloading map");
        self.map = ::TiledMap::load(&p.core, "../assets/maps/city.tmx");
    }
}

impl Drop for Game {
    fn drop(&mut self) {
        println!("Dropping Game state");
    }
}
