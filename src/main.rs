use geng::prelude::*;

mod assets;
mod game_state;

use assets::*;

fn main() {
    logger::init().unwrap();
    geng::setup_panic_handler();

    // Setup working directory
    if let Some(dir) = std::env::var_os("CARGO_MANIFEST_DIR") {
        std::env::set_current_dir(std::path::Path::new(&dir).join("static")).unwrap();
    } else {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = std::env::current_exe().unwrap().parent() {
                std::env::set_current_dir(path).unwrap();
            }
        }
    }

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Categories".to_owned(),
        max_delta_time: 0.05,
        ..default()
    });
    let assets = <Assets as geng::LoadAsset>::load(&geng, ".");

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
