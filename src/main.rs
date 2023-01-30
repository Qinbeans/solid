use bevy::prelude::*;
use window::game::{GamePlugin};
use game_reader::configuration::{ScenePlugin};

mod window;
mod game_reader;
mod ui;

#[tokio::main]
async fn main() -> Result<(),()> {
    App::new()
        .add_plugin(GamePlugin::default())
        .add_plugin(ScenePlugin::default())
        .run();
    Ok(())
}
