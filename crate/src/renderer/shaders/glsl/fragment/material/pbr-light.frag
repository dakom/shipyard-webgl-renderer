// Basic Lambertian diffuse
// Implementation from Lambert's Photometria https://archive.org/details/lambertsphotome00lambgoog
// See also [1], Equation 1
//
vec3 pbr_diffuse(Pbr pbr)
{
    return pbr.diffuseColor / PI;
}

// The following equation models the Fresnel reflectance term of the spec equation (aka F())
// Implementation of fresnel from [4], Equation 15
vec3 pbr_specularReflection(Pbr pbr, FragmentVectors fvectors, Light light)
{
    return pbr.reflectance0 + (pbr.reflectance90 - pbr.reflectance0) * pow(clamp(1.0 - light.VdotH, 0.0, 1.0), 5.0);
}

// This calculates the specular geometric attenuation (aka G()),
// where rougher material will reflect less light back to the viewer.
// This implementation is based on [1] Equation 4, and we adopt their modifications to
// alphaRoughness as input as originally proposed in [2].
float pbr_geometricOcclusion(Pbr pbr, FragmentVectors fvectors, Light light)
{
    float NdotL = light.NdotL;
    float NdotV = fvectors.NdotV;
    float r = pbr.alphaRoughness;

    float attenuationL = 2.0 * NdotL / (NdotL + sqrt(r * r + (1.0 - r * r) * (NdotL * NdotL)));
    float attenuationV = 2.0 * NdotV / (NdotV + sqrt(r * r + (1.0 - r * r) * (NdotV * NdotV)));
    return attenuationL * attenuationV;
}

// The following equation(s) model the distribution of microfacet normals across the area being drawn (aka D())
// Implementation from "Average Irregularity Representation of a Roughened Surface for Ray Reflection" by T. S. Trowbridge, and K. P. Reitz
// Follows the distribution function recommended in the SIGGRAPH 2013 course notes from EPIC Games [1], Equation 3.
float pbr_microfacetDistribution(Pbr pbr, FragmentVectors fvectors, Light light)
{
    float roughnessSq = pbr.alphaRoughness * pbr.alphaRoughness;
    float f = (light.NdotH * roughnessSq - light.NdotH) * light.NdotH + 1.0;
    return roughnessSq / (PI * f * f);
}

vec3 pbr_lightColor(Pbr pbr, FragmentVectors fvectors, Light light) {
    // Calculate the shading terms for the microfacet specular shading model
    vec3 F = pbr_specularReflection(pbr, fvectors, light);
    float G = pbr_geometricOcclusion(pbr, fvectors, light);
    float D = pbr_microfacetDistribution(pbr, fvectors, light);

    // Calculation of analytical lighting contribution
    vec3 diffuseAmt = pbr_diffuse(pbr);
    vec3 specAmt = F * (G * D);

    vec3 diffuseContrib = (1.0 - F) * diffuseAmt; 
    vec3 specContrib = specAmt / (4.0 * light.NdotL * fvectors.NdotV);

    // Obtain final intensity as reflectance (BRDF) scaled by the energy of the light (cosine law)
    //vec3 color = light.NdotL * light.color * light.falloff * light.intensity * (diffuseContrib + specContrib);
    vec3 color = light.NdotL * light.color * light.falloff * light.intensity * (diffuseContrib + specContrib);
    //color = color + (light.color * light.falloff * light.intensity * specContrib);


    return color;
}

vec3 pbr_ambient(Pbr pbr, vec3 ambientLight) {
    vec3 diffuseAmt = pbr_diffuse(pbr);
    return diffuseAmt * ambientLight;
}
