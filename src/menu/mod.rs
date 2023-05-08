use egui::{RichText, Color32, Button, Widget};
use ggegui::{Gui};
use ggez::{event::EventHandler, graphics::{self, Color, DrawParam},glam};
use std::{path::{Path}, env::current_dir, fs::File, io::BufReader, io::Read};
use crate::{core::{
    toml_loader::{Configuration, TomlAsset}, logger::{error,debug}
}, integrity::Integrity, game::configuration::Game};


const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);
//ecolor
const TEXT_SIZE: f32 = 24.0;

const COREDIR: &str = "core";
const MODFILE: &str = "core/mods.toml";

#[derive(PartialEq)]
enum Mode {
    Debug,
    #[allow(dead_code)]
    Release,
}

pub struct Menu {
    pub gui: Gui,
    pub configuration: Box<Configuration>,
    pub event: Option<Box<dyn crate::core::Event>>,
    pub background: graphics::Image
}

impl Menu {
    pub fn new(ctx: &mut ggez::Context) -> Menu {
        #[cfg(debug_assertions)]
        let mode = Mode::Debug;
        #[cfg(not(debug_assertions))]
        let mode = Mode::Release;

        let path = if mode == Mode::Debug {
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
                error!("{}", toml.err().unwrap());
                panic!("Could not load configuration file!");
            }
        };
        let core = if mode == Mode::Debug {
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
        configuration.retrieve_sum(core.clone());
        let res = Integrity::new(configuration.get_sum(), core.clone()).check();
        if let Err(err) = res {
            panic!("Integrity check failed: {}", err);
        }
        configuration.settings.set_render_scale();
        configuration.map_textures();
        #[cfg(debug_assertions)]
        let image_path = current_dir().unwrap().join("core").join("assets").join("images").join("background.png");
        #[cfg(not(debug_assertions))]
        let image_path = std::env::current_exe().unwrap().parent().unwrap().join("core").join("assets").join("images").join("background.png");

        Menu {
            gui: Gui::new(ctx),
            configuration,
            event: None,
            background: {
                let file_res = File::open(image_path);
                if let Ok(file) = file_res {
                    let mut buf_reader = BufReader::new(file);
                    let mut buf = Vec::new();
                    buf_reader.read_to_end(&mut buf).unwrap();
                    let image = graphics::Image::from_bytes(ctx, &buf);
                    if let Ok(image) = image {
                        image
                    } else {
                        panic!("Could not load background image!");
                    }
                } else {
                    panic!("Could not load background image!");
                }
            },
        }
    }
}

impl EventHandler for Menu {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let gui_ctx = self.gui.ctx();
        let (width, height) = ctx.gfx.drawable_size();
        if width != self.configuration.settings.resolution.w as f32 || height != self.configuration.settings.resolution.h as f32 {
            if let Err(err) = ctx.gfx.set_drawable_size(self.configuration.settings.resolution.w as f32, self.configuration.settings.resolution.h as f32) {
                error!("{}", err);
            }
        }
        if self.event.is_none() {
            egui::Window::new(RichText::new("Solid").size(TEXT_SIZE * self.configuration.settings.scale))
            .fixed_size(egui::vec2(width * self.configuration.settings.scale, height * self.configuration.settings.scale))
            .fixed_pos(egui::pos2(0.0,0.0))
            .resizable(false)
            .show(&gui_ctx, |ui| {
                ui.heading(RichText::new("Solid").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::LIGHT_BLUE));
                ui.label("A game by Ryan Fong");
                ui.separator();
                let buttons = [
                    ("Play",Button::new(RichText::new("Play").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Load",Button::new(RichText::new("Load").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Settings",Button::new(RichText::new("Settings").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Quit",Button::new(RichText::new("Quit").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                ];
                for button in buttons.iter() {
                    if button.1.clicked() {
                        match button.0 {
                            "Play" => {
                                debug!("Play");
                                //within the context of *this* scene, the configs can be changed
                                let game = Game::new(ctx, self.configuration.clone());
                                self.event = Some(Box::new(game));
                            },
                            "Load" => {
                                debug!("Load");
                            },
                            "Settings" => {
                                debug!("Settings");
                            },
                            "Quit" => {
                                debug!("Quit");
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
            if self.event.is_some() {
                self.event.as_mut().unwrap().update(ctx);
            }
        }
        self.gui.update(ctx);
        Ok(())
    }
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BG_COLOR);
        if let Some(val) = self.event.as_mut() {
            val.draw(&mut canvas);
        } else {
            //resize image to fit screen
            let bg_param = DrawParam::default().dest(glam::Vec2::ZERO).scale(glam::Vec2::new(
                self.configuration.settings.resolution.w as f32 / self.background.width() as f32,
                self.configuration.settings.resolution.h as f32 / self.background.height() as f32,
            ));
            canvas.draw(&self.background, bg_param);
            canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        }
        canvas.finish(ctx)
    }
}