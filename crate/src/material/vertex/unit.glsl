#version 300 es
precision mediump float;

layout(location=0) in vec2 a_geom_vertex;

out vec2 v_uv;

uniform vec2 u_quad_scaler;
uniform mat4 u_model;
uniform mat4 u_camera;

void main() {

    mat4 mvp = u_camera * u_model; 

    mat4 quad_scaler = mat4(1.0);
    quad_scaler[0][0] = u_quad_scaler[0];
    quad_scaler[1][1] = u_quad_scaler[1];

    gl_Position = mvp * (quad_scaler * vec4(a_geom_vertex,1, 1));
    v_uv = a_geom_vertex;

    gl_Position = mat4(1.0) * vec4(a_geom_vertex, 1, 1);
}
