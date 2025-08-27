use macroquad::prelude::*;
use std::time::SystemTime;

use crate::{
    // Here we will import other files.
    body::*,

    GAME_STATE,
};


const G: f64 = 6.6743e-11; // m^3 * kg^-1 * s^-2
const SIM_SPEED: f64 = 20.0 * 1000.0; // temporary, to see if physics works

// Could also be acceleration to not multiply by the mass and then divide it back
fn gravity_force(body_1: &Body, body_2: &Body) -> DVec2 {
    let relative_pos = body_2.pos - body_1.pos;
    let distance_squared = relative_pos.length_squared();
    let direction = relative_pos.normalize();
    
    return G * body_1.mass * body_2.mass / distance_squared * direction
}

fn now() -> u128 { SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() }

// is our whole update function unsafe
pub fn main() { unsafe {
    let mut frame_time = 1.;
    let mut frame_start;

    GAME_STATE.bodies.push(Body::new(dvec2(0., 0.), dvec2(0., 0.), 5.972e24, 6_371_000.));
    GAME_STATE.bodies.push(Body::new(dvec2(0., 100_000_000.), dvec2(2_000., 0.), 5.972e24, 6_371_000.));

    GAME_STATE.bodies.push(Body::new(dvec2(0., 384_400_000.), dvec2(1_000., 0.), 7.348e22, 1_737_500.0));
    GAME_STATE.bodies.push(Body::new(dvec2(0., 400_400_000.), dvec2(500., 0.), 7.348e11, 627_500.0));

    GAME_STATE.bodies.push(Body::new(dvec2(0., -370_000_000.), dvec2(1_000., 0.), 7.348e22, 1_737_500.0));

    loop {
        frame_start = now(); 
        
        // force assumed to be a zero vector here
        for i in 0..GAME_STATE.bodies.len() {
            for j in (i+1)..GAME_STATE.bodies.len() {
                //println!("{} : {}", i, j);
                let body_1 = &mut GAME_STATE.bodies[i];
                let body_2 = &mut GAME_STATE.bodies[j];
                let force = gravity_force(body_1, body_2);

                body_1.force += force;
                body_2.force -= force; // same force opposite direction

            }
        }

        for body in &mut GAME_STATE.bodies {
            body.vel += body.force / body.mass * frame_time * SIM_SPEED;
            body.pos += body.vel * frame_time * SIM_SPEED;

            // force should be zero at the beginning of the update. Its initialized to zero at start so clearing it after the update should be fine
            // although again, force only makes sense in the context of this update function.
            // maybe there is a way to not reallocate without making it larger scope
            // also is allocating a vec of length 10 really that bad? No it's not. -T
            body.force = DVec2::ZERO; 
        }

        GAME_STATE.ups = 1. / frame_time;
        while now() - frame_start < 5_000 {} // Busy waiting because if we give control to Kernel it might eat a lot more time.
        frame_time = (now() - frame_start) as f64 / 1_000_000_000.;
    }
}}
