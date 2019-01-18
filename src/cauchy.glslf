#version 150 core

out vec4 color;

uniform bool u_dark_plot;
uniform Tokens {
    int u_tokens[128];
};
uniform Floats {
    float u_floats[128];
};

const float TAU = 6.283185307179586;


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

float smooth_fract(float f) {
    return (1 + sin(TAU * f)) / 2;
}

// Complex functions

vec2 cart2polar(vec2 cart) {
    return vec2(atan(cart.y, cart.x), length(cart));
}

vec2 c_mul(vec2 z1, vec2 z2) {
    return vec2(z1.x * z2.x - z1.y * z2.y, z1.y * z2.x + z1.x * z2.y);
}

vec2 c_inv(vec2 cart) {
    return vec2(cart.x, -cart.y) / dot(cart, cart);
}

vec2 c_div(vec2 z1, vec2 z2) {
    return c_mul(z1, c_inv(z2));
}

vec2 c_exp(vec2 z) {
    return exp(z.x) * vec2(cos(z.y), sin(z.y));
}

vec2 c_ln(vec2 z) {
    vec2 polar = cart2polar(z);
    return vec2(log(polar.y), polar.x);
}

vec2 c_pow(vec2 z1, vec2 z2) {
    return c_exp(c_mul(c_ln(z1), z2));
}

vec2 c_sin(vec2 cart) {
    float re = sin(cart.x) * cosh(cart.y);
    float im = cos(cart.x) * sinh(cart.y);
    return vec2(re, im);
}

vec2 c_gamma(vec2 cart) {
    float e = exp(1.0);
    vec2 z_over_e = c_pow(c_div(cart, vec2(e, 0)), cart);
    return sqrt(TAU) * c_mul(c_pow(cart, vec2(0.5, 0)), z_over_e);
}

vec2 c_cos(vec2 cart) {
    float re = cos(cart.x) * cosh(cart.y);
    float im = sin(cart.x) * sinh(cart.y);
    return vec2(re, -im);
}


void main() {
    vec2 num = vec2(gl_FragCoord.x, gl_FragCoord.y) / 100 - 3;
    vec2 stack[40];
    int stack_i = -1;

    for (int t_i = 0; t_i < u_tokens.length(); ++t_i) {
        bool should_exit = false;
        vec2 a;
        vec2 b;
        int token = u_tokens[t_i];
        switch (token) {
            case 0:
                should_exit = true;
                break;
            case 1:
                ++stack_i;
                stack[stack_i] = num;
                break;
            case 2:
                ++stack_i;
                stack[stack_i] = vec2(0, 1);
                break;
            case 3:
                b = stack[stack_i];
                --stack_i;
                stack[stack_i] += b;
                break;
            case 4:
                b = stack[stack_i];
                --stack_i;
                stack[stack_i] -= b;
                break;
            case 5:
                b = stack[stack_i];
                --stack_i;
                a = stack[stack_i];
                stack[stack_i] = c_mul(a, b);
                break;
            case 6:
                b = stack[stack_i];
                --stack_i;
                a = stack[stack_i];
                stack[stack_i] = c_div(a, b);
                break;
            case 7:
                b = stack[stack_i];
                --stack_i;
                a = stack[stack_i];
                stack[stack_i] = c_pow(a, b);
                break;
            case 8:
                b = stack[stack_i];
                stack[stack_i] = c_sin(b);
                break;
            case 9:
                b = stack[stack_i];
                stack[stack_i] = c_exp(b);
                break;
            case 10:
                b = stack[stack_i];
                stack[stack_i] = c_ln(b);
                break;
            case 11:
                b = stack[stack_i];
                stack[stack_i] = c_gamma(b);
                break;
            case 12:
                b = stack[stack_i];
                stack[stack_i] = c_cos(b);
                break;
            default:
                if (token >= 0) {
                    continue;
                }
                int float_i = ~token;
                ++stack_i;
                stack[stack_i] = vec2(u_floats[float_i], 0);
                break;
        }
        if (should_exit) {
            break;
        }
    }
    
    vec2 polar = cart2polar(stack[stack_i]);

    float h = polar.x / TAU;
    float s = 1.0;
    
    if (u_dark_plot) {
        float v = smooth_fract(log(polar.y));
        color = vec4(hsv2rgb(vec3(h, s, v)), 1.0);
    } else {
        float l = pow(0.2, 1 / (polar.y + 1));
        color = vec4(hsl2rgb(vec3(h, s, l)), 1.0);
    }
}
