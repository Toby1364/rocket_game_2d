#version 100
precision mediump float;

uniform vec2 screen_size;
uniform vec2 cam_pos;
uniform float cam_zoom;
uniform vec3 bodies[50]; // x, y, radius

void main() {
    vec2 frag = vec2(gl_FragCoord.x, screen_size.y - gl_FragCoord.y);

    vec3 color = vec3(0.0);

    for (int i = 0; i < 50; i++) {
        vec2 body_pos = bodies[i].xy;
        vec2 center = body_pos * cam_zoom - (cam_pos - 0.5 * screen_size);
        float radius = bodies[i].z * cam_zoom; 

        if (radius <= 0.0) continue;

        float dist = distance(frag, center);

        if (dist < radius) {
            float intensity = dist / radius;

            float edge = 1.0;
            float a = smoothstep(radius, radius - edge, dist);

            gl_FragColor = vec4(vec3(mix(0.0, intensity, a)), 1.0);
            break;
        }
    }
}
