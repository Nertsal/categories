use geng::prelude::*;

mod game_state;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new("Categories");
    let state = game_state::GameState::new(&geng);

    geng::run(&geng, state);
}
