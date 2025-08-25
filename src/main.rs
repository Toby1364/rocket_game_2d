#![allow(static_mut_refs)]
use macroquad::prelude::*;

mod update;
mod rendering;
mod body;

struct GameState {
    bodies: Vec<body::Body>,

    ups: f64,
}

static mut GAME_STATE: GameState = GameState {
    bodies: Vec::new(),

    ups: 0.
};


fn window_conf() -> Conf {
    Conf {

        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    std::thread::spawn(update::main);
    rendering::main().await;
}
