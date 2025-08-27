#![allow(static_mut_refs)]
use macroquad::prelude::*;

mod update;
mod rendering;
mod body;
mod physics;

struct GameState {
    bodies: Vec<body::Body>,

    rockets: Vec<body::Rocket>,
    engine_working: bool,

    sim_speed: f64,

    ups: f64,
}

static mut GAME_STATE: GameState = GameState {
    bodies: Vec::new(),

    rockets: Vec::new(),
    engine_working: true,

    sim_speed: 20.,

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
