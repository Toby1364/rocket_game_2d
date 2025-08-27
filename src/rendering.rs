use macroquad::prelude::*;
use macroquad::miniquad::*;

use crate::{

    GAME_STATE,
};


pub struct Cam {
    pub base_pos: DVec2,
    pub off_pos: DVec2,
    pub pos: DVec2,
    pub zoom: f64,
}
impl Cam {
    pub fn off(&self) -> DVec2 {
        self.pos + dvec2((-screen_width()/2.) as f64, (-screen_height()/2.) as f64)
    }
    pub fn update(&mut self) { self.pos = self.base_pos + self.off_pos }
}

enum Track {
    Body(usize)
}

pub async fn main() { unsafe {
    let font = load_ttf_font_from_bytes(include_bytes!("fonts/NaturalMono-Regular.ttf")).ok();

    let body_shader = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("shaders/default/vert.glsl"),
            fragment: include_str!("shaders/body/frag.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("screen_size", UniformType::Float2),
                UniformDesc::new("cam_pos", UniformType::Float2),
                UniformDesc::new("cam_zoom", UniformType::Float1),
                UniformDesc::new("bodies", UniformType::Float3).array(50),
            ],
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
                ),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    let body_traj_shader = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("shaders/default/vert.glsl"),
            fragment: include_str!("shaders/body_trajectory/frag.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("screen_size", UniformType::Float2),
                UniformDesc::new("cam_pos", UniformType::Float2),
                UniformDesc::new("cam_zoom", UniformType::Float1),
                UniformDesc::new("bodies", UniformType::Float2).array(100),
            ],
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
                ),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    let ui_shader = load_material(
        ShaderSource::Glsl {
            vertex: include_str!("shaders/default/vert.glsl"),
            fragment: include_str!("shaders/ui/frag.glsl"),
        },
        MaterialParams {
            uniforms: vec![
                UniformDesc::new("screen_size", UniformType::Float2),
                UniformDesc::new("sim_speed", UniformType::Float1),
            ],
            pipeline_params: PipelineParams {
                color_blend: Some(BlendState::new(
                    Equation::Add,
                    BlendFactor::Value(BlendValue::SourceAlpha),
                    BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
                ),
                ..Default::default()
            },
            ..Default::default()
        },
    )
    .unwrap();

    let mut cam = Cam {
        base_pos: DVec2::ZERO,
        off_pos: DVec2::ZERO,
        pos: DVec2::ZERO,
        zoom: 0.000_01,
    };

    let mut hold_start: Option<(DVec2, DVec2)> = None;
    let mut tracking: Option<Track> = None;

    loop {
        next_frame().await;
        clear_background(BLACK);

        let sw = screen_width();
        let sh = screen_height();

        /* Cam Control */ {
            let zoom_factor = 1. + ((mouse_wheel().1 as f64) * 0.001);
            cam.zoom *= zoom_factor;
            if cam.zoom < 0.000_000_001 { cam.zoom = 0.000_000_001 }

            cam.off_pos *= zoom_factor;

            let mp = dvec2(mouse_position().0 as f64, mouse_position().1 as f64);

            if is_mouse_button_down(MouseButton::Right) {
                if let Some(hs) = hold_start {
                    cam.off_pos = hs.0 + (hs.1 - mp)
                }
                else {
                    hold_start = Some((cam.off_pos, mp))
                }
            }
            else {
                hold_start = None;
            }

            /* Tracking captuere */ {
                if is_mouse_button_pressed(MouseButton::Left) {
                    for i in 0..GAME_STATE.bodies.len() {
                        if mp.distance(GAME_STATE.bodies[i].draw_pos(&cam)) < (GAME_STATE.bodies[i].radius * cam.zoom).max(5.) {
                            cam.off_pos = DVec2::ZERO; tracking = Some(Track::Body(i)); break;
                        }
                    }
                }

                match tracking {
                    Some(Track::Body(i)) => { cam.base_pos = GAME_STATE.bodies[i].pos * cam.zoom }

                    _ => {}
                }
            }

            cam.update();
        }

        body_shader.set_uniform("screen_size", vec2(sw, sh));
        body_shader.set_uniform("cam_pos", vec2(cam.pos.x as f32, cam.pos.y as f32));
        body_shader.set_uniform("cam_zoom", cam.zoom as f32);

        body_traj_shader.set_uniform("screen_size", vec2(sw, sh));
        body_traj_shader.set_uniform("cam_pos", vec2(cam.pos.x as f32, cam.pos.y as f32));
        body_traj_shader.set_uniform("cam_zoom", cam.zoom as f32);

        /* Body Rendering */ {
            let mut body_info = Vec::new();
            for body in &GAME_STATE.bodies { body_info.push(vec3(body.pos.x as f32, body.pos.y as f32, (body.radius) as f32)) }
            while body_info.len() < 50 { body_info.push(Vec3::ZERO) }

            body_shader.set_uniform_array("bodies", &body_info);

            gl_use_material(&body_shader); draw_rectangle(-sw/2., -sh/2., sw, sh, WHITE);

            gl_use_material(&body_traj_shader); draw_rectangle(-sw/2., -sh/2., sw, sh, WHITE);
        }

        ui_shader.set_uniform("sim_speed", GAME_STATE.sim_speed as f32);
        ui_shader.set_uniform("screen_size",vec2(sw, sh));

        gl_use_material(&ui_shader); draw_rectangle(-sw/2., -sh/2., sw, sh, WHITE);



        gl_use_default_material();

        let rp = (GAME_STATE.rockets[0].pos * cam.zoom) - cam.off();
        draw_circle(rp.x as f32, rp.y as f32, /*2_000_000. * cam.zoom as f32*/ 20., RED);

        let rp = (GAME_STATE.rockets[1].pos * cam.zoom) - cam.off();
        draw_circle(rp.x as f32, rp.y as f32, /*2_000_000. * cam.zoom as f32*/ 20., YELLOW);

        /* Interface */ {
            draw_text_ex(
                &format!("FPS: {}", get_fps()), 
                10., 80., 
                TextParams {
                    font: font.as_ref(),
                    font_size: 20,
                    color: WHITE,
                    ..Default::default()
                }
            );
            
            draw_text_ex(
                &format!("UPS: {:.0}", GAME_STATE.ups), 
                10., 110., 
                TextParams {
                    font: font.as_ref(),
                    font_size: 20,
                    color: WHITE,
                    ..Default::default()
                }
            );

            if is_mouse_button_down(MouseButton::Left) {
                let mp = Vec2::from(mouse_position());

                if mp.y < 60. && mp.x > 20. && mp.x < 240. {
                    if mp.x < 30. { GAME_STATE.sim_speed = 1. }
                    else {
                        GAME_STATE.sim_speed = 5_000_000. * (((mp.x as f64) - 30.) / 200.).powf(2.).min(1.);
                    }
                }
            }

            /*draw_rectangle(0., 0.,sw, 40., Color::new(0.6, 0.6, 0.6, 1.));
            draw_rectangle(30., 5., 200., 30., BLACK);*/
        }
    }
}}
