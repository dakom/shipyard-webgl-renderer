#version 300 es

precision mediump float;

flat in uint v_face;
uniform vec4 u_colors[6];

out vec4 color;

void main() {
    color = u_colors[v_face]; 
}
