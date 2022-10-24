#version 300 es
precision mediump float;

% INCLUDES_CAMERA %

% INCLUDES_NORMALS %
#ifdef HAS_NORMALS
    layout(location=1) in vec3 a_normal;
    out vec3 v_normal;
#endif

% INCLUDES_TANGENTS %
#ifdef HAS_TANGENTS
    layout(location=2) in vec3 a_tangent;
#endif

% INCLUDES_SKIN_VARS %

% INCLUDES_MORPH_VARS %

layout(location=0) in vec3 a_position;

uniform mat4 u_model;

out vec2 v_uv;
out vec3 v_pos;

void main() {
    vec3 position = a_position;

    #ifdef HAS_NORMALS
        vec3 normal = a_normal;
    #endif

    #ifdef HAS_TANGENTS
        vec3 tangent = a_tangent;
    #endif

    % INCLUDES_SKIN_FN %

    % INCLUDES_MORPH_FN %


    Camera camera = getCamera();

    mat4 mvp = (camera.projection * (camera.view * u_model));

    #ifdef HAS_NORMALS
        v_normal = normal;
    #endif

    gl_Position = mvp * vec4(position, 1);
}

