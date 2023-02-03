use bevy::{ecs::system::Res,prelude::{Commands, TextBundle, Color, BuildChildren, JustifyContent, AlignItems, info, AssetServer, Vec2}, ui::{ZIndex::Global, Size, Val::Percent, PositionType::Absolute,UiRect, Style, node_bundles::ButtonBundle}, text::{TextStyle}, scene::SceneBundle, sprite::{SpriteBundle, Sprite}};
use serde::{Deserialize, Serialize};
use crate::{game_reader::{
    location::Location,
    functions::{Function, Value, Parameter},
}, ui::component::UIType};
use std::fmt::Display;
use crate::game_reader::functions::{Vector3D, Vector2D, Vector4D};
use std::{env, path::PathBuf};

//map of functions
const SCENE_FUNCTIONS: &[(&str, fn(&Description, &mut Commands, &Res<AssetServer>, Vec<Parameter>))] = &[
    ("spawn_ui", Description::spawn_ui),
];

//a plugin that loads the game data
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Scene {
    pub name: String,
    pub description: Description,
    #[serde(skip_serializing, skip_deserializing)]
    pub started: bool,
}

impl Scene {
    pub fn start(&mut self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
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
        button_bundle.z_index = Global(pos.z as i32);
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

    pub fn spawn_ui(&self, command: &mut Commands, asset_server: &Res<AssetServer>, params: Vec<Parameter>) {
        //spawns a UI component
        //first index is type
        if let Some(param1) = params.get(0){
            let value = match param1{
                Parameter::Ui(s) => s,
                _ => "",
            };
            let tp = UIType::from_str(&value);
            println!("Spawning UI: {}", value);
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
                    command.spawn(button_bundle).with_children(|parent|{
                        parent.spawn(text_bundle);
                    });
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

    fn start(&mut self, command: &mut Commands, asset_server: &Res<AssetServer>){
        for function in self.actions.clone() {
            let params = function.parameters.clone();
            let name = function.name.clone();
            //check if function is in the map
            if let Some((_, func)) = SCENE_FUNCTIONS.iter().find(|(n,_)| n == &name){
                func(self, command, asset_server, params);
            }
        }
    }
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