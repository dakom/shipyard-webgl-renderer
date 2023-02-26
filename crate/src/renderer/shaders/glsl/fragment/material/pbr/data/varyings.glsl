// Data imported at top level so they are available everywhere 

in vec3 v_position;

#ifdef VARYING_NORMAL
    in vec3 v_normal;
    #ifdef VARYING_TANGENT_MATRIX
        in mat3 v_tbn;
    #endif
#endif

#ifdef VERTEX_COLORS
    in vec4 v_vertex_color;
#endif

// TEXTURES
#ifdef METALLIC_ROUGHNESS_UV_MAP
    in vec2 v_metallic_roughness_uv;
#endif

#ifdef BASE_COLOR_UV_MAP
    in vec2 v_base_color_uv;
#endif

#ifdef NORMAL_UV_MAP
    in vec2 v_normal_uv;
#endif

#ifdef EMISSIVE_UV_MAP
    in vec2 v_emissive_uv;
#endif

#ifdef OCCLUSION_UV_MAP
    in vec2 v_occlusion_uv;
#endif

#ifdef SPECULAR_GLOSSINESS_UV_MAP
    in vec2 v_specular_glossiness_uv;
#endif

#ifdef DIFFUSE_UV_MAP
    in vec2 v_diffuse_uv;
#endif

#ifdef CLEARCOAT_UV_MAP
    in vec2 v_clearcoat_uv;
#endif

#ifdef CLEARCOAT_ROUGHNESS_UV_MAP
    in vec2 v_clearcoat_roughness_uv;
#endif

#ifdef CLEARCOAT_NORMAL_UV_MAP
    in vec2 v_clearcoat_normal_uv;
#endif

    //
#ifdef SHEEN_COLOR_UV_MAP
    in vec2 v_sheen_color_uv;
#endif

#ifdef SHEEN_ROUGHNESS_UV_MAP
    in vec2 v_sheen_roughness_uv;
#endif

#ifdef SPECULAR_UV_MAP
    in vec2 v_specular_uv;
#endif

#ifdef SPECULAR_COLOR_UV_MAP
    in vec2 v_specular_color_uv;
#endif

#ifdef TRANSMISSION_UV_MAP
    in vec2 v_transmission_uv;
#endif

#ifdef VOLUME_THICKNESS_UV_MAP
    in vec2 v_volume_thickness_uv;
#endif

#ifdef IRIDESCENCE_UV_MAP
    in vec2 v_iridescence_uv;
#endif

#ifdef IRIDESCENCE_THICKNESS_UV_MAP
    in vec2 v_iridescence_thickness_uv;
#endif
