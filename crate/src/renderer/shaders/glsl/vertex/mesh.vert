#version 300 es
precision mediump float;

% INCLUDES_COMMON_MATH %
% INCLUDES_COMMON_CAMERA %

% INCLUDES_POSITION_VARS %
% INCLUDES_NORMAL_VARS %
% INCLUDES_TANGENT_VARS %

% INCLUDES_VERTEX_COLOR_VARS %

% INCLUDES_SKIN_VARS %

% INCLUDES_MORPH_VARS %

% INCLUDES_TEXTURE_VARS %

uniform mat4 u_model;


void main() {
    vec3 position = a_position;

    #ifdef VARYING_NORMAL 
        vec3 normal = a_normal;
    #endif

    #ifdef VARYING_TANGENT 
        vec3 tangent = a_tangent;
    #endif

    % INCLUDES_VERTEX_COLOR_FN %

    % INCLUDES_SKIN_FN %

    % INCLUDES_MORPH_FN %

    Camera camera = getCamera();

    mat4 mvp = (camera.projection * (camera.view * u_model));

    #ifdef VARYING_NORMAL 
        v_normal = normal;
    #endif

    % INCLUDES_ASSIGN_TEXTURE_VARS %

    #ifdef VARYING_POSITION
        v_position = position;
    #endif


    gl_Position = mvp * vec4(position, 1);
}

