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

    let body_shader = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("shaders/body/vert.glsl"),
            fragment: include_str!("shaders/body/frag.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("screen_size", UniformType::Float2),
                UniformDesc::new("cam_pos", UniformType::Float2),
                UniformDesc::new("cam_zoom", UniformType::Float1),
                UniformDesc::new("bodies", UniformType::Float3).array(50),
            ],
            ..Default::default()
        },
    )
    .unwrap();

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

        body_shader.set_uniform("screen_size", vec2(screen_width(), screen_height()));
        body_shader.set_uniform("cam_pos", vec2(cam.pos.x as f32, cam.pos.y as f32));
        body_shader.set_uniform("cam_zoom", cam.zoom as f32);

        /* Body Rendering */ {
            let mut body_info = Vec::new();
            for body in &GAME_STATE.bodies { body_info.push(vec3(body.draw_pos(&cam).x as f32, body.draw_pos(&cam).y as f32, (body.radius * cam.zoom) as f32)) }
            while body_info.len() < 50 { body_info.push(Vec3::ZERO) }

            body_shader.set_uniform_array("bodies", &body_info);

            gl_use_material(&body_shader);
            draw_rectangle(-screen_width()/2., -screen_height()/2., screen_width(), screen_height(), WHITE);
        }

        gl_use_default_material();

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
