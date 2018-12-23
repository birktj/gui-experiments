#version 140

uniform sampler2D tex;
uniform vec4 col;

in vec2 vertex_pos;
out vec4 color;

void main() {
    color = col;
    //color = texture(tex, vertex_pos);
    color.w *= texture(tex, vertex_pos).x;
}
