#version 100
precision mediump float;

uniform vec2 screen_size;
uniform vec2 cam_pos;
uniform float cam_zoom;
uniform vec3 bodies[50]; // x, y, mass
uniform vec2 bodies_vel[50] //

void main() {
    vec2 frag_screen = vec2(gl_FragCoord.x, screen_size.y - gl_FragCoord.y);

    gl_FragColor = vec4(0.0);

    vec3 color = vec3(0.0);

    for (int i = 0; i < 50; i++) {
        /*vec2 point_pos = points[i];
        vec2 point = point_pos * cam_zoom - (cam_pos - 0.5 * screen_size);

        /*float dist = distance(frag_screen, point);

        if (dist < 1.0) {
            gl_FragColor = vec4(vec3(1.0), 1.0);
            break;
        }*/
    }
}
