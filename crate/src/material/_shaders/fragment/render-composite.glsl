#version 300 es
precision highp float;

uniform sampler2D u_entities_sampler;
uniform sampler2D u_diffuse_sampler;

in vec2 v_uv;

layout(location = 0) out vec4 color;

void main() {
    vec4 tex_color = texture(u_diffuse_sampler, v_uv);
    vec4 entities_color = texture(u_entities_sampler, v_uv);
    color = tex_color;

}
