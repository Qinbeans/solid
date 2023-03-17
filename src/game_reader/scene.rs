use crate::game_reader::functions::Function;
use egui::{Frame, Margin};
use serde::{Deserialize, Serialize};
use crate::egui::{egui::{self, Pos2, Vec2, Ui, WidgetText, RichText, Color32}, GuiContext};
use super::{functions::{Vector4D, Vector3D, Vector2D, Value, Parameter}, location::Location};

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
    pub fn update(&mut self, ctx: &GuiContext) {
        for action in self.actions.iter() {
            //execute functions from scene_functions
            match action.name.as_str() {
                "spawn_window" => {
                    if let Err(e) = self.spawn_window(ctx, None, action.parameters.clone()) {
                        println!("{}", e);
                    }
                }
                _ => {
                    println!("Unknown function");
                }
            }
        }
    }
    fn spawn_window(&self, ctx: &GuiContext, _: Option<&mut Ui>, parameters: Vec<Parameter>) -> Result<(), String> {
        let mut text = String::new();
        let mut position = Vector3D::default();
        let mut size = Vector2D::default();
        let mut color = Vector4D::default();
        let mut font_size = 0.0;
        let mut functions:Vec<Function> = Vec::new();
        for parameter in parameters {
            match parameter {
                Parameter::String(s) => text = s,
                Parameter::Position(v) => position = v,
                Parameter::Vector2D(v) => size = v,
                Parameter::Color(v) => color = v,
                Parameter::Font(f) => font_size = f,
                Parameter::Actions(a) => functions = a,
                _ => {}
            }
        }
        let pos = Pos2::new(position.x as f32, position.y as f32);
        let depth = position.z;
        let color32 = Color32::from_rgba_premultiplied((color.x * 255.0) as u8,(color.y * 255.0) as u8,(color.z * 255.0) as u8,(color.w * 255.0) as u8);
        let frame = Frame::default().fill(color32).inner_margin(Margin::same(5.0));
        egui::Window::new(text)
            .collapsible(false)
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .min_width(size.x as f32)
            .min_height(size.y as f32)
            .current_pos(pos)
            .frame(frame)
            .show(ctx, move |ui| {
                for function in functions.iter() {
                    match function.name.as_str() {
                        "spawn_ui" => {
                            if let Err(e) = self.spawn_ui(ctx, Some(ui), function.parameters.clone()) {
                                println!("{}", e);
                            }
                        }
                        _ => {
                            println!("Unknown function");
                        }
                    }
                }
            });
        Ok(())
    }
    fn spawn_ui(&self, ctx: &GuiContext, ui: Option<&mut Ui>, parameters: Vec<Parameter>) -> Result<(), String> {
        if let Some(ui) = ui {
            let mut ui_name = String::new();
            let mut text = String::new();
            let mut colors: Vec<Vector4D> = Vec::new();
            let mut font_size = 0.0;
            let mut functions:Vec<Function> = Vec::new();
            for parameter in parameters {
                match parameter {
                    Parameter::Ui(s) => ui_name = s,
                    Parameter::Text(s) => text = s,
                    Parameter::Color(v) => colors.push(v),
                    Parameter::Font(f) => font_size = f,
                    Parameter::Actions(a) => functions = a,
                    _ => {}
                }
            }
            match ui_name.as_str() {
                "button" => {
                    let color = colors.pop().unwrap_or(Vector4D::default());
                    let bg_color = colors.pop().unwrap_or(Vector4D::default());
                    let color32 = Color32::from_rgba_premultiplied((color.x * 255.0) as u8,(color.y * 255.0) as u8,(color.z * 255.0) as u8,(color.w * 255.0) as u8);
                    let bg_color32 = Color32::from_rgba_premultiplied((bg_color.x * 255.0) as u8,(bg_color.y * 255.0) as u8,(bg_color.z * 255.0) as u8,(bg_color.w * 255.0) as u8);
                    let text = WidgetText::RichText(RichText::new(text).size(font_size as f32)).color(color32);
                    //assumes a window has been created
                    let button = egui::Button::new(text).fill(bg_color32);
                    if ui.add(button).clicked() {
                        for function in functions.iter() {
                            match function.name.as_str() {
                                "exit" => {
                                    std::process::exit(0);
                                }
                                _ => {
                                    println!("Unknown function");
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            return Ok(());
        }
        Err(String::from("No UI found"))
    }

}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub name: String,
    pub description: Description,
    #[serde(skip)]
    pub started: bool,
}

impl Clone for Scene {
    fn clone(&self) -> Self {
        Scene {
            name: self.name.clone(),
            description: self.description.clone(),
            started: false,
        }
    }
}