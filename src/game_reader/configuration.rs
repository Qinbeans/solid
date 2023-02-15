use bevy::{ecs::system::Resource};
use crate::game_reader::{
    scene::Scene,
    toml_loader::{TomlAsset,Configuration},
};
use bevy::prelude::*;
use std::{fs, env, path::PathBuf, collections::HashMap};

use crate::game_reader::functions::Function;

#[derive(Resource)]
pub struct GameState {
    pub current_scene: Option<Scene>,
    pub configuration: Option<Configuration>,
}

#[derive(Resource, Clone)]
pub struct ActionsState {
    pub module_actions: HashMap<String, Vec<Function>>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            current_scene: None,
            configuration: None,
        }
    }
}

impl Default for ActionsState {
    fn default() -> Self {
        Self {
            module_actions: HashMap::new(),
        }
    }
}

#[derive(Default)]
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .init_resource::<ActionsState>()
            .add_startup_system(setup)
            .add_system( render);
    }
}

fn setup(mut command: Commands, mut state: ResMut<GameState>, mut a_state: ResMut<ActionsState>, asset_server: Res<AssetServer>) {
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
    let tex_map = state.configuration.as_mut().unwrap().texture_map.clone();
    if let Some(some) = &mut state.current_scene {
        some.start(&mut command, &mut a_state, tex_map, &asset_server);
    }
}

fn render(mut interaction_query: Query<
    (&Interaction, &mut BackgroundColor, &Children),
    (Changed<Interaction>, With<Button>),
>,
mut text_query: Query<&mut Text>, state: ResMut<ActionsState>) {
    for (interaction, mut background_color, children) in interaction_query.iter_mut() {
        let text = &text_query.get_mut(children[0]).unwrap().sections[0].value.clone();
        let actions = {
            if let Some(some) = state.module_actions.get(text) {
                some.clone()
            } else {
                Vec::new()
            }
        };
        match *interaction {
            Interaction::Clicked => {
                background_color.0 = crate::ui::ACTIVE;
                for action in actions {
                   match action.name.as_str() {
                       "exit" => {
                           std::process::exit(0);
                       }
                       _ => {}
                   }
                }
            }
            Interaction::Hovered => {
                background_color.0 = crate::ui::HOVER;
            }
            Interaction::None => {
                background_color.0 = crate::ui::NORMAL;
            }
        }
    }
}