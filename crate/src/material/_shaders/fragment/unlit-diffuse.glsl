#version 300 es

precision mediump float;

uniform sampler2D u_sampler;

in vec2 v_uv;

layout(location = 0) out vec4 diffuse; 

void main() {
    diffuse = texture(u_sampler, v_uv);
}
