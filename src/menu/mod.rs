use egui::{RichText, Color32, Button, Widget};
use ggegui::{Gui};
use ggez::{event::EventHandler, graphics::{self, Color, DrawParam},glam};
use std::{path::Path};
use crate::{core::{
    toml_loader::{Configuration, TomlAsset}
}, integrity::Integrity, game::configuration::Game};

const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);
//ecolor
const TEXT_SIZE: f32 = 24.0;

const COREDIR: &str = "core";
const MODFILE: &str = "core/mods.toml";

pub struct Menu {
    pub gui: Gui,
    pub configuration: Box<Configuration>,
    pub event: Option<Box<dyn crate::core::Event>>,
}

impl Menu {
    pub fn new(ctx: &mut ggez::Context) -> Menu {
        #[cfg(debug_assertions)]
        let mode = "debug";
        #[cfg(not(debug_assertions))]
        let mode = "release";

        let path = if mode == "debug" {
            Path::new(MODFILE).to_owned()
        } else {
            //get executable path
            //get parent path
            let mut pathbuf = std::env::current_exe().unwrap();
            pathbuf.pop();
            pathbuf.push(MODFILE);
            let path = pathbuf.as_path().to_owned();
            path
        };
        let file_string = {
            if let Ok(ok) = std::fs::read_to_string(path) {
                ok
            } else {
                String::new()
            }
        };
        let mut configuration = {
            let toml = toml::from_str::<TomlAsset>(&file_string);
            if let Ok(ok) = toml {
                match ok {
                    TomlAsset::Configuration(configuration) => Box::new(configuration),
                    _ => panic!("Could not load configuration file!"),
                }
            } else {
                println!("{}", toml.err().unwrap());
                panic!("Could not load configuration file!");
            }
        };
        let core = if mode == "debug" {
            Path::new(COREDIR).to_owned()
        } else {
            //get executable path
            //get parent path
            let mut pathbuf = std::env::current_exe().unwrap();
            pathbuf.pop();
            pathbuf.push(COREDIR);
            let path = pathbuf.as_path().to_owned();
            path
        };
        configuration.get_sum(core.clone());
        let res = Integrity::new(configuration.sum.clone(), core).check();
        if let Err(err) = res {
            panic!("Integrity check failed: {}", err);
        }

        configuration.map_textures();

        Menu {
            gui: Gui::new(ctx),
            configuration,
            event: None,
        }
    }
}

impl EventHandler for Menu {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let gui_ctx = self.gui.ctx();
        let (width, height) = ctx.gfx.drawable_size();
        if width != self.configuration.settings.resolution.w as f32 || height != self.configuration.settings.resolution.h as f32 {
            if let Err(err) = ctx.gfx.set_drawable_size(self.configuration.settings.resolution.w as f32, self.configuration.settings.resolution.h as f32) {
                println!("{}", err);
            }
        }
        if self.event.is_none() {
            egui::Window::new(RichText::new("Solid").size(TEXT_SIZE))
            .fixed_size(egui::vec2(width, height))
            .fixed_pos(egui::pos2(0.0,0.0))
            .resizable(false)
            .show(&gui_ctx, |ui| {
                ui.heading(RichText::new("Solid").size(TEXT_SIZE).color(Color32::LIGHT_BLUE));
                ui.label("A game by Ryan Fong");
                ui.separator();
                let buttons = [
                    ("Play",Button::new(RichText::new("Play").size(TEXT_SIZE).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Load",Button::new(RichText::new("Load").size(TEXT_SIZE).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Settings",Button::new(RichText::new("Settings").size(TEXT_SIZE).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Quit",Button::new(RichText::new("Quit").size(TEXT_SIZE).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                ];
                for button in buttons.iter() {
                    if button.1.clicked() {
                        match button.0 {
                            "Play" => {
                                println!("Play");
                                let game = Game::new(ctx, self.configuration.clone());
                                self.event = Some(Box::new(game));
                            },
                            "Load" => {
                                println!("Load");
                            },
                            "Settings" => {
                                println!("Settings");
                            },
                            "Quit" => {
                                println!("Quit");
                                std::process::exit(0);
                            },
                            _ => {},
                        }
                    }
                }
            });
        } else {
            //triggers when event is done
            if !self.event.as_mut().unwrap().status() {
                self.event = None;
            }
            return self.event.as_mut().unwrap().update(ctx);
        }
        self.gui.update(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        if self.event.is_some() {
            return self.event.as_mut().unwrap().draw(ctx);
        } else {
            let mut canvas = graphics::Canvas::from_frame(ctx, BG_COLOR);
            canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
            canvas.finish(ctx)
        }
    }
}