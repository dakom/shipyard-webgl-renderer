#version 300 es
precision mediump float;

layout(location=0) in vec3 a_vertex;
layout(location=1) in vec3 a_normal;
layout(location=2) in vec2 a_uv;
layout(location=3) in uint a_face;
flat out uint v_face;

layout (std140) uniform camera {
    uniform mat4 view;
    uniform mat4 projection;
} u_camera;

uniform vec3 u_cube_scaler;
uniform mat4 u_model;


void main() {
    mat4 mvp = (u_camera.projection * (u_camera.view * u_model));

    mat4 cube_scaler = mat4(1.0);
    cube_scaler[0][0] = u_cube_scaler[0];
    cube_scaler[1][1] = u_cube_scaler[1];
    cube_scaler[2][2] = u_cube_scaler[2];

    v_face = a_face;

    gl_Position = mvp * (cube_scaler * vec4(a_vertex,1));
}
