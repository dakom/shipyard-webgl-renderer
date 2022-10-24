struct Light
{
    float NdotL;                  // cos angle between normal and light direction
    float NdotH;                  // cos angle between normal and half vector
    float LdotH;                  // cos angle between light direction and half vector
    float VdotH;                  // cos angle between view direction and half vector
    vec3 color;                   
    float falloff;               
    float intensity;
};

Light getDirectionalLight(Vectors vectors, vec3 lightDirection, vec3 color, float intensity) {
    vec3 N = vectors.normal;
    vec3 V = vectors.surfaceToCamera;
    float NdotV = vectors.NdotV;

    vec3 L = -normalize(lightDirection); // Light Direction
    vec3 H = normalize(L+V); // Half vector between both l and v

    float NdotL = clamp(dot(N, L), 0.001, 1.0);
    float NdotH = clamp(dot(N, H), 0.0, 1.0);
    float LdotH = clamp(dot(L, H), 0.0, 1.0);
    float VdotH = clamp(dot(V, H), 0.0, 1.0);

    return Light(
        NdotL,
        NdotH,
        LdotH,
        VdotH,
        color,
        1.0,
        intensity
    );
}
