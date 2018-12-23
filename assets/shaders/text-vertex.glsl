#version 140

uniform mat4 matrix;
uniform float width;
uniform float height;
uniform float x;
uniform float y;
uniform float t_w;
uniform float t_h;
uniform float t_x;
uniform float t_y;

in vec2 position;

out vec2 vertex_pos;

void main() {
    vertex_pos = position * vec2(t_w, t_h);
    vertex_pos.x += t_x;
    vertex_pos.y += t_y;
    vec2 position2 = position * vec2(width, height);
    position2.y += y;
    position2.x += x;
    position2.y *= -1.0;
    gl_Position = vec4(position2, 0.0, 1.0) * matrix;
}

