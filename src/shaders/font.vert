#version 430 core

uniform mat4 uView;
uniform mat4 uProj;

// out vec3 tintColor;
out vec2 uv;
flat out uint start;
flat out uint count;
out vec4 color;

struct QuadData {
    vec4 color;
    vec2 pos;
    vec2 uv0;
    vec2 uv1;
    uint start;
    uint count; 
};

layout(std430, binding = 0) readonly buffer QuadBuffer {
    QuadData quads[];
};

void main() {
    uint index = gl_VertexID / 4;
    uint vertex = gl_VertexID % 4;
    QuadData quad = quads[index];

    vec2 vert0 = (quad.pos + quad.uv0);
    vec2 vert1 = (quad.pos + quad.uv1);

    const vec2 positions[4] = vec2[](
	vec2(vert0.x, vert1.y),
	vec2(vert1.x, vert1.y),
	vec2(vert1.x, vert0.y),
	vec2(vert0.x, vert0.y)
    );
    const vec2 uvs[4] = vec2[](
	vec2(quad.uv0.x, quad.uv1.y),
	quad.uv1,
	vec2(quad.uv1.x, quad.uv0.y),
	quad.uv0
    );

    gl_Position = uProj * uView * vec4(positions[vertex], 0.0, 1.0);
    uv = uvs[vertex];
    start = quad.start;
    count = quad.count;
    color = quad.color;
}

