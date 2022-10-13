#version 300 es
precision highp float;

uniform sampler2D u_diffuse_sampler;

in vec2 v_uv;

out vec4 color;

void main() {
    vec4 tex_color = texture(u_diffuse_sampler, v_uv);
    color = tex_color;
}
