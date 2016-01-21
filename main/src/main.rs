#![feature(fs_walk)]

#[macro_use] extern crate allegro;
extern crate allegro_font;
extern crate state;
extern crate dylib;

use allegro_font::FontAddon;
use dylib::DynamicLibrary;
use state::State;
use std::default::Default;
use std::fs::{self, DirEntry};
use std::mem;
use std::os::linux::fs::MetadataExt;
use std::path::{Path, PathBuf};

const FPS:           f64 = 60.0;
const SCREEN_WIDTH:  i32 = 640;
const SCREEN_HEIGHT: i32 = 480;

/// A reference to the game's shared library.
enum Handle {
    Open {
        #[allow(dead_code)]
        lib: DynamicLibrary,
        update: fn(State) -> State,
        render: fn(&allegro::Core, &FontAddon, &State),
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
                lib: lib,
                inode: fs::metadata(path).unwrap().as_raw_stat().st_ino,
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

    fn update(&self, s: State) -> State {
        match *self {
            Handle::Open{update, ..} => update(s),
            Handle::Closed => s,
        }
    }

    fn render(&self, core: &allegro::Core, font_addon: &FontAddon, s: &State) {
        match *self {
            Handle::Open{render, ..} => render(core, font_addon, s),
            Handle::Closed => (),
        }
    }
}

// Find the compiled dynamic library for a Cargo project.
//
// Given the relative path to another Cargo project, this method returns
// the path to its compiled dynamic library, if found.
fn find_lib(root: &str) -> Option<PathBuf> {
    fn is_dylib(entry: &DirEntry) -> bool {
        entry.path().extension().map(|ext| ext == if cfg!(windows) { "dll" } else { "so" }).unwrap_or(false)
    }

    let p = Path::new(root).join("target").join("debug");
    match fs::walk_dir(&p) {
        Ok(mut iter) => match iter.find(|x| x.as_ref().map(is_dylib).unwrap_or(false)) {
            Some(f) => Some(Path::new(f.unwrap().path().as_path().to_str().unwrap()).to_path_buf()),
            None => None,
        },
        Err(e) => panic!("failed to walk path {}: {}", p.display(), e),
    }
}

allegro_main!
{
    let mut core = allegro::Core::init().unwrap();
    let font_addon = FontAddon::init(&core).unwrap();

    let disp = allegro::Display::new(&core, SCREEN_WIDTH, SCREEN_HEIGHT).unwrap();
    disp.set_window_title("Hello, Allegro!");

    core.install_keyboard().unwrap();
    core.install_mouse().unwrap();

    let so = match find_lib("../game") {
        Some(so) => so,
        None => panic!("no shared library found!"),
    };

    let mut handle = Handle::open(so.as_path()).unwrap();
    let mut state = Default::default();

    let timer = allegro::Timer::new(&core, 1.0 / FPS).unwrap();
    let q = allegro::EventQueue::new(&core).unwrap();
    q.register_event_source(disp.get_event_source());
    q.register_event_source(core.get_keyboard_event_source());
    q.register_event_source(core.get_mouse_event_source());
    q.register_event_source(timer.get_event_source());

    let mut redraw = true;
    timer.start();

    'main: loop {
        if redraw && q.is_empty() {
            handle.render(&core, &font_addon, &state);
            core.flip_display();
            redraw = false;
        }

        match q.wait_for_event() {
            allegro::DisplayClose{..} => break 'main,
            allegro::TimerTick{..} => {
                if match handle {
                    Handle::Open{inode, ..} => match fs::metadata(so.as_path()) {
                        Ok(m) => {
                            let new_ino = m.as_raw_stat().st_ino;
                            let new_size = m.as_raw_stat().st_size;
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
                state = handle.update(state);
                redraw = true;
            },
            _ => (),
        }
    }
}
