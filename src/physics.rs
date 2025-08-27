use macroquad::prelude::*;

/*use crate::{

};*/

pub const G: f64 = 6.6743e-11; // m^3 * kg^-1 * s^-2

// Could also be acceleration to not multiply by the mass and then divide it back
// Force that body 1 experiences. Negative of body 2 force
pub fn gravity_force(pos_1: DVec2, pos_2: DVec2, mass_1: f64, mass_2: f64) -> DVec2 {
    let relative_pos = pos_2 - pos_1;
    let distance_squared = relative_pos.length_squared();
    let direction = relative_pos.normalize();
    
    return G * mass_1 * mass_2 / distance_squared * direction
}