vec4 final_color(Material material, LightOutput light_output) {
    light_output.f_emissive = u_emissive_factor;

    #ifdef EMISSIVE_STRENGTH
        light_output.f_emissive *= u_emissive_strength;
    #endif

    #ifdef EMISSIVE_UV_MAP
        light_output.f_emissive *= texture(u_emissive_sampler, get_emissive_uv()).rgb;
    #endif

    // Layer blending

    float clearcoat_factor = 0.0;
    vec3 clearcoat_fresnel = vec3(0);

    #ifdef CLEARCOAT
        clearcoat_factor = materialInfo.clearcoat_factor;
        clearcoat_fresnel = F_Schlick(material.clearcoat_F0, material.clearcoat_F90, clampedDot(material._clearcoat_normal, v));
        f_clearcoat = f_clearcoat * clearcoat_factor;
    #endif

    #ifdef TRANSMISSION
        vec3 diffuse = mix(light_output.f_diffuse, light_output.f_transmission, material.transmission_factor);
    #else
        vec3 diffuse = light_output.f_diffuse;
    #endif

        vec3 color = vec3(0);

    #ifdef UNLIT
        color = material.base_color.rgb;
    #else
        color = light_output.f_emissive + diffuse + light_output.f_specular;
        color = light_output.f_sheen + color * light_output.albedo_sheen_scaling;
        color = color * (1.0 - clearcoat_factor * clearcoat_fresnel) + light_output.f_clearcoat;
    #endif

    #if ALPHAMODE == ALPHAMODE_MASK
        // Late discard to avoid samplig artifacts. See https://github.com/KhronosGroup/glTF-Sample-Viewer/issues/267
        if (material.base_color.a < u_alpha_cutoff) {
            discard;
        }
        material.base_color.a = 1.0;
    #endif

    #ifdef LINEAR_OUTPUT
        return vec4(color.rgb, material.base_color.a);
    #else
        return vec4(tone_map(color), material.base_color.a);
    #endif
}
