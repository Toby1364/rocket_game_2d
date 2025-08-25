#version 100
precision mediump float;

uniform vec2 screen_size;
uniform vec3 bodies[50]; // x, y, radius

void main() {
    vec2 frag_pos = vec2(gl_FragCoord.x, screen_size.y - gl_FragCoord.y);

    vec3 color = vec3(0.0);

    for (int i = 0; i < 50; i++) {
        vec2 center = bodies[i].xy;
        float radius = bodies[i].z;

        float dist = distance(frag_pos, center);

        float alpha = smoothstep(radius, radius - 1.0, dist);
        color = mix(color, vec3(1.0), alpha);
    }

    gl_FragColor = vec4(color, 1.0);
}
