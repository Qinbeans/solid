use ggegui::{Gui};
use ggez::{event::EventHandler, graphics::{self, Color, DrawParam},glam};

use crate::{game_reader::{
    scene::Scene,
    toml_loader::{Configuration},
}, integrity::Integrity};
use std::path::Path;
use super::toml_loader::TomlAsset;

const COREDIR: &str = "core";
const DATADIR: &str = "core/data";
const MODFILE: &str = "core/mods.toml";

pub struct Game {
    pub current_scene: Box<Scene>,
    pub configuration: Box<Configuration>,
    pub gui: Gui,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context) -> Game {
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
        let mut file_string = {
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
        let entry = configuration.entry.clone();
        let path = if mode == "debug" {
            let pathbuf = Path::new(DATADIR).join(entry);
            pathbuf.as_path().to_owned()
        } else {
            //get executable path
            //get parent path
            let mut path = std::env::current_exe().unwrap();
            path.pop();
            path.push(DATADIR);
            path.push(entry);
            path.as_path().to_owned()
        };
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(path) {
                ok
            } else {
                String::new()
            }
        };
        let current_scene = {
            if let Ok(ok) = toml::from_str::<TomlAsset>(&file_string) {
                match ok {
                    TomlAsset::Scene(scene) => Box::new(scene),
                    _ => panic!("Could not load scene file!"),
                }
            } else {
                panic!("Could not load scene file!");
            }
        };
        Game {
            current_scene,
            configuration,
            gui: Gui::new(ctx),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        //go through scene and update all entities
        let gui_ctx = self.gui.ctx(); 
        self.current_scene.description.update(&gui_ctx);
        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let bg_color = self.current_scene.description.background.clone();
        let bg_color = Color::new(bg_color.x as f32, bg_color.y as f32, bg_color.z as f32, bg_color.w as f32);
        let mut canvas = graphics::Canvas::from_frame(ctx, bg_color);
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }
}