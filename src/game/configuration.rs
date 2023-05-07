use egui::{RichText, Button, Color32, Widget};
use ggegui::{Gui};
use ggez::{graphics::{self, Color, DrawParam},glam};

use crate::core::{toml_loader::Configuration,Event, logger::{debug, error}};
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
    pub fn new(ctx: &mut ggez::Context, mut config: Box<Configuration>) -> Game {
        let mut scene = Box::new(Scene::new(*(config.clone())));
        config.load_chunks(scene.clone().map.unwrap().dungeon_list.clone());
        let camera: (f32, f32) = (config.settings.size.w as f32/2.0, config.settings.size.h as f32/2.0);
        scene.set_camera(camera);
        Game {
            data: scene,
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

        //Temporary movement, set to appropriate movement amount later
        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::W) {
            self.data.move_vert(1.0);
        }else if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::S) {
            self.data.move_vert(-1.0);
        }
        if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::A) {
            self.data.move_horiz(-1.0);
        }else if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::D) {
            self.data.move_horiz(1.0);
        }

        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BG_COLOR);
        //get all tiles in data.map.map
        //scale each tile by self.configuration.settings.fit
        if let Some(val) = &self.data.map {
            let raw_png = self.configuration.build_map_as_image(self.data.camera, val.dungeon.clone());
            if let Ok(ok) = graphics::Image::from_bytes(ctx, &raw_png) {
                canvas.draw(&ok, DrawParam::default().dest(glam::Vec2::ZERO));
            } else {
                error!("Failed to load image");
            };
        } else {
            error!("Failed to load map");
        }
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }

    fn status(&self) -> bool {
        self.running
    }
}