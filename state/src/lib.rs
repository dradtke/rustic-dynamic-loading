extern crate allegro;
extern crate allegro_font;
extern crate allegro_image;

use allegro::{Bitmap, MemoryBitmap, SubBitmap};
use std::rc::Rc;
use std::sync::mpsc::Receiver;

#[no_mangle]
pub enum State {
    Loading(LoadingDetail),
    GameMap(GameMapDetail),
}

impl Default for State {
    fn default() -> State {
        State::Loading(LoadingDetail::default())
    }
}

pub struct LoadingDetail {
    pub initialized: bool,

    pub base_text: String,
    /// The number of dots to display.
    pub dot_count: i8,
    /// The number of dots after which they will be reset to 0.
    pub dot_max: i8,
    /// Number of frames before adding the next dot.
    pub dot_delay: i8,
    /// Incremented once each frame.
    pub dot_timer: i8,

    pub tiles_recv: Option<Receiver<MemoryBitmap>>,
    pub tiles: Option<Spritesheet>,
}

impl Default for LoadingDetail {
    fn default() -> LoadingDetail {
        LoadingDetail{
            initialized: false,
            base_text: String::from("Loading"),
            dot_count: 0,
            dot_max: 3,
            dot_delay: 15,
            dot_timer: 0,
            tiles_recv: None,
            tiles: None,
        }
    }
}

#[no_mangle]
pub struct GameMapDetail {
    pub tiles: Spritesheet,
}

/// The backing bitmap needs to be here so that it's not lost
/// when this object is passed between states.
pub struct Spritesheet {
    pub image: Rc<Bitmap>,
    pub parts: Vec<SubBitmap>,
}

/// Platform is a collection of the Allegro core and any initialized addons.
#[allow(dead_code)]
pub struct Platform {
    pub core: allegro::Core,
    pub font_addon: allegro_font::FontAddon,
    pub image_addon: allegro_image::ImageAddon,
}
