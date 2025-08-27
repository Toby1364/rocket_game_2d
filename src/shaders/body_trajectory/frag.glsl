#version 100
precision mediump float;

uniform vec2 screen_size;
uniform vec2 cam_pos;
uniform float cam_zoom;
uniform vec3 bodies[50]; // x, y, mass
//uniform vec2 bodies_vel[50] //

void main() {
    vec2 frag = vec2(gl_FragCoord.x, screen_size.y - gl_FragCoord.y);

}
