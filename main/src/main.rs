#[macro_use] extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;
extern crate dylib;
extern crate game;

use allegro_font::FontAddon;
use allegro_image::ImageAddon;
use dylib::DynamicLibrary;
use game::{State, Platform};
use std::fs;
use std::mem;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};

const FPS:           f64 = 60.0;
const SCREEN_WIDTH:  i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

/// A reference to the game's shared library.
enum Handle {
    Open {
        #[allow(dead_code)] lib: DynamicLibrary,
        update: fn(&Platform, State) -> State,
        render: fn(&Platform, &State),
        handle_event: fn(&Platform, State, allegro::Event) -> State,
        clean_up: fn(State),
        inode: u64,
    },
    Closed,
}

impl Handle {
    fn open(path: &Path) -> Option<Handle> {
        match DynamicLibrary::open(Some(path)) {
            Ok(lib) => Some(Handle::Open{
                update: unsafe { mem::transmute(lib.symbol::<usize>("update").unwrap()) },
                render: unsafe { mem::transmute(lib.symbol::<usize>("render").unwrap()) },
                handle_event: unsafe { mem::transmute(lib.symbol::<usize>("handle_event").unwrap()) },
                clean_up: unsafe { mem::transmute(lib.symbol::<usize>("clean_up").unwrap()) },
                lib: lib,
                inode: fs::metadata(path).unwrap().ino(),
            }),
            Err(..) => None,
        }
    }

    fn is_closed(&self) -> bool {
        match *self {
            Handle::Open{..} => false,
            Handle::Closed => true,
        }
    }

    fn close(&mut self) {
        *self = Handle::Closed;
    }

    fn update(&self, p: &Platform, s: State) -> State {
        match *self {
            Handle::Open{update, ..} => update(p, s),
            Handle::Closed => s,
        }
    }

    fn render(&self, p: &Platform, s: &State) {
        match *self {
            Handle::Open{render, ..} => render(p, s),
            Handle::Closed => (),
        }
    }

    fn handle_event(&self, p: &Platform, s: State, e: allegro::Event) -> State {
        match *self {
            Handle::Open{handle_event, ..} => handle_event(p, s, e),
            Handle::Closed => s,
        }
    }

    fn clean_up(&self, s: State) {
        match *self {
            Handle::Open{clean_up, ..} => clean_up(s),
            Handle::Closed => (),
        }
    }
}

// Find the compiled dynamic library for a Cargo project.
//
// Given the relative path to another Cargo project, this method returns
// the path to its compiled dynamic library, if found.
fn find_lib(root: &str) -> Option<PathBuf> {
    /*
    fn is_dylib(entry: &DirEntry) -> bool {
        entry.path().extension().map(|ext| ext == if cfg!(windows) { "dll" } else { "so" }).unwrap_or(false)
    }
    */

    Some(Path::new(root).join("target").join("debug").join("libgame.dylib").to_path_buf())
}

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

    let so = match find_lib("../game") {
        Some(so) => so,
        None => panic!("no shared library found!"),
    };

    let mut handle = Handle::open(so.as_path()).unwrap();
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
            handle.render(&platform, &state);
            platform.core.flip_display();
            redraw = false;
        }

        match q.wait_for_event() {
            allegro::DisplayClose{..} => break 'main,
            allegro::TimerTick{..} => {
                if match handle {
                    Handle::Open{inode, ..} => match fs::metadata(so.as_path()) {
                        Ok(m) => {
                            let new_ino = m.ino();
                            let new_size = m.size();
                            new_ino != inode && new_size > 0
                        },
                        _ => false,
                    },
                    _ => false,
                } {
                    println!("reloading");
                    handle.close();
                }

                if handle.is_closed() {
                    match Handle::open(&Path::new(so.as_path())) {
                        Some(h) => handle = h,
                        _ => (),
                    };
                }
                state = handle.update(&platform, state);
                redraw = true;
            },
            e => {
                state = handle.handle_event(&platform, state, e);
            },
        }
    }

    handle.clean_up(state);
}
