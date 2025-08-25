use macroquad::prelude::*;

use crate::{

    GAME_STATE,
};


pub struct Cam {
    pub pos: DVec2,
    pub zoom: f64,
}
impl Cam {
    pub fn off(&self) -> DVec2 {
        self.pos + dvec2((-screen_width()/2.) as f64, (-screen_height()/2.) as f64)
    }
}


pub async fn main() { unsafe {
    let font = load_ttf_font_from_bytes(include_bytes!("fonts/NaturalMono-Regular.ttf")).ok();

    let mut cam = Cam {
        pos: DVec2::ZERO,
        zoom: 1.,
    };

    let mut hold_start: Option<(DVec2, DVec2)> = None;

    loop {
        next_frame().await;
        clear_background(BLACK);

        /* Cam Control */ {
            let zoom_factor = 1. + ((mouse_wheel().1 as f64) * 0.001);
            cam.zoom *= zoom_factor;
            if cam.zoom < 0.01 { cam.zoom = 0.01 }

            cam.pos *= zoom_factor;

            if is_mouse_button_down(MouseButton::Right) {
                if let Some(hs) = hold_start {
                    cam.pos = hs.0 + (hs.1 - dvec2(mouse_position().0 as f64, mouse_position().1 as f64))
                }
                else {
                    hold_start = Some((cam.pos, dvec2(mouse_position().0 as f64, mouse_position().1 as f64)))
                }
            }
            else {
                hold_start = None;
            }
        }

        /* Rendering */ {
            for body in &GAME_STATE.bodies { body.draw(&cam) }
        }

        draw_text_ex(
            &format!("FPS: {}", get_fps()), 
            10., 30., 
            TextParams {
                font: font.as_ref(),
                font_size: 20,
                color: WHITE,
                ..Default::default()
            }
        );
        
        draw_text_ex(
            &format!("UPS: {:.0}", GAME_STATE.ups), 
            10., 60., 
            TextParams {
                font: font.as_ref(),
                font_size: 20,
                color: WHITE,
                ..Default::default()
            }
        );
    }
}}
