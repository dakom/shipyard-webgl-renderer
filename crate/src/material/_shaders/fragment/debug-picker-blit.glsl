#version 300 es

precision mediump float;

uniform highp usampler2D u_sampler;

in vec2 v_uv;

layout(location = 0) out vec4 color; 

void main() {
    uvec4 tex_color = texture(u_sampler, v_uv);
    color = vec4(tex_color);// / float(0xFFFF);

}
