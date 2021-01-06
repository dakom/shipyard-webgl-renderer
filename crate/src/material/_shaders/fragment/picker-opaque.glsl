#version 300 es

precision mediump float;

uniform uvec4 u_entity_color;
layout(location = 0) out uvec4 color;

void main() {
    color = uvec4(u_entity_color);
}
