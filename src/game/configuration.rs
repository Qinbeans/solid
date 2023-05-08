use egui::{RichText, Button, Color32, Widget};
use ggegui::{Gui};
use ggez::{graphics::{self, DrawParam},glam};

use crate::core::{toml_loader::Configuration,Event, logger::{debug, error}};
use super::scene::Scene;

const TEXT_SIZE: f32 = 18.0;

#[allow(dead_code)]
const TILE_SIZE: f32 = 32.0;
// const RENDER_CHUNK_SIZE: f32 = 320.0;

pub struct Game {
    pub data: Box<Scene>,
    pub configuration: Box<Configuration>,
    pub gui: Gui,
    pub running: bool,
    pub heartbeat: (u64,u64),
}

impl Game {
    pub fn new(ctx: &mut ggez::Context, mut config: Box<Configuration>) -> Game {
        let mut scene = Box::new(Scene::new(*(config.clone())));
        config.load_chunks(ctx, scene.clone().map.unwrap().dungeon_list.clone());
        scene.set_char_text(ctx, config.clone().texture_map.texture_buf);
        let camera: (f32, f32) = (config.settings.get_map_size().0/2.0, config.settings.get_map_size().1/2.0);
        scene.set_camera(camera);
        Game {
            data: scene,
            configuration: config,
            gui: Gui::new(ctx),
            running: true,
            heartbeat: (0,0),
        }
    }
}

impl Event for Game {
    fn update(&mut self, ctx: &mut ggez::Context) {
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
                    ("Menu",Button::new(RichText::new("Main Menu").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                    ("Desktop",Button::new(RichText::new("Desktop").size(TEXT_SIZE * self.configuration.settings.scale).color(Color32::DARK_GRAY)).fill(Color32::LIGHT_BLUE).ui(ui)),
                ];
                for button in buttons.iter() {
                    if button.1.clicked() {
                        match button.0 {
                            "Settings" => {
                                debug!("Settings");
                            },
                            "Menu" => {
                                debug!("Exit");
                                self.running = false;
                            },
                            "Desktop" => {
                                debug!("Quit");
                                std::process::exit(0);
                            },
                            _ => {},
                        }
                    }
                }
            }
        );

        if self.heartbeat.0 == 0 {
            self.heartbeat.0 = ctx.time.fps() as u64;
        }

        //ensures movement is consistent across all computers
        //check for key press every 1/100th of a second
        // if every cycle is 1/144 aka 144 fps, then 1/100th of a second is 1.44 cycles
        // we want to check for key press every 1/100th of a second
        let cycle = 100.0 / self.heartbeat.0 as f64 * self.heartbeat.1 as f64;
        
        if cycle == 100.0 {
            if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::W) {
                self.data.move_vert(-0.1);//refers to tile based movement
            }else if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::S) {
                self.data.move_vert(0.1);
            }
            if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::A) {
                self.data.move_horiz(-0.1);
            }else if ctx.keyboard.is_key_pressed(ggez::input::keyboard::KeyCode::D) {
                self.data.move_horiz(0.1);
            }
            self.heartbeat.1 = 0;
        }
        self.heartbeat.1 += 1;

        let label_text = format!("FPS: {} Pos: {:?}", ctx.time.fps() as u64, self.data.camera);
        //draw fps in top right corner
        egui::Area::new("fps")
            .fixed_pos(egui::pos2(0.0, height - (TEXT_SIZE / 2.0 * self.configuration.settings.scale)))
            .show(&gui_ctx, |ui| {
                ui.label(RichText::new(label_text).size(TEXT_SIZE / 2.0 * self.configuration.settings.scale));
            }
        );
        //Temporary movement, set to appropriate movement amount later

        self.gui.update(ctx);
    }

    //Do not clone when drawing, it will cause lag
    fn draw(&mut self, canvas: &mut graphics::Canvas) {
        //get all tiles in data.map.map
        //scale each tile by self.configuration.settings.fit
        let val = self.data.map.as_ref().unwrap();
        // //get chunk buffer
        let chunk_buffer = self.configuration.get_chunks();
        let dungeon = &val.dungeon;
        let camera = self.data.camera;
        //camera is in chunks
        //center chunk loading on camera
        let start_x = camera.0 as u32 - (self.configuration.settings.fit.w / 2) as u32;
        let start_y = camera.1 as u32 - (self.configuration.settings.fit.h / 2) as u32;
        let end_x = start_x + self.configuration.settings.fit.w as u32 + 1; //add one as a buffer
        let end_y = start_y + self.configuration.settings.fit.h as u32 + 1;
        //get all chunks in range
        for x in start_x..end_x {
            for y in start_y..end_y {
                let chunk = if let Some(chunk) = dungeon.get_chunk((x, y)) {
                    chunk.id
                } else {
                    error!("Chunk ({}, {}) not found", x, y);
                    continue;
                };
                canvas.draw(
                    chunk_buffer.get(chunk as usize).unwrap(),
                    DrawParam::default().scale(
                        glam::Vec2::new(
                            self.configuration.settings.render_scale,
                            self.configuration.settings.render_scale
                        )
                    ).dest(
                        glam::Vec2::new(
                            (x - start_x) as f32 * self.configuration.settings.render_size - self.configuration.settings.render_size / 2.0,
                            (y - start_y) as f32 * self.configuration.settings.render_size - self.configuration.settings.render_size / 2.0
                        )
                    )
                );
            }
        }

        //draw player
        let pos = (
            self.configuration.settings.resolution.w / 2,
            self.configuration.settings.resolution.h / 2
        ); 
        canvas.draw(
            self.data.get_character_cln().get_texture(self.data.direction.clone()),
            DrawParam::default().scale(
                glam::Vec2::new(
                    self.configuration.settings.render_scale,
                    self.configuration.settings.render_scale
                )
            ).dest(
                glam::Vec2::new(
                    pos.0 as f32,
                    pos.1 as f32
                )
            )
        );

        canvas.draw(&self.gui, DrawParam::default().dest(glam::Vec2::ZERO));
    }

    fn status(&self) -> bool {
        self.running
    }
}