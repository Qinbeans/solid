use egui::{RichText, Button, Color32, Widget};
use ggegui::{Gui};
use ggez::{graphics::{self, Color, DrawParam},glam};

use crate::core::{toml_loader::Configuration,Event, logger::debug};
use super::scene::Scene;

const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

const TEXT_SIZE: f32 = 18.0;
const TILE_SIZE: f32 = 32.0;

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
                                debug!("Settings");
                            },
                            "Quit" => {
                                debug!("Quit");
                                std::process::exit(0);
                            },
                            "Exit" => {
                                debug!("Exit");
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
        //get all tiles in data.map.map
        //scale each tile by self.configuration.settings.fit
        let scale = if self.configuration.settings.fit.w as f32/self.configuration.settings.resolution.w as f32 > self.configuration.settings.fit.h as f32 /self.configuration.settings.resolution.h as f32 {
            self.configuration.settings.fit.h as f32 /self.configuration.settings.resolution.h as f32
        } else {
            self.configuration.settings.fit.w as f32/self.configuration.settings.resolution.w as f32
        };
        for tile in self.data.map.as_mut().unwrap().map.iter() {
            //use tile.0 as a coordinate of where to draw the tile
            //use tile.1 as the tile to draw
            //use scale to scale the tile
        }
        canvas.finish(ctx)
    }

    fn status(&self) -> bool {
        self.running
    }
}