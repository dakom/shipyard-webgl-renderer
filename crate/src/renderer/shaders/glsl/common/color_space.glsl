const float GAMMA = 2.2;
const float INV_GAMMA = 1.0 / GAMMA;

// linear to sRGB approximation
// see http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html
vec3 linear_to_srgb(vec3 color)
{
    return pow(color, vec3(INV_GAMMA));
}


// sRGB to linear approximation
// see http://chilliant.blogspot.com/2012/08/srgb-approximations-for-hlsl.html
vec3 srgb_to_linear(vec3 srgbIn)
{
    return vec3(pow(srgbIn.xyz, vec3(GAMMA)));
}


vec4 srgb_to_linear(vec4 srgbIn)
{
    return vec4(srgb_to_linear(srgbIn.xyz), srgbIn.w);
}
