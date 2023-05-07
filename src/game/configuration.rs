use egui::{RichText, Button, Color32, Widget};
use ggegui::{Gui};
use ggez::{graphics::{self, Color, DrawParam},glam};

use crate::core::{toml_loader::Configuration,Event, logger::{debug, error}};
use super::scene::Scene;

const BG_COLOR: Color = Color::new(0.0, 0.0, 0.0, 1.0);

const TEXT_SIZE: f32 = 18.0;

#[allow(dead_code)]
const TILE_SIZE: f32 = 32.0;
const CHUNK_SIZE: f32 = 10.0;

pub struct Game {
    pub data: Box<Scene>,
    pub configuration: Box<Configuration>,
    pub gui: Gui,
    pub running: bool,
}

impl Game {
    pub fn new(ctx: &mut ggez::Context, mut config: Box<Configuration>) -> Game {
        let mut scene = Box::new(Scene::new(*(config.clone())));
        config.load_chunks(ctx, scene.clone().map.unwrap().dungeon_list.clone());
        let camera: (f32, f32) = (config.settings.get_map_size().0/2.0, config.settings.get_map_size().1/2.0);
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
            //get chunk buffer
            let chunk_buffer = self.configuration.get_chunks();
            let dungeon = val.dungeon.clone();
            let camera = self.data.camera;
            let chunk_size = CHUNK_SIZE * TILE_SIZE;
            //camera is in pixels, we need to convert it to chunks
            let start_x = (camera.0 / chunk_size) as u32;
            let start_y = (camera.1 / chunk_size) as u32;
            let end_x = start_x + self.configuration.settings.fit.w;
            let end_y = start_y + self.configuration.settings.fit.h;
            //render scale refers to pixels per chunk
            let render_size = self.configuration.settings.render_scale;
            let render_scale = render_size / chunk_size;
            debug!("Start: ({}, {}), End: ({}, {}), Render Size vs Chunk Size: ({},{})", start_x, start_y, end_x, end_y, render_size, chunk_size);
            //get all chunks in range
            for x in start_x..end_x {
                for y in start_y..end_y {
                    //get chunk
                    let chunk = if let Some(chunk) = dungeon.get_chunk((x, y)) {
                        chunk.id
                    } else {
                        error!("Chunk ({}, {}) not found", x, y);
                        continue;
                    };
                    let chunk = chunk_buffer.get(chunk as usize).unwrap().clone();
                    //draw chunk
                    canvas.draw(&chunk, DrawParam::default().scale(glam::Vec2::new(render_scale,render_scale)).dest(glam::Vec2::new(x as f32 * render_size, y as f32 * render_size)));
                }
            }       
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