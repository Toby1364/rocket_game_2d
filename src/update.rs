use macroquad::prelude::*;
use std::time::SystemTime;

use crate::{
    // Here we will import other files.
    body::*,

    GAME_STATE,
};


fn now() -> u128 { SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() }

// is our whole update function unsafe
pub fn main() { unsafe {
    let mut frame_time = 1.;
    let mut frame_start;

    GAME_STATE.bodies.push(Body::new(dvec2(0., 0.), dvec2(0., 0.), 100., 100.));

    GAME_STATE.bodies.push(Body::new(dvec2(0., 200.), dvec2(50., 0.), 10., 10.));

    loop {
        frame_start = now(); // Sorry for the pull, so how do you want to do rockets and bodies?

        // It's probably okay if everything is just in "the universe", just store positions of celestials with some absolute center like the sun or whatever
        // Ideally planets should orbit each other yes
        // In 2D i feel comfortable trying to do N-body simulation. I think it should be reasonably possible for me to do
        // I don't feel much need for like, fancy data structures there, I don't think it needs to store planets somehow in relation to their "main" body
        // For now a celestial is a circle. Should have some hope to have an atmosphere, i think.
        // Probably not, density is a thing. A gas giant is less dense than a rocky planet i believe
        // I think hardcoding like, 10 masses and 10 radii is fine
        //
        
        // But I would probably make that an Enum, saying if it's solid or gas?
        // Will mass and size be one thing?
        // Yeah, okay, so we will store, pos vec2, vel vec2, size and mass, and do we want to define the surface somehow si it's not just flat? For now probably doesn't matter.

        // how do you want to define a rocket, will it be for no just an ininitly small point? 

        // Here you can run updates, so for example `for rocket in &mut GAME_STATE.rockets { rocket.update(frame_time) }`

        // Do something:
        //&mut GAME_STATE.bodies;
        
        // Can't have multiple mutable borrows, use indexes, for i in 0..GAME_STATE.bodies.len() // Can't borrow `x` as reference while `x` is borrowed as mutable.
        // i dont think i need 2 mutables, im only planning on updating body 1. do i use two?
        // maybe i do, hold up
        
        // not sure if rust has neat cartesian products
        for i in 0..GAME_STATE.bodies.len() {
            for j in 0..GAME_STATE.bodies.len() { // shouldn't you go from i..GAME_STATE.bodies.len() to avoid repetition?
                if i == j { continue; }
                //wow i cannot think at all. uh, yeah i can see what you mean, literally cant tell if I need all posibilities for something or not
                //i..len is what, going through all the... whats, pairs? 
            }
        }

        // Do I do physics updates here, like moving bodies by velocity * delta, like applying forces? Yup, frame_time is delta. 
        // Do you want to maybe store next_pos so you can iterate over them, and only after set the new positions and velocities?
        
        


        

        GAME_STATE.ups = 1. / frame_time;
        while now() - frame_start < 50_000 {} // Busy waiting because if we give control to Kernel it might eat a lot more time.
        frame_time = (now() - frame_start) as f32 / 1_000_000_000.;
    }
}}
