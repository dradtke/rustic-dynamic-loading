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
