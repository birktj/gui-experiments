#version 140

uniform mat4 matrix;
uniform float width;
uniform float height;
uniform float x;
uniform float y;

in vec2 position;

out vec2 vertex_pos;

void main() {
    vec2 position2 = position *  vec2(width, height);
    vertex_pos = position2;
    position2.y += y;
    position2.x += x;
    position2.y *= -1.0;
    gl_Position = vec4(position2, 0.0, 1.0) * matrix;
}

