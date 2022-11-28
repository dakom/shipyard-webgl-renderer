float set_ambient_occlusion(inout LightOutput light_output) {
    float ao = 1.0;
    // Apply optional PBR terms for additional (optional) shading
#ifdef OCCLUSION_UV_MAP 
    ao = texture(u_occlusion_sampler,  get_occlusion_uv()).r;
    light_output.f_diffuse = mix(light_output.f_diffuse, f_diffuse * ao, u_occlusion_strength);
    // apply ambient occlusion to all lighting that is not punctual
    light_output.f_specular = mix(f_specular, f_specular * ao, u_occlusion_strength);
    light_output.f_sheen = mix(f_sheen, f_sheen * ao, u_occlusion_strength);
    light_output.f_clearcoat = mix(f_clearcoat, f_clearcoat * ao, u_occlusion_strength);
#endif

    return ao;
}
