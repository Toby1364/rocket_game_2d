use macroquad::prelude::*;
// Structure, I know you hate it.

use crate::{
    rendering,
};

pub struct Body {
    pub pos: DVec2,
    pub vel: DVec2,
    pub force: DVec2,

    pub mass: f64,
    pub radius: f64,
}

impl Body {
    pub fn new(pos: DVec2, vel: DVec2, mass: f64, radius: f64) -> Self {
        Self {
            pos,
            vel,
            force: DVec2::ZERO,

            mass,
            radius,
        }
    }

    pub fn draw_pos(&self, cam: &rendering::Cam) -> DVec2 { self.pos * cam.zoom - cam.off() }
}

pub struct Rocket {
    pub pos: DVec2,
    pub vel: DVec2,
    pub force: DVec2,
    pub mass: f64,
}