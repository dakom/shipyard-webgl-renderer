vec2 get_normal_uv()
{
    #ifdef NORMAL_UV_MAP
        vec3 uv = vec3(v_normal_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef NORMAL_UV_TRANSFORM
        uv = u_normal_uv_transform * uv;
    #endif

    return uv.xy;
}


vec2 get_emissive_uv()
{
    #ifdef EMISSIVE_UV_MAP
        vec3 uv = vec3(v_emissive_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef EMISSIVE_UV_TRANSFORM
        uv = u_emissive_uv_transform * uv;
    #endif

    return uv.xy;
}


vec2 get_occlusion_uv()
{
    #ifdef OCCLUSION_UV_MAP
        vec3 uv = vec3(v_occlusion_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef OCCLUSION_UV_TRANSFORM
        uv = u_occlusion_uv_transform * uv;
    #endif

    return uv.xy;
}


vec2 get_base_color_uv()
{
    #ifdef BASE_COLOR_UV_MAP
        vec3 uv = vec3(v_base_color_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef BASE_COLOR_UV_TRANSFORM
        uv = u_base_color_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_metallic_roughness_uv()
{
    #ifdef METALLIC_ROUGHNESS_UV_MAP
        vec3 uv = vec3(v_metallic_roughness_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef METALLIC_ROUGHNESS_UV_TRANSFORM
        uv = u_metallic_roughness_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_specular_glossiness_uv()
{
    #ifdef SPECULAR_GLOSSINESS_UV_MAP
        vec3 uv = vec3(v_specular_glossiness_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef SPECULAR_GLOSSINESS_UV_TRANSFORM
        uv = u_specular_glossiness_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_diffuse_uv()
{
    #ifdef DIFFUSE_UV_MAP
        vec3 uv = vec3(v_diffuse_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef DIFFUSE_UV_TRANSFORM
        uv = u_diffuse_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_clearcoat_uv()
{
    #ifdef CLEARCOAT_UV_MAP
        vec3 uv = vec3(v_clearcoat_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef CLEARCOAT_UV_TRANSFORM
        uv = u_clearcoat_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_clearcoat_roughness_uv()
{
    #ifdef CLEARCOAT_ROUGHNESS_UV_MAP
        vec3 uv = vec3(v_clearcoat_roughness_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef CLEARCOAT_ROUGHNESS_UV_TRANSFORM
        uv = u_clearcoat_roughness_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_clearcoat_normal_uv()
{
    #ifdef CLEARCOAT_NORMAL_UV_MAP
        vec3 uv = vec3(v_clearcoat_normal_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef CLEARCOAT_NORMAL_UV_TRANSFORM
        uv = u_clearcoat_normal_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_sheen_color_uv()
{
    #ifdef SHEEN_COLOR_UV_MAP
        vec3 uv = vec3(v_sheen_color_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef SHEEN_COLOR_UV_TRANSFORM
        uv = u_sheen_color_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_sheen_roughness_uv()
{
    #ifdef SHEEN_ROUGHNESS_UV_MAP
        vec3 uv = vec3(v_sheen_roughness_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef SHEEN_ROUGHNESS_UV_TRANSFORM
        uv = u_sheen_roughness_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_specular_uv()
{
    #ifdef SPECULAR_UV_MAP
        vec3 uv = vec3(v_specular_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef SPECULAR_UV_TRANSFORM
        uv = u_specular_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_specular_color_uv()
{
    #ifdef SPECULAR_COLOR_UV_MAP
        vec3 uv = vec3(v_specular_color_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef SHEEN_COLOR_UV_TRANSFORM
        uv = u_specular_color_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_transmission_uv()
{
    #ifdef TRANSMISSION_UV_MAP
        vec3 uv = vec3(v_transmission_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef TRANSMISSION_UV_TRANSFORM
        uv = u_transmission_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_volume_thickness_uv()
{
    #ifdef VOLUME_THICKNESS_UV_MAP
        vec3 uv = vec3(v_volume_thickness_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef VOLUME_THICKNESS_UV_TRANSFORM
        uv = u_volume_thickness_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_iridescence_uv()
{
    #ifdef IRIDESCENCE_UV_MAP
        vec3 uv = vec3(v_iridescence_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef IRIDESCENCE_UV_TRANSFORM
        uv = u_iridescence_uv_transform * uv;
    #endif

    return uv.xy;
}

vec2 get_iridescence_thickness_uv()
{
    #ifdef IRIDESCENCE_THICKNESS_UV_MAP
        vec3 uv = vec3(v_iridescence_thickness_uv, 1.0);
    #else
        vec3 uv = vec3(0.0);
    #endif

    #ifdef IRIDESCENCE_THICKNESS_UV_TRANSFORM
        uv = u_iridescence_thickness_uv_transform * uv;
    #endif

    return uv.xy;
}
