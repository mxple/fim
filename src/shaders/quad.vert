#version 430 core

uniform mat4 uView;
uniform mat4 uProj;

out vec4 color;

struct QuadData {
    vec4 col;
    vec2 pos[4];
};

layout(std430, binding = 0) readonly buffer QuadBuffer {
    QuadData quads[];
};

void main() {
    uint index = gl_VertexID / 4;
    uint vertex = gl_VertexID % 4;
    QuadData quad = quads[index];

    gl_Position = uProj * uView * vec4(quad.pos[vertex], 0., 1.0);
    color = quad.col;
}
