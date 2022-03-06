use geng::prelude::*;

mod assets;
mod game_state;
mod util;

use assets::*;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Categories".to_owned(),
        max_delta_time: 0.05,
        ..default()
    });
    let assets = <Assets as geng::LoadAsset>::load(&geng, &static_path());

    geng::run(
        &geng,
        geng::LoadingScreen::new(&geng, geng::EmptyLoadingScreen, assets, {
            let geng = geng.clone();
            move |assets| {
                let assets = assets.unwrap();
                game_state::GameState::new(&geng, &Rc::new(assets))
            }
        }),
    );
}
