use macroquad::prelude::*;
use std::time::SystemTime;

use crate::{
    body::*,
    physics::*,

    GAME_STATE,
};

fn now() -> u128 { SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() }

// is our whole update function unsafe
pub fn main() { unsafe {
    let mut frame_time = 1.;
    let mut frame_start;

    /* Sun */ GAME_STATE.bodies.push(Body::new(dvec2(-150_000_000_000., 0.), dvec2(0., 0.), 1.989e30, 696_256_000.));
    
    /* Earth */ GAME_STATE.bodies.push(Body::new(dvec2(0., 0.), dvec2(0., -29_800.), 5.972e24, 6_371_000.));

    /* Moon */ GAME_STATE.bodies.push(Body::new(dvec2(0., 384_400_000.), dvec2(1_000., -29_800.), 7.348e22, 1_737_500.0));
    /* Bonus Moon */ GAME_STATE.bodies.push(Body::new(dvec2(0., 400_400_000.), dvec2(500., -29_800.), 7.348e11, 627_500.0));

    /* Moon moon */ GAME_STATE.bodies.push(Body::new(dvec2(0., -370_000_000.), dvec2(1_000., -29_800.), 7.348e22, 1_737_500.0));

    GAME_STATE.rockets.push(Rocket {
        pos: dvec2(0.0, 100_000_000.0),
        vel: dvec2(0.0, 0.0),
        force: dvec2(0.0, 0.0),
        dry_mass: 1000.0,
        fuel_mass: 0.,
        thrust: 100.0, // should actually be about 1000 times more, but we're currently simulating very quickly
        ..Default::default()
    });

    let rocket = &mut GAME_STATE.rockets[0];
    
    loop {
        frame_start = now(); 
        
        let physics_delta_t = frame_time * GAME_STATE.sim_speed;

        // force assumed to be a zero vector here
        for i in 0..GAME_STATE.bodies.len() {
            for j in (i+1)..GAME_STATE.bodies.len() {
                let body_1 = &mut GAME_STATE.bodies[i];
                let body_2 = &mut GAME_STATE.bodies[j];
                let force = gravity_force(body_1.pos, body_2.pos, body_1.mass, body_2.mass);

                body_1.force += force;
                body_2.force -= force;
            }
        }

        rocket.force = DVec2::ZERO;
        for body in &mut GAME_STATE.bodies {
            body.vel += body.force / body.mass * physics_delta_t;
            body.pos += body.vel * physics_delta_t;

            body.force = DVec2::ZERO;

            rocket.force += gravity_force(rocket.pos, body.pos, rocket.mass(), body.mass);
        }
        if GAME_STATE.engine_working {
            rocket.force += rocket.thrust;
        }
        rocket.vel += rocket.force / rocket.mass() * physics_delta_t;
        rocket.pos += rocket.vel * physics_delta_t;


        GAME_STATE.ups = 1. / frame_time;
        while now() - frame_start < 25_000 {} // Busy waiting because if we give control to Kernel it might eat a lot more time.
        frame_time = (now() - frame_start) as f64 / 1_000_000_000.;
    }
}}
