#version 430 core

uniform float uTime;

// Based on: http://wdobbie.com/post/gpu-text-rendering-with-vector-textures/

struct Curve {
    vec2 p0, p1, p2;
};

layout(std430, binding = 1) readonly buffer CurvesBuffer {
    Curve curves[];
};

const vec3 tintColor = vec3(1., 1., 1.);

in vec2 uv;
flat in uint start;
flat in uint count;

out vec4 result;

vec3 transColor(float uTime) {
    float x = gl_FragCoord.x / 600 * 0.96592582628 + gl_FragCoord.y / 800 * 0.2588190451;
    
    // Add animation based on time (uTime)
    x += uTime * .5;
    x -= floor(x);

    vec3 colors[] = {
        vec3(0.392, 0.549, 0.929),
        vec3(0.905, 0.298, 0.588),
        vec3(1.0),
        vec3(0.905, 0.298, 0.588),
        vec3(0.392, 0.549, 0.929),
    };

    vec3 col1 = colors[int(floor(x * 4.0))];
    vec3 col2 = colors[int(ceil(x * 4.0))];

    x *= 4;
    x -= floor(x);

    return vec3( (1-x) * col1 + x * col2 );
}

float computeCoverage(float inverseDiameter, vec2 p0, vec2 p1, vec2 p2) {
    if (p0.y > 0 && p1.y > 0 && p2.y > 0) return 0.0;
    if (p0.y < 0 && p1.y < 0 && p2.y < 0) return 0.0;

    // Note: Simplified from abc formula by extracting a factor of (-2) from b.
    vec2 a = p0 - 2 * p1 + p2;
    vec2 b = p0 - p1;
    vec2 c = p0;

    float t0, t1;
    if (abs(a.y) >= 1e-5) {
        // Quadratic segment, solve abc formula to find roots.
        float radicand = b.y * b.y - a.y * c.y;
        if (radicand <= 0) return 0.0;

        float s = sqrt(radicand);
        t0 = (b.y - s) / a.y;
        t1 = (b.y + s) / a.y;
    } else {
        // Linear segment, avoid division by a.y, which is near zero.
        float t = p0.y / (p0.y - p2.y);
        if (p0.y < p2.y) {
            t0 = -1.0;
            t1 = t;
        } else {
            t0 = t;
            t1 = -1.0;
        }
    }

    float alpha = 0;

    if (t0 >= 0 && t0 < 1) {
        float x = (a.x * t0 - 2.0 * b.x) * t0 + c.x;
        alpha += clamp(x * inverseDiameter + 0.5, 0, 1);
    }

    if (t1 >= 0 && t1 < 1) {
        float x = (a.x * t1 - 2.0 * b.x) * t1 + c.x;
        alpha -= clamp(x * inverseDiameter + 0.5, 0, 1);
    }

    return alpha;
}

vec2 rotate(vec2 v) {
    return vec2(v.y, -v.x);
}

void main() {
    float alpha = 0;

    vec2 diameter = fwidth(uv);
    vec2 inverseDiameter = 1.0 / diameter;

    for (int i = 0; i < count; i++) {
        Curve curve = curves[start + i];

        vec2 p0 = curve.p0 - uv;
        vec2 p1 = curve.p1 - uv;
        vec2 p2 = curve.p2 - uv;

        alpha -= computeCoverage(inverseDiameter.x, p0, p1, p2);
        alpha -= computeCoverage(inverseDiameter.y, rotate(p0), rotate(p1), rotate(p2));
    }
//     for (int i = 0; i < count; i++) {
//         Curve curve = curves[start + i];
//
//         vec2 p0 = curve.pMade cursor 0 - uv;
//         vec2 p1 = curve.p1 - uv;
//         vec2 p2 = curve.p2 - uv;
//
//         p0.y -= diameter.y / 4.;
//         p1.y -= diameter.y / 4.;
//         p2.y -= diameter.y / 4.;
//         alpha -= computeCoverage(inverseDiameter.x, p0, p1, p2);
//
//         p0.y += diameter.y / 2.;
//         p1.y += diameter.y / 2.;
//         p2.y += diameter.y / 2.;
//         alpha -= computeCoverage(inverseDiameter.x, p0, p1, p2);
//
//         p0.y -= diameter.y / 4.;
//         p1.y -= diameter.y / 4.;
//         p2.y -= diameter.y / 4.;
//         p0 = rotate(p0);
//         p1 = rotate(p1);
//         p2 = rotate(p2);
//
//         p0.x -= diameter.x / 4.;
//         p1.x -= diameter.x / 4.;
//         p2.x -= diameter.x / 4.;
//         alpha -= computeCoverage(inverseDiameter.y, p0, p1, p2);
//
//         p0.x += diameter.x / 2.;
//         p1.x += diameter.x / 2.;
//         p2.x += diameter.x / 2.;
//         alpha -= computeCoverage(inverseDiameter.y, p0, p1, p2);
//     }
    vec4 color = transColor(uTime);
    alpha = clamp(0.5 * alpha, 0.0, 1.0);
    result = color * alpha;
}