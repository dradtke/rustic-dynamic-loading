use allegro::{Bitmap, Color};
use allegro_font::{Font, FontAlign, FontDrawing};
use state::{State, Platform, GameMapDetail, LoadingDetail};
use std::sync::mpsc::{channel, TryRecvError};
use std::thread;
use std::u8;

pub fn update(p: &Platform, detail: &mut LoadingDetail) -> Option<State> {
    if detail.tiles_recv.is_none() {
        let (tx, rx) = channel();
        p.core.spawn(move |core| {
            thread::sleep_ms(2000);
            match Bitmap::load(&core, "../assets/city/Spritesheet/roguelikeCity_magenta.png") {
                Ok(bmp) => match bmp.into_memory_bitmap() {
                    Ok(mem) => tx.send(mem).unwrap(),
                    Err(_) => println!("failed to convert bitmap to memory bitmap"),
                },
                Err(()) => println!("failed to load bitmap"),
            }
        });
        detail.tiles_recv = Some(rx);
    } else if detail.tiles.is_none() {
        match detail.tiles_recv.as_ref().unwrap().try_recv() {
            Ok(bmp) => match ::util::parse_spritesheet(bmp, 16, 16, 1) {
                Ok(tiles) => detail.tiles = Some(tiles),
                Err(()) => println!("failed to parse spritesheet"),
            },
            Err(TryRecvError::Empty) => (/* data's not ready yet */),
            Err(TryRecvError::Disconnected) => (/* nothing else is coming */),
        }
    }

    // Handle the dot animation.
    if detail.dot_timer >= detail.dot_delay {
        if detail.tiles.is_some() {
            // Fuck it, we're done here.
            return Some(State::GameMap(GameMapDetail{ tiles: detail.tiles.take().unwrap() }));
        }
        detail.dot_count = if detail.dot_count < detail.dot_max { detail.dot_count + 1 } else { 0 };
        detail.dot_timer = 0;
    } else {
        detail.dot_timer += 1;
    }

    None
}

pub fn render(p: &Platform, detail: &LoadingDetail) {
    let font = Font::new_builtin(&p.font_addon).unwrap();
    let color = Color::from_rgb(u8::MAX, u8::MAX, u8::MAX);
    let pos = (10.0, 10.0); // (x, y)
    let align = FontAlign::Left;

    let dots = (0..detail.dot_count).map(|_| '.').collect::<String>();
    p.core.draw_text(&font, color, pos.0, pos.1, align, &(detail.base_text.clone() + &dots)[..]);
}
