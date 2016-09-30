#[macro_use] extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate libloading;
extern crate game;

use allegro_font::FontAddon;
use allegro_image::ImageAddon;
use libloading::Library;
use game::{State, Platform};
use std::fs;

const FPS:           f64 = 60.0;
const SCREEN_WIDTH:  i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

struct Application(Library);

impl Application {
    fn update(&self, p: &Platform, s: State) -> State {
        unsafe {
            let f = self.0.get::<fn(&Platform, State) -> State>(b"update\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `update`: {}", error));
            f(p, s)
        }
    }

    fn render(&self, p: &Platform, s: &State) {
        unsafe {
            let f = self.0.get::<fn(&Platform, &State)>(b"render\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `render`: {}", error));
            f(p, s)
        }
    }

    fn handle_event(&self, p: &Platform, s: State, e: allegro::Event) -> State {
        unsafe {
            let f = self.0.get::<fn(&Platform, State, allegro::Event) -> State>(b"handle_event\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `handle_event`: {}", error));
            f(p, s, e)
        }
    }

    fn clean_up(&self, s: State) {
        unsafe {
            let f = self.0.get::<fn(State)>(b"clean_up\0")
                .unwrap_or_else(|error| panic!("failed to get symbol `clean_up`: {}", error));
            f(s)
        }
    }
}

const LIB_PATH: &'static str = "../game/target/debug/libgame.dylib";

allegro_main!
{
    let core = allegro::Core::init().unwrap();
    let mut platform = Platform {
        font_addon: FontAddon::init(&core).unwrap_or_else(|msg| panic!(msg)),
        image_addon: ImageAddon::init(&core).unwrap_or_else(|msg| panic!(msg)),
        core: core,
    };

    let disp = allegro::Display::new(&platform.core, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    disp.set_window_title("Hello, Allegro!");

    platform.core.install_keyboard().unwrap();
    platform.core.install_mouse().unwrap();

    let mut app = Application(Library::new(LIB_PATH).unwrap());
    let mut last_modified = fs::metadata(LIB_PATH).unwrap().modified().unwrap();
    let mut state = game::State::new(&platform);

    let timer = allegro::Timer::new(&platform.core, 1.0 / FPS).unwrap();
    let q = allegro::EventQueue::new(&platform.core).unwrap();
    q.register_event_source(disp.get_event_source());
    q.register_event_source(platform.core.get_keyboard_event_source());
    q.register_event_source(platform.core.get_mouse_event_source());
    q.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();

    'main: loop {
        if redraw && q.is_empty() {
            app.render(&platform, &state);
            platform.core.flip_display();
            redraw = false;
        }

        match q.wait_for_event() {
            allegro::DisplayClose{..} => break 'main,
            allegro::TimerTick{..} => {
                if let Ok(Ok(modified)) = fs::metadata(LIB_PATH).map(|m| m.modified()) {
                    if modified > last_modified {
                        drop(app);
                        app = Application(Library::new(LIB_PATH)
                                          .unwrap_or_else(|error| panic!("{}", error)));
                        last_modified = modified;
                    }
                }
                state = app.update(&platform, state);
                redraw = true;
            },
            e => {
                state = app.handle_event(&platform, state, e);
            },
        }
    }

    println!("Cleaning up...");
    app.clean_up(state);
    //mem::forget(state);
    println!("Bye!");
}
