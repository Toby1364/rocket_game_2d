#version 100
precision lowp float;

uniform vec2 screen_size;
uniform float sim_speed;

void main() {
    vec2 frag = vec2(gl_FragCoord.x, screen_size.y - gl_FragCoord.y);

    if (frag.y < 60.0) {
        if (frag.y > 15.0 && frag.y < 30.0 && frag.x > 30.0 && frag.x < 230.0) {
            gl_FragColor = vec4(vec3(0.0), 1.0);

            float perc = sqrt(sim_speed / 5000000.0);

            if (frag.x - 30.0 < 200.0 * perc) {
                gl_FragColor = vec4(mix(vec3(0.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0), (frag.x - 30.0) / 200.0), 1.0);
            }
        } 
        else {
            float shade = smoothstep(0.0, 0.5, 1.0 - frag.y / 60.0) - smoothstep(0.0, 0.8, 1.0 - frag.y / 10.0) + (0.8 * smoothstep(0.0, 1.0, ((20.0 - frag.y) / 20.0)));
            gl_FragColor = vec4(vec3(0.2, 0.3, 1.0) * shade, 1.0);
        }
    }
}
