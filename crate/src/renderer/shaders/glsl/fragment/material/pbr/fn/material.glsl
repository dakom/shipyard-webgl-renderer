void set_material_defaults(inout Material material) {
    // The default index of refraction of 1.5 yields a dielectric normal incidence reflectance of 0.04.
    material.ior = 1.5;
    material.f0 = vec3(0.04);
    // Anything less than 2% is physically impossible and is instead considered to be shadowing. Compare to "Real-Time-Rendering" 4th editon on page 325.
    material.f90 = vec3(1.0);
    material.specular_weight = 1.0;
}

void set_material_base_color(inout Material material) {
    vec4 base_color = vec4(1);
    #if defined(SPECULAR_GLOSSINESS)
        base_color = u_diffuse_factor;
        #ifdef DIFFUSE_UV_MAP
            base_color *= texture(u_diffuse_sampler, get_diffuse_uv());
        #endif
    #elif defined(METALLIC_ROUGHNESS)
        base_color = u_base_color_factor;
        #ifdef BASE_COLOR_UV_MAP
            base_color *= texture(u_base_color_sampler, get_base_color_uv());
        #endif
    #endif

    #ifdef VERTEX_COLORS
        base_color *= v_vertex_color;
    #endif

    material.base_color = base_color;

    #if ALPHAMODE == ALPHAMODE_OPAQUE
        material.base_color.a = 1.0;
    #endif
}

#ifdef METALLIC_ROUGHNESS
void set_material_metallic_roughness(inout Material material) {
    material.metallic = u_metallic_roughness_factors.x;
    material.perceptual_roughness = u_metallic_roughness_factors.y;

    #ifdef METALLIC_ROUGHNESS_UV_MAP
        // Roughness is stored in the 'g' channel, metallic is stored in the 'b' channel.
        // This layout intentionally reserves the 'r' channel for (optional) occlusion map data
        vec4 mr_sample = texture(u_metallic_roughness_sampler, get_metallic_roughness_uv());
        material.perceptual_roughness *= mr_sample.g;
        material.metallic *= mr_sample.b;
    #endif

    // Achromatic f0 based on IOR.
    material.c_diff = mix(material.base_color.rgb,  vec3(0), material.metallic);
    material.f0 = mix(material.f0, material.base_color.rgb, material.metallic);
}
#endif

#ifdef IRIDESCENCE
void set_material_iridescence(inout Material material) {
    material.iridescence_factor = u_iridescence_factor;
    material.iridescence_ior = u_iridescence_ior;
    material.iridescence_thickness = u_iridescence_thickness_maximum;

    #ifdef IRIDESCENCE_UV_MAP
        info.iridescence_factor *= texture(u_iridescence_sampler, get_iridescence_uv()).r;
    #endif

    #ifdef IRIDESCENCE_THICKNESS_UV_MAP
        float thickness_sampled = texture(u_iridescence_thickness_sampler, get_iridescence_thickness_uv()).g;
        float thickness = mix(u_iridescence_thickness_minimum, u_iridescence_thickness_maximum, thickness_sampled);
        material.iridescence_thickness = thickness;
    #endif

    if (material.iridescence_thickness == 0.0) {
        material.iridescence_factor = 0.0;
    }
}
#endif

void set_material_sanitize(inout Material material) {
    material.perceptual_roughness = clamp(material.perceptual_roughness, 0.0, 1.0);
    material.metallic = clamp(material.metallic, 0.0, 1.0);

    // Roughness is authored as perceptual roughness; as is convention,
    // convert to material roughness by squaring the perceptual roughness.
    material.alpha_roughness = material.perceptual_roughness * material.perceptual_roughness;

    // Compute reflectance... but, doesn't seem to be used for anything??
    // float reflectance = max(max(materialInfo.f0.r, materialInfo.f0.g), materialInfo.f0.b);
}

Material get_material(NormalInfo normal_info) {

    Material material;

    set_material_defaults(material);
    set_material_base_color(material);

    #ifdef IOR
        set_material_ior(material);
    #endif

    #ifdef SPECULAR_GLOSSINESS
        set_material_specular_glossiness(material);
    #endif

    #ifdef METALLIC_ROUGHNESS
        set_material_metallic_roughness(material);
    #endif

    #ifdef SHEEN
        set_material_sheen(material);
    #endif

    #ifdef CLEARCOAT
        set_material_clearcoat(material, normal_info);
    #endif

    #ifdef SPECULAR
        set_material_specular(material);
    #endif

    #ifdef TRANSMISSION
        set_material_transmission(material);
    #endif

    #ifdef VOLUME
        set_material_volume(material);
    #endif

    #ifdef IRIDESCENCE
        set_material_iridescence(material);
    #endif

    set_material_sanitize(material);

    return material; 
}
