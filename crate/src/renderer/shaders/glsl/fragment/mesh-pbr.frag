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

    #ifdef IBL
        set_ibl(material, iridescence, light_output);
    #endif

    #ifdef OCCLUSION_UV_MAP
        float ao = set_ambient_occlusion(light_output);
    #endif

    fragment_color = vec4(0.0, 0.0, 0.0, 1.0);

    #ifdef MAX_LIGHTS
        LightOutput light_output = get_light_output();
        % INCLUDES_LIGHT_MAIN %
        fragment_color = final_color(material, light_output);
    #endif

    #ifdef DEBUG_NORMALS
        fragment_color = vec4(normal_info.normal, 1.0); 
    #endif
    #ifdef DEBUG_OCCLUSION
        fragment_color = vec(linear_to_srgb(vec3(ao)), 1.0); 
    #endif
}

