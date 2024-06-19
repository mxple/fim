#version 430 core

uniform uint CircleResolution;
uniform mat4 uView;
uniform mat4 uProj;

out vec4 color;
out vec2 TexCoord;

struct CircleData {
    vec4 position_rad;
    vec4 tintCol;
};

layout(std430, binding = 0) readonly buffer CircleBuffer {
    CircleData Circles[];
};

vec2 getPosition(uint vertex) {
    if (vertex == 0) return vec2(0., 0.);
    float angle = 6.28318 * vertex / CircleResolution;
    return vec2(cos(angle), sin(angle));
}

// Vertex shader is called (CircleResolution * 3) times to produce CircleResolution 
// triangles. IBO: 0 1 2 0 2 3 0 3 4 0 4 5 ... 0 CircleResolution 1
void main() {
    uint index = gl_VertexID / (CircleResolution + 1);
    uint vertex = gl_VertexID % (CircleResolution + 1); // 0-17
    CircleData circle = Circles[index];

    mat2 scaleMat = mat2(
	circle.position_rad.w, 0,
	0, circle.position_rad.w
    );
 //    mat2 scaleMat = mat2(
	// 0.01, 0,
	// 0, 0.01
 //    );

    gl_Position = uProj * uView * vec4((circle.position_rad.xy + scaleMat * getPosition(vertex)), 0., 1.0);
    color = circle.tintCol;
}
