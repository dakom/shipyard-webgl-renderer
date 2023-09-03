#version 300 es
precision highp float;
 
// https://webgl2fundamentals.org/webgl/lessons/webgl-skybox.html


% INCLUDES_COMMON_CAMERA %
% INCLUDES_COMMON_COLOR_SPACE %
 
uniform samplerCube u_sampler;

in vec2 v_uv;
out vec4 outColor;
 
void main() {
    vec4 t = camera.view_projection_direction_inverse * vec4(v_uv, 0.0, 1.0);
    outColor = texture(u_sampler, normalize(t.xyz / t.w));
    outColor = vec4(linear_to_srgb(outColor.rgb), 1.0);
}