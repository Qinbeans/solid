use ggegui::{Gui};
use ggez::{graphics::{self, Color, DrawParam},glam};

use crate::core::{toml_loader::Configuration,Event};
use super::scene::Scene;

const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

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
        let _ = self.gui.ctx();
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