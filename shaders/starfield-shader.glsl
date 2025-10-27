#version 100
precision highp float;

varying vec4 color;
varying vec2 uv;
varying float iTime;

uniform vec2 iResolution;
uniform float direction_modifier;

#define NUM_LAYERS 5.0

mat2 Rot(float a) {
    float s = sin(a), c = cos(a);
    return mat2(c, -s, s, c);
}

float Hash21(vec2 p) {
    p = fract(p * vec2(123.34, 456.21));
    p += dot(p, p + 45.32);
    return fract(p.x * p.y);
}

float Star(vec2 uv, float flare) {
    float d = length(uv);
    float m = 0.05 / (d + 0.001);
    float rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    m += rays * flare;
    uv *= Rot(3.1415 / 4.0);
    rays = max(0.0, 1.0 - abs(uv.x * uv.y * 1000.0));
    m += rays * 0.3 * flare;
    m *= smoothstep(1.0, 0.2, d);
    return m;
}

vec3 StarLayer(vec2 uv, float t) {
    vec3 col = vec3(0.0);
    vec2 gv = fract(uv) - 0.5;
    vec2 id = floor(uv);

    for (int y = -1; y <= 1; y++) {
        for (int x = -1; x <= 1; x++) {
            vec2 offs = vec2(x, y);
            float n = Hash21(id + offs);
            float size = fract(n * 345.32);
            vec2 pos = gv - offs - vec2(n, fract(n * 42.0)) + 0.5;
            float flicker = sin(t * 6.0 + n * 6.2831) * 0.5 + 0.5;
            float star = Star(pos, smoothstep(0.8, 1.0, size) * 0.7) * flicker;

            // Cosmic color palette
            vec3 color = vec3(
                0.5 + 0.5 * sin(n * 10.0 + t * 0.2),
                0.5 + 0.5 * sin(n * 6.0 + t * 0.3 + 2.0),
                0.5 + 0.5 * sin(n * 8.0 + t * 0.5 + 4.0)
            );
            color = pow(color, vec3(2.2)); // soft gamma curve
            col += star * size * color * 0.8;
        }
    }
    return col;
}

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * iResolution.xy) / iResolution.y;
    float t = iTime * 0.05;
    
    vec3 col = vec3(0.0);
    vec2 dir = vec2(-0.25 + direction_modifier, -1.0);
    float speed = 2.5;

    for (float i = 0.0; i < 1.0; i += 1.0 / NUM_LAYERS) {
        float depth = fract(i + t);
        float scale = mix(30.0, 0.6, depth);
        float fade = depth * smoothstep(1.0, 0.8, depth);
        vec2 motion = dir * t * speed * (1.5 - depth * 1.3);
        col += StarLayer(uv * scale + motion + i * 450.0, iTime) * fade;
    }

    // Add nebula color overlay
    vec3 nebula = vec3(
        0.5 + 0.5 * sin(uv.x * 4.0 + t * 0.3),
        0.4 + 0.4 * sin(uv.y * 3.0 + t * 0.4 + 2.0),
        0.6 + 0.4 * sin((uv.x + uv.y) * 2.0 + t * 0.2)
    );
    nebula *= 0.25;

    col += nebula;

    // Vignette & contrast polish
    float vignette = smoothstep(1.2, 0.4, length(uv));
    col *= vignette;
    col = pow(col, vec3(0.9)); // mild contrast curve
    col = mix(col, vec3(1.0), 0.05); // tiny bloom-style white lift

    gl_FragColor = vec4(col, 1.0);
}
