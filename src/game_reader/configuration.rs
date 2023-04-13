use ggegui::{Gui};
use ggez::{event::EventHandler, graphics::{self, Color, DrawParam},glam};

use crate::{game_reader::{
    toml_loader::{Configuration},
}, integrity::Integrity};
use std::path::Path;
use super::{toml_loader::TomlAsset,scene::Scene};

const COREDIR: &str = "core";
const MODFILE: &str = "core/mods.toml";
const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

pub struct Game {
    pub data: Box<Scene>,
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
        Game {
            data: Box::new(Scene::new(*(configuration.clone()))),
            configuration,
            gui: Gui::new(ctx),
        }
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        //go through scene and update all entities
        let _ = self.gui.ctx();
        self.gui.update(ctx);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, BG_COLOR);
        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
        canvas.finish(ctx)
    }
}