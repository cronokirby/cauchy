#version 150 core

out vec4 color;

#define PI (3.14159265358);

vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

vec3 hsl2rgb(vec3 c) {
    float v = c.z + c.y * min(c.z, 1 - c.z);
    float s;
    if (v == 0) {
        s = 0;
    } else {
        s = 2 - 2 * c.z / v;
    }
    return hsv2rgb(vec3(c.x, s, v));
}

vec2 cart2polar(vec2 cart) {
    return vec2(atan(cart.y, cart.x), length(cart));
}

vec2 sinx(vec2 cart) {
    float re = sin(cart.x) * cosh(cart.y);
    float im = cos(cart.x) * sinh(cart.y);
    return cart2polar(vec2(re, im));
}

void main() {
    vec2 pos = vec2(gl_FragCoord.x, gl_FragCoord.y) / 100 - 4;
    
    vec2 polar = sinx(pos);

    float h = polar.x / 2 / PI + 0.5;
    float s = 1.0;
    float l = pow(0.2, 1 / (polar.y + 1));

    color = vec4(hsl2rgb(vec3(h, s, l)), 1.0);
}
