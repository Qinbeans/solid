#![allow(clippy::unnecessary_wraps)]
use ggez::event::{self};
use ggez::{ContextBuilder};

mod egui;

mod game_reader;
use game_reader::configuration::Game;

const AUTHOR: &str = "Ryan Fong";
const TITLE: &str = "Solid";
const SIZE: (f32, f32) = (800.0, 600.0);

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new(TITLE, AUTHOR)
        .window_setup(ggez::conf::WindowSetup::default().title(TITLE))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SIZE.0, SIZE.1).resizable(true))
        .build()
        .expect("Could not create ggez context!");
    let my_game = Game::new(&mut ctx);
    event::run(ctx, event_loop, my_game);
}