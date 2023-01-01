#version 300 es

precision mediump float;
precision highp int;

% INCLUDES_COMMON_MATH %
% INCLUDES_COMMON_CAMERA %
% INCLUDES_MATERIAL_DEPS %

out vec4 fragment_color; 

void main() {
    NormalInfo normal_info = get_normal_info();
    Material material = get_material(normal_info);
    Iridescence iridescence = get_iridescence(material, normal_info);
    LightOutput light_output = get_light_output();

    #ifdef IBL
        set_ibl(material, iridescence, light_output);
    #endif

    #ifdef OCCLUSION_UV_MAP
        float ao = set_ambient_occlusion(light_output);
    #endif

    // quick ambient hack
    light_output.f_diffuse = vec3(0.3) * material.c_diff;
    #ifdef MAX_LIGHTS
        % INCLUDES_LIGHT_MAIN %
    #endif

    fragment_color = final_color(material, light_output);

    #ifdef DEBUG_NORMALS
        fragment_color = vec4(normal_info.normal, 1.0); 
    #endif
    #ifdef DEBUG_OCCLUSION
        fragment_color = vec(linear_to_srgb(vec3(ao)), 1.0); 
    #endif
}

