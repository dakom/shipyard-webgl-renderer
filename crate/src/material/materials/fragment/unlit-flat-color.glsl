#version 300 es

precision mediump float;

uniform uvec4 u_picker_color;

out uvec4 color;

void main() {
    color = u_picker_color;
    /*
    uint max_16 = 0xFFFF;

    color = vec4(
            u_picker_color[0] / max_16,
            u_picker_color[1] / max_16,
            u_picker_color[2] / max_16,
            u_picker_color[3] / max_16);
            */
    //color = vec4(1.0, 0.0, 0.0, 1.0);
}
