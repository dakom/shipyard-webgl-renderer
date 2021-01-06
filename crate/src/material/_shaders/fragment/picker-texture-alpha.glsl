#version 300 es

precision mediump float;

uniform sampler2D u_sampler;
uniform uvec4 u_entity_color;

in vec2 v_uv;

layout(location = 0) out uvec4 color; 

void main() {
    vec4 tex_color = texture(u_sampler, v_uv);
    float alpha_gate = step(1.0, tex_color.a);

    if(alpha_gate != 1.0) {
        discard;
    }
    color = uvec4(u_entity_color);
}
