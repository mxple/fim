#version 430 core

uniform uint BezierResolution;

out vec4 color;
out vec2 TexCoord;

struct BezierData {
    vec4 p12;
    vec4 p3;
    vec4 col;
};

layout(std430, binding = 0) readonly buffer BezierBuffer {
    BezierData Beziers[];
};

vec2 getPosition(uint vertex, BezierData curve) {
    float t = float(vertex) / float(BezierResolution);
    float omt = 1.0 - t;
    return omt * omt * curve.p12.xy
	+ 2.0 * omt * t * curve.p12.zw
	+ t * t * curve.p3.xy;
}

void main() {
    uint index = gl_VertexID / (BezierResolution + 1);
    uint vertex = gl_VertexID % (BezierResolution + 1);
    BezierData bezier = Beziers[index];

    gl_Position = vec4(getPosition(vertex, bezier), 0.0, 1.0);
    color = bezier.col;
}
