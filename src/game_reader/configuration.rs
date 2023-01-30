use bevy::{ecs::system::Resource};
use crate::game_reader::{
    scene::Scene,
    toml_loader::{TomlAsset,Configuration},
};
use bevy::prelude::*;
use std::{fs, env, path::PathBuf};

#[derive(Resource)]
pub struct GameState {
    pub current_scene: Option<Scene>,
    pub configuration: Option<Configuration>
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_scene: None,
            configuration: None
        }
    }
}

#[derive(Default)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .add_startup_system(setup)
            .add_system( render);
    }
}

fn setup(mut command: Commands, mut state: ResMut<GameState>) {
    command.spawn(Camera2dBundle::default());
    let execpath: PathBuf = {
        if let Ok(ok) = env::current_exe() {
            ok.parent().unwrap().to_path_buf()
        } else {
            "".into()
        }
    };
    let mut file = {
        if let Ok(ok) = fs::read_to_string(execpath.join("core/mods.toml")) {
            ok
        } else {
            "".into()
        }
    };
    state.configuration = {
        if let Ok(ok) = toml::from_str::<TomlAsset>(&file) {
            if let TomlAsset::Configuration(configuration) = ok {
                Some(configuration)
            } else {
                error!("Wrong asset type");
                None
            }
        } else {
            error!("Error loading mods.toml file");
            None
        }
    };
    file = {
        if let Some(some) = &state.configuration {
            if let Ok(ok) = fs::read_to_string(execpath.join("core/data/").join(&some.entry)){
                ok
            } else {
                "".into()
            }
        } else {
            "".into()
        }
    };
    state.current_scene = {
        let res = toml::from_str::<TomlAsset>(&file);
        if let Ok(ok) = res {
            if let TomlAsset::Scene(scene) = ok {
                Some(scene)
            } else {
                error!("Wrong asset type");
                None
            }
        } else {
            if let Err(err) = res {
                error!("Error loading entry scene: {}", err);
            }
            error!("Error loading entry scene");
            None
        }
    };
    
    if let Some(some) = &mut state.current_scene {
        some.start(&mut command);
    }
}

fn render(state: ResMut<GameState>, mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
>,
mut text_query: Query<&mut Text>,) {
    for (interaction, mut background_color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                background_color.0 = crate::ui::ACTIVE;
                println!("Button clicked")
            }
            Interaction::Hovered => {
                background_color.0 = crate::ui::HOVER;
                println!("Button hovered")
            }
            Interaction::None => {
                background_color.0 = crate::ui::NORMAL;
                println!("Button normal");
            }
        }
    }
}