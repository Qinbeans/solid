use bevy::{
    ecs::system::Res,
    prelude::{
        error,
        Commands,
        TextBundle,
        SpriteBundle,
        Color,
        BuildChildren,
        JustifyContent,
        AlignItems,
        info,
        AssetServer,
        NodeBundle,
        Vec2
    },
    ui::{
        ZIndex::Global,
        ZIndex::Local,
        Size,
        Val::Percent,
        Val::Px,
        PositionType::Absolute,
        PositionType::Relative,
        UiRect,
        Style,
        node_bundles::ButtonBundle
    },
    text::{TextStyle}, sprite::Sprite
};
use serde::{Deserialize, Serialize};
use crate::{
    game_reader::{
        location::Location,
        functions::{
            Function,
            Value,
            Parameter
        },
    },
    ui::component::UIType
};
use std::fmt::Display;
use crate::game_reader::functions::{Vector3D, Vector2D, Vector4D};
use std::{env, path::PathBuf};

use super::toml_loader::TextureMap;

//map of functions
const SCENE_FUNCTIONS: &[(&str, fn(&Description, &mut Commands, Option<NodeBundle>, &Res<AssetServer>, Vec<Parameter>))] = &[
    ("spawn_ui", Description::spawn_ui),
    ("create_node", Description::create_node),
    ("draw_texture", Description::draw_texture),
    ("spawn_text", Description::spawn_text)
];

//a plugin that loads the game data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scene {
    pub name: String,
    pub description: Description,
    #[serde(skip_serializing, skip_deserializing)]
    pub started: bool
}

impl Scene {
    pub fn start(&mut self, commands: &mut Commands, texture_map: TextureMap, asset_server: &Res<AssetServer>) {
        self.description.texture_map = texture_map;
        self.description.start(commands, asset_server);
    }
    #[allow(dead_code)]
    pub fn render(&mut self, commands: &mut Commands) {
        self.description.render(commands);
    }
}

impl Display for Scene {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Scene {{ name: {}, description: {:?} }}", self.name, self.description)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Description {
    pub background: Vector4D,
    pub locations: Vec<Location>,
    pub spawn: Option<Location>,
    pub actions: Vec<Function>,
    #[serde(skip_serializing, skip_deserializing)]
    #[allow(dead_code)]
    pub results: Vec<Value>,
    #[serde(skip_serializing, skip_deserializing)]
    pub texture_map: TextureMap,
}
impl Description {
    fn parse_button(&self, params: Vec<Parameter>) -> Result<ButtonBundle,&str> {
        //spawn button
        let mut button_bundle = ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            ..Default::default()
        };
        if params.get(1).is_none(){
            return Err("No position");
        }
        let pos = match params.get(1).unwrap(){
            Parameter::Position(v) => v,
            _ => &Vector3D{x: 0.0, y: 0.0, z: 0.0}
        };
        button_bundle.style.position_type = Absolute;
        button_bundle.style.position = UiRect {
            left: Percent(pos.x as f32),
            top: Percent(pos.y as f32),
            ..Default::default()
        };
        //use z to bring to front
        button_bundle.z_index = Local(pos.z as i32);
        if params.get(2).is_none(){
            return Err("No dimensions");
        }
        let dims = match params.get(2).unwrap(){
            Parameter::Size(v) => v,
            _ => &Vector2D{x: 0.0, y: 0.0}
        };
        button_bundle.style.size = Size::new(Percent(dims.x as f32), Percent(dims.y as f32));
        if params.get(2).is_none(){
            return Err("No text");
        }
        button_bundle.background_color = crate::ui::NORMAL.into();
        Ok(button_bundle)
    }
    
    fn parse_text(&self, last_index: usize, params: Vec<Parameter>, asset_server: &Res<AssetServer>) -> Result<TextBundle, &str> {
        if params.get(last_index + 1).is_none(){
            return Err("No font size");
        }
        let font = match params.get(last_index + 1).unwrap(){
            Parameter::Font(i) => *i as f32,
            _ => 0.0
        };
        if font <= 0.0 {
            return Err("Invalid font size");
        }
        let text = params.get(last_index + 2);
        if text.is_none(){
            return Err("No text");
        }
        let text = match text.unwrap(){
            Parameter::Text(s) => s,
            _ => "",
        };
        if params.get(last_index + 3).is_none(){
            return Err("No font color");
        }
        let color = match params.get(last_index + 3).unwrap(){
            Parameter::Color(v) => v,
            _ => &Vector4D{x: 1.0, y: 1.0, z: 1.0, w: 1.0},
        };
        let execpath: PathBuf = {
            if let Ok(ok) = env::current_exe() {
                ok.parent().unwrap().to_path_buf()
            } else {
                "".into()
            }
        };
        let file = execpath.join("core/assets/fonts/FiraSans-Bold.ttf");
        info!("Text: {}", text);
        Ok(TextBundle::from_section(
            text, 
            TextStyle {
                font: asset_server.load(file.to_str().unwrap()),
                font_size: font,
                color: Color::rgba(color.x as f32, color.y as f32, color.z as f32, color.w as f32),
                ..Default::default()
            }
        ))
    }

    pub fn spawn_ui(&self, command: &mut Commands, node: Option<NodeBundle>, asset_server: &Res<AssetServer>, params: Vec<Parameter>) {
        //spawns a UI component
        //first index is type
        if let Some(param1) = params.get(0){
            let value = match param1{
                Parameter::Ui(s) => s,
                _ => "",
            };
            let tp = UIType::from_str(&value);
            if tp.is_none(){
                return;
            }
            let tp = tp.unwrap();
            match tp {
                UIType::Button => {
                    //spawn button
                    let button_bundle = self.parse_button(params.clone());
                    if button_bundle.is_err(){
                        return;
                    }
                    let button_bundle = button_bundle.unwrap();
                    let text_bundle = self.parse_text(2, params, asset_server);
                    if text_bundle.is_err(){
                        return;
                    }
                    let text_bundle = text_bundle.unwrap();
                    if node.is_none(){
                        command.spawn(button_bundle).with_children(|parent|{
                            parent.spawn(text_bundle);
                        });
                    } else {
                        let node = node.unwrap();
                        command.spawn(node).with_children(|parent|{
                            parent.spawn(button_bundle).with_children(|parent|{
                                parent.spawn(text_bundle);
                            });
                        });
                    }
                },
                UIType::Text => {
                    //spawn text
                },
                UIType::Slider => {
                    //spawn slider
                },
                UIType::Input => {
                    //spawn input
                },
                UIType::Checkbox => {
                    //spawn checkbox
                },
                UIType::Dropdown => {
                    //spawn dropdown
                },
                UIType::Radio => {
                    //spawn radio
                },
                UIType::List => {
                    //spawn list
                },
                UIType::Table => {
                    //spawn table
                }
            }
        }
    }

    pub fn create_node(&self, command: &mut Commands, _node: Option<NodeBundle>, asset_server: &Res<AssetServer>, params: Vec<Parameter>) {
        //get id
        if params.get(0).is_none(){
            return;
        }
        let pos = match params.get(0).unwrap(){
            Parameter::Position(v) => v,
            _ => &Vector3D{x: 0.0, y: 0.0, z: 0.0}
        };
        //size
        if params.get(1).is_none(){
            return;
        }
        let size = match params.get(1).unwrap(){
            Parameter::Size(v) => v,
            _ => &Vector2D{x: 0.0, y: 0.0}
        };
        if params.get(2).is_none(){
            return;
        }
        let actions = match params.get(2).unwrap(){
            Parameter::Actions(v) => v.clone(),
            _ => Vec::new()
        };
        let node = NodeBundle {
            style: Style {
                size: Size::new(Percent(size.x as f32), Percent(size.y as f32)),
                position_type: Relative,
                position: UiRect {
                    left: Px(pos.x as f32),
                    top: Px(pos.y as f32),
                    ..Default::default()
                },
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            z_index: Global(pos.z as i32),
            ..Default::default()
        };
        for action in actions {
            let params = action.parameters.clone();
            let name = action.name.clone();
            //check if function is in the map
            if let Some((_, func)) = SCENE_FUNCTIONS.iter().find(|(n,_)| n == &name){
                func(self, command, Some(node.clone()), asset_server, params);
            }
        }
    }

    pub fn draw_texture(&self, command: &mut Commands, node: Option<NodeBundle>, asset_server: &Res<AssetServer>, params: Vec<Parameter>) {
        //texture
        if params.get(0).is_none(){
            return;
        }
        let execpath: PathBuf = {
            if let Ok(ok) = env::current_exe() {
                ok.parent().unwrap().to_path_buf()
            } else {
                "".into()
            }
        };
        let texture = match params.get(0).unwrap(){
            Parameter::Texture(v) => v,
            _ => ""
        };
        let file = execpath.join("core/assets/textures").join(texture);
        let texture = asset_server.load(file);
        //get position
        if params.get(1).is_none(){
            return;
        }
        #[allow(unused)]
        let pos = match params.get(1).unwrap(){
            Parameter::Position(v) => v,
            _ => &Vector3D{x: 0.0, y: 0.0, z: 0.0}
        };
        //size
        if params.get(2).is_none(){
            if node.is_none(){
                command.spawn(SpriteBundle {
                    texture: texture,
                    ..Default::default()
                });
            } else {
                let node = node.unwrap();
                command.spawn(node).with_children(|parent|{
                    parent.spawn(SpriteBundle {
                        texture: texture,
                        ..Default::default()
                    });
                });
            }
            return;
        }
        let size = match params.get(2).unwrap(){
            Parameter::Size(v) => v,
            _ => &Vector2D{x: 0.0, y: 0.0}
        };
        if node.is_none(){
            command.spawn(SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(size.x as f32, size.y as f32)),
                    ..Default::default()
                },
                texture: texture,
                ..Default::default()
            });
        } else {
            let node = node.unwrap();
            command.spawn(node).with_children(|parent|{
                parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(size.x as f32, size.y as f32)),
                        ..Default::default()
                    },
                    texture: texture,
                    ..Default::default()
                });
            });
        }
    }

    pub fn spawn_text(&self, command: &mut Commands, _node: Option<NodeBundle>, asset_server: &Res<AssetServer>, params: Vec<Parameter>) {
        let res = self.parse_text(1, params.clone(), asset_server);
        if res.is_err(){
            let err = res.err().unwrap();
            error!("Error: {}", err);
            return;
        }
        if params.get(0).is_none(){
            return;
        }
        let pos = match params.get(0).unwrap(){
            Parameter::Position(p) => p,
            _ => &Vector3D{x: 0.0, y: 0.0, z: 0.0}
        };
        if params.get(1).is_none(){
            return;
        }
        let size = match params.get(1).unwrap(){
            Parameter::Size(s) => s,
            _ => &Vector2D{x: 0.0, y: 0.0}
        };
        let mut text = res.unwrap();
        text.style.size = Size::new(Percent(size.x as f32), Percent(size.y as f32));
        text.style.position_type = Relative;
        text.style.position = UiRect {
            left: Px(pos.x as f32),
            top: Px(pos.y as f32),
            ..Default::default()
        };
        command.spawn(text);
    }

    fn start(&mut self, command: &mut Commands, asset_server: &Res<AssetServer>){
        for function in self.actions.clone() {
            let params = function.parameters.clone();
            let name = function.name.clone();
            //check if function is in the map
            if let Some((_, func)) = SCENE_FUNCTIONS.iter().find(|(n,_)| n == &name){
                func(self, command, None, asset_server, params);
            }
        }
    }
    #[allow(unused)]
    fn render(&mut self, command: &mut Commands){
        // for function in self.actions.clone() {
        //     let params = function.parameters.clone();
        //     let name = function.name.clone();
        //     //check if function is in the map
        //     if let Some((_, func)) = SCENE_FUNCTIONS.iter().find(|(n,_)| n == &name){
        //         func(self, command, params);
        //     }
        // }
    }
}