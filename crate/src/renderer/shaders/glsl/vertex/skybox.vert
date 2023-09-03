#version 300 es
precision mediump float;

layout(location=0) in vec2 a_vertex;

out vec2 v_uv;

void main() {
    vec2 vertex = (a_vertex * 2.0) - 1.0;
    gl_Position = mat4(1.0) * vec4(vertex, 1, 1);
    v_uv = a_vertex;
}