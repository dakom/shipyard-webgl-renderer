// TODO - eliminate more branches
LightOutput get_light_output() {
    LightOutput light_output;

    light_output.albedo_sheen_scaling = 1.0;

    return light_output;
}

// https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Khronos/KHR_lights_punctual/README.md#range-property
float getRangeAttenuation(float range, float distance)
{
    if (range <= 0.0)
    {
        // negative range means unlimited
        return 1.0 / pow(distance, 2.0);
    }
    return max(min(1.0 - pow(distance / range, 4.0), 1.0), 0.0) / pow(distance, 2.0);
}


// https://github.com/KhronosGroup/glTF/blob/master/extensions/2.0/Khronos/KHR_lights_punctual/README.md#inner-and-outer-cone-angles
float getSpotAttenuation(vec3 pointToLight, vec3 spotDirection, float outerConeCos, float innerConeCos)
{
    float actualCos = dot(normalize(spotDirection), normalize(-pointToLight));
    if (actualCos > outerConeCos)
    {
        if (actualCos < innerConeCos)
        {
            return smoothstep(outerConeCos, innerConeCos, actualCos);
        }
        return 1.0;
    }
    return 0.0;
}


vec3 getLightIntensity(Light light, vec3 pointToLight)
{
    float rangeAttenuation = 1.0;
    float spotAttenuation = 1.0;

    if (light.type != LIGHT_TYPE_DIRECTIONAL)
    {
        rangeAttenuation = getRangeAttenuation(light.range, length(pointToLight));
    }
    if (light.type == LIGHT_TYPE_SPOT)
    {
        spotAttenuation = getSpotAttenuation(pointToLight, light.direction, light.outer_cone_cos, light.inner_cone_cos);
    }

    return rangeAttenuation * spotAttenuation * light.intensity * light.color;
}


vec3 getPunctualRadianceTransmission(vec3 normal, vec3 view, vec3 pointToLight, float alpha_roughness,
    vec3 f0, vec3 f90, vec3 baseColor, float ior)
{
    float transmissionRougness = apply_ior_to_roughness(alpha_roughness, ior);

    vec3 n = normalize(normal);           // Outward direction of surface point
    vec3 v = normalize(view);             // Direction from surface point to view
    vec3 l = normalize(pointToLight);
    vec3 l_mirror = normalize(l + 2.0*n*dot(-l, n));     // Mirror light reflection vector on surface
    vec3 h = normalize(l_mirror + v);            // Halfway vector between transmission light vector and v

    float D = D_GGX(clamp(dot(n, h), 0.0, 1.0), transmissionRougness);
    vec3 F = F_Schlick(f0, f90, clamp(dot(v, h), 0.0, 1.0));
    float Vis = V_GGX(clamp(dot(n, l_mirror), 0.0, 1.0), clamp(dot(n, v), 0.0, 1.0), transmissionRougness);

    // Transmission BTDF
    return (1.0 - F) * baseColor * D * Vis;
}


vec3 getPunctualRadianceClearCoat(vec3 clearcoatNormal, vec3 v, vec3 l, vec3 h, float VdotH, vec3 f0, vec3 f90, float clearcoatRoughness)
{
    float NdotL = clamped_dot(clearcoatNormal, l);
    float NdotV = clamped_dot(clearcoatNormal, v);
    float NdotH = clamped_dot(clearcoatNormal, h);
    return NdotL * BRDF_specularGGX(f0, f90, clearcoatRoughness * clearcoatRoughness, 1.0, VdotH, NdotL, NdotV, NdotH);
}


vec3 getPunctualRadianceSheen(vec3 sheenColor, float sheenRoughness, float NdotL, float NdotV, float NdotH)
{
    return NdotL * BRDF_specularSheen(sheenColor, sheenRoughness, NdotL, NdotV, NdotH);
}


// Compute attenuated light as it travels through a volume.
vec3 applyVolumeAttenuation(vec3 radiance, float transmissionDistance, vec3 attenuationColor, float attenuationDistance)
{
    if (attenuationDistance == 0.0)
    {
        // Attenuation distance is +∞ (which we indicate by zero), i.e. the transmitted color is not attenuated at all.
        return radiance;
    }
    else
    {
        // Compute light attenuation using Beer's law.
        vec3 attenuationCoefficient = -log(attenuationColor) / attenuationDistance;
        vec3 transmittance = exp(-attenuationCoefficient * transmissionDistance); // Beer's law
        return transmittance * radiance;
    }
}


vec3 getVolumeTransmissionRay(vec3 n, vec3 v, float thickness, float ior, mat4 modelMatrix)
{
    // Direction of refracted light.
    vec3 refractionVector = refract(-v, normalize(n), 1.0 / ior);

    // Compute rotation-independant scaling of the model matrix.
    vec3 modelScale;
    modelScale.x = length(vec3(modelMatrix[0].xyz));
    modelScale.y = length(vec3(modelMatrix[1].xyz));
    modelScale.z = length(vec3(modelMatrix[2].xyz));

    // The thickness is specified in local space.
    return normalize(refractionVector) * thickness * modelScale;
}

Light convert_ubo_light(UboLight ubo_light) {
    Light light;

    light.direction = ubo_light.direction_range.xyz;
    light.range = ubo_light.direction_range.z;

    light.color = ubo_light.color_intensity.rgb;
    light.intensity = ubo_light.color_intensity.a;

    light.position = ubo_light.position_type.xyz;
    light.type = int(ubo_light.position_type.w);

    light.inner_cone_cos = ubo_light.extra[0];
    light.outer_cone_cos = ubo_light.extra[1];
    return light;
}

void apply_light_output(Material material, NormalInfo normal_info, Light light, inout LightOutput light_output, float enabled) {
    vec3 v = normal_info.view;
    vec3 n = normal_info.normal;

    // conditional assignment is not inefficient the way a proper branch is
    // according to some random online comment, at least...
    vec3 point_to_light = light.type == LIGHT_TYPE_DIRECTIONAL ? -light.direction : light.position - v_position;

    // BSTF
    vec3 l = normalize(point_to_light);   // Direction from surface point to light
    vec3 h = normalize(l + v);          // Direction of the vector between l and v, called halfway vector
    float NdotL = clamped_dot(n, l);
    float NdotV = clamped_dot(n, v);
    float NdotH = clamped_dot(n, h);
    float LdotH = clamped_dot(l, h);
    float VdotH = clamped_dot(v, h);

    // instead of branching, use this gate as a multiplier to discount the contribution
    // also only allow if NdotL > 0.0 || NdotV > 0.0 
    float allow_l = when_gt_flt(NdotL, 0.0);
    float allow_v = when_gt_flt(NdotV, 0.0);
    vec3 gate = vec3(or_flt(allow_l, allow_v) * enabled);


    // Calculation of analytical light
    // https://github.com/KhronosGroup/glTF/tree/master/specification/2.0#acknowledgments AppendixB
    vec3 intensity = getLightIntensity(light, point_to_light);
    #ifdef IRIDESCENCE
        light_output.f_diffuse += gate * (intensity * NdotL *  BRDF_lambertianIridescence(material.f0, material.f90, iridescenceFresnel, material.iridescenceFactor, material.c_diff, material.specular_weight, VdotH));
        light_output.f_specular += gate * (intensity * NdotL * BRDF_specularGGXIridescence(material.f0, material.f90, iridescenceFresnel, material.alpha_roughness, material.iridescenceFactor, material.specular_weight, VdotH, NdotL, NdotV, NdotH));
    #else
        light_output.f_diffuse += gate * (intensity * NdotL *  BRDF_lambertian(material.f0, material.f90, material.c_diff, material.specular_weight, VdotH));
        light_output.f_specular += gate * (intensity * NdotL * BRDF_specularGGX(material.f0, material.f90, material.alpha_roughness, material.specular_weight, VdotH, NdotL, NdotV, NdotH));
    #endif

    #ifdef SHEEN
        light_output.f_sheen += gate * (intensity * getPunctualRadianceSheen(material.sheenColorFactor, material.sheenRoughnessFactor, NdotL, NdotV, NdotH));
        light_output.albedo_sheen_scaling = gate * min(
            1.0 - max3(material.sheenColorFactor) * albedoSheenScalingLUT(NdotV, material.sheenRoughnessFactor),
            1.0 - max3(material.sheenColorFactor) * albedoSheenScalingLUT(NdotL, material.sheenRoughnessFactor)
        );
    #endif

    #ifdef CLEARCOAT
        light_output.f_clearcoat += gate * (intensity * getPunctualRadianceClearCoat(material.clearcoatNormal, v, l, h, VdotH, material.clearcoatF0, material.clearcoatF90, material.clearcoatRoughness));
    #endif

    gate = vec3(enabled);
    // BDTF
    #ifdef TRANSMISSION
        // If the light ray travels through the geometry, use the point it exits the geometry again.
        // That will change the angle to the light source, if the material refracts the light ray.
        vec3 transmissionRay = getVolumeTransmissionRay(n, v, material.thickness, material.ior, u_ModelMatrix);
        point_to_light -= transmissionRay;
        l = normalize(point_to_light);

        vec3 intensity = getLightIntensity(light, point_to_light);
        vec3 transmittedLight = intensity * getPunctualRadianceTransmission(n, v, l, material.alpha_roughness, material.f0, material.f90, material.c_diff, material.ior);

        #ifdef VOLUME
            transmittedLight = applyVolumeAttenuation(transmittedLight, length(transmissionRay), material.attenuationColor, material.attenuationDistance);
        #endif

        light_output.f_transmission += gate * transmittedLight;
    #endif

}
