#version 300 es
precision highp float;
uniform highp usampler2D u_tex;

in vec2 v_uv;
out vec4 color;

void main() {
    uvec4 data = texture(u_tex, v_uv);

    color = vec4(data) / float(0xFFFF);
}
