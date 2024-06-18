#version 330 core

uniform mat4 uView;
uniform mat4 uProj;

// out vec3 tintColor;
out vec2 uv;

layout (location = 0) in vec2 Position;

void main()
{
    gl_Position = uProj * uView * vec4(Position, 0.0, 1.0);
    uv = Position;
}
