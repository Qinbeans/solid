use egui::{RichText, Button, Color32, Widget};
use ggegui::{Gui};
use ggez::{graphics::{self, Color, DrawParam},glam};

use crate::core::{toml_loader::Configuration,Event};
use super::scene::Scene;

const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

const TEXT_SIZE: f32 = 18.0;

pub struct Game {
    pub data: Box<Scene>,
    pub configuration: Box<Configuration>,
    pub gui: Gui,
    pub running: bool,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context, config: Box<Configuration>) -> Game {
        Game {
            data: Box::new(Scene::new(*(config.clone()))),
            configuration: config,
            gui: Gui::new(ctx),
            running: true,
        }
    }
}

impl Event for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        //go through scene and update all entities
        let gui_ctx = self.gui.ctx();
        //default as collapsed
        let (width, height) = ctx.gfx.drawable_size();
        egui::Window::new(RichText::new("Menu").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY))
            .fixed_size(egui::vec2(width * self.configuration.settings.scale, height * self.configuration.settings.scale))
            .fixed_pos(egui::pos2(0.0,0.0))
            .resizable(false)
            .show(&gui_ctx, |ui| {
                //
                let buttons = [
                    ("Settings",Button::new(RichText::new("Settings").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Quit",Button::new(RichText::new("Quit").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Exit",Button::new(RichText::new("Exit").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                ];
                for button in buttons.iter() {
                    if button.1.clicked() {
                        match button.0 {
                            "Settings" => {
                                println!("Settings");
                            },
                            "Quit" => {
                                println!("Quit");
                                std::process::exit(0);
                            },
                            "Exit" => {
                                println!("Exit");
                                self.running = false;
                            },
                            _ => {},
                        }
                    }
                }
            });
        
        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BG_COLOR);
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }

    fn status(&self) -> bool {
        self.running
    }
}