#version 140

uniform float width;
uniform float height;
uniform float radius;
uniform float blur;
uniform vec4 col;

in vec2 vertex_pos;
out vec4 color;

void main() {
    float dx = max(abs(vertex_pos.x - width/2.0) - width / 2.0 + radius + blur, 0);
    float dy = max(abs(vertex_pos.y - height/2.0) - height / 2.0 + radius + blur, 0);
    float a = smoothstep(radius-0.6-blur, radius+0.6+blur, sqrt(dx*dx + dy*dy));
    color = col; //* vec4(1.0, 1.0, 1.0, 1.0 - a);
    color.w *= (1.0 - a);
    //color = vec4(1.0, 0.0, 0.0, 1.0);
}
