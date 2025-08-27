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
    pub pos: DVec2, // m
    pub vel: DVec2, // m/s
    pub force: DVec2, // newtons
    pub dry_mass: f64, // kg
    pub fuel_mass: f64, // kg
    pub mass_flow_rate: f64, // kg/s
    pub effective_exhaust_velocity: f64, // m/s
    pub rotation: f64,
    pub angular_velocity: f64,
}

impl Default for Rocket {
    fn default() -> Self {
        Self {
            pos: DVec2::ZERO,
            vel: DVec2::ZERO,
            force: DVec2::ZERO,
            dry_mass: 0.,
            fuel_mass: 0.,
            mass_flow_rate: 0.0,
            effective_exhaust_velocity: 0.0,
            rotation: 0.,
            angular_velocity: 0.0,
        }
    }
}

impl Rocket {
    pub fn mass(&self) -> f64 {
        self.dry_mass + self.fuel_mass
    }
}
