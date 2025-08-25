#version 100
precision mediump float;

uniform vec2 screen_size;
uniform vec2 cam_pos;
uniform float cam_zoom;
uniform vec3 bodies[50]; // x, y, radius

void main() {
    vec2 frag_screen = gl_FragCoord.xy - 0.5 * screen_size; 
    vec2 world_pos = frag_screen / cam_zoom + cam_pos;

    vec3 color = vec3(0.0);

    for (int i = 0; i < 50; i++) {
        vec2 center = bodies[i].xy;
        float radius = bodies[i].z;

        if (radius <= 0.0) continue;

        float dist = distance(world_pos, center);

        if (dist < radius) {
            float intensity = dist / radius; 

            float edge = 1.0 / cam_zoom;
            float alpha = smoothstep(radius, radius - edge, dist);

            color = mix(color, vec3(intensity), alpha);
        }
    }

    gl_FragColor = vec4(color, 1.0);
}
