#version 300 es
precision mediump float;

% INCLUDES_COMMON_MATH %
% INCLUDES_COMMON_CAMERA %

% INCLUDES_NORMALS %
#ifdef ATTRIBUTE_NORMALS
    layout(location=1) in vec3 a_normal;
    out vec3 v_normal;
#endif

% INCLUDES_TANGENTS %
#ifdef ATTRIBUTE_TANGENTS
    layout(location=2) in vec3 a_tangent;
#endif

% INCLUDES_SKIN_VARS %

% INCLUDES_MORPH_VARS %

% INCLUDES_TEXTURE_VARS %

% INCLUDES_MATERIAL_VARS %

layout(location=0) in vec3 a_position;

uniform mat4 u_model;

out vec3 v_position;

void main() {
    vec3 position = a_position;

    #ifdef ATTRIBUTE_NORMALS
        vec3 normal = a_normal;
    #endif

    #ifdef ATTRIBUTE_TANGENTS
        vec3 tangent = a_tangent;
    #endif

    % INCLUDES_SKIN_FN %

    % INCLUDES_MORPH_FN %


    Camera camera = getCamera();

    mat4 mvp = (camera.projection * (camera.view * u_model));

    #ifdef ATTRIBUTE_NORMALS
        v_normal = normal;
    #endif

    % INCLUDES_ASSIGN_MATERIAL_VARS %

    // not 100% sure about this one..
    v_position = position;

    gl_Position = mvp * vec4(position, 1);
}

