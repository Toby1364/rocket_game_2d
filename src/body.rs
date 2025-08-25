use macroquad::prelude::*;
// Structure, I know you hate it.

use crate::{
    rendering,
};

pub struct Body {
    pub pos: DVec2,
    pub vel: DVec2,

    pub next_pos: DVec2,
    pub next_vel: DVec2,
    pub force: DVec2,

    pub mass: f64,
    radius: f64,
}

impl Body {
    // Do you want to update all planets from outside, or because they hopefully won't collide have an update function that will just add the pos and vel?

    // Well you need to calculate forces of gravity for each planet. Which depend on the state of all bodies. Assuming n-body gravity
    // Outside seems reasonable

    // You are right, so from outside. So this is for me.

    // Do you want some file for that or will we just put it in update?
    // im okay with just an update
    // do we want it in the body itself? not within an update function? You'll have to collect and Vec of all those stuff anyway, and this way we aren't reallocating that memory every time. 
    // You can also just after doing calculations do for b in bodies { b.update() }
    
    pub fn new(pos: DVec2, vel: DVec2, mass: f64, radius: f64) -> Self {
        Self {
            pos,
            vel,
            next_pos: pos,
            next_vel: vel,
            force: DVec2::ZERO, // honestly, makes no sense, shouldnt really be available here
            mass,
            radius,
        }
    }

    pub fn draw(&self, cam: &rendering::Cam) {
        let center = (self.pos * cam.zoom) - cam.off();

        draw_circle(
            center.x as f32,
            center.y as f32,
            (self.radius * cam.zoom) as f32, 
            WHITE
        ); 
        draw_circle(
            center.x as f32,
            center.y as f32,
            (self.radius * cam.zoom) as f32 - 1., 
            BLACK
        ); 
    }

    pub fn update(&mut self) {
        self.pos = self.next_pos;
        self.vel = self.next_vel;
    }
}
