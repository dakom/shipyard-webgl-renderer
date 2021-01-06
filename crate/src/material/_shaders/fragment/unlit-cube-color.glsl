#version 300 es

precision mediump float;

layout(location = 0) out vec4 diffuse;

flat in uint v_face;
uniform vec4 u_colors[6];

void main() {
    diffuse = u_colors[v_face]; 
}
