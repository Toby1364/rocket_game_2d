use std::default;

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
    pub dry_mass: f64,
    pub fuel_mass: f64,
    pub thrust: f64,
}

impl Default for Rocket {
    fn default() -> Self {
        Self {
            pos: DVec2::ZERO,
            vel: DVec2::ZERO,
            force: DVec2::ZERO,
            dry_mass: 0.,
            fuel_mass: 0.,
            thrust: 0.,
        }
    }
}

impl Rocket {
    pub fn mass(&self) -> f64 {
        self.dry_mass + self.fuel_mass
    }
}
