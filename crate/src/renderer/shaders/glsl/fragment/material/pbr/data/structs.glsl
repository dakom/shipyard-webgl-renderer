// Data imported at top level so they are available everywhere 
struct Material
{
    float ior;
    float perceptual_roughness;      // roughness value, as authored by the model creator (input to shader)
    vec3 f0;                        // full reflectance color (n incidence angle)

    float alpha_roughness;           // roughness mapped to a more linear change in the roughness (proposed by [2])
    vec3 c_diff;

    vec3 f90;                       // reflectance color at grazing angle
    float metallic;

    vec4 base_color;

    float sheen_roughness_factor;
    vec3 sheen_color_factor;

    vec3 clearcoat_F0;
    vec3 clearcoat_F90;
    float clearcoat_factor;
    vec3 clearcoat_normal;
    float clearcoat_roughness;

    // KHR_materials_specular 
    float specular_weight; // product of specularFactor and specularTexture.a

    float transmission_factor;

    float volume_thickness;
    vec3 attenuation_color;
    float attenuation_distance;

    // KHR_materials_iridescence
    float iridescence_factor;
    float iridescence_ior;
    float iridescence_thickness;
};

struct NormalInfo {
    vec3 view; // surface to camera, a.k.a. v
    vec3 geom_normal;   // Geometry normal, a.k.a. ng
    vec3 tangent;    // Geometry tangent, a.k.a. t
    vec3 bitangent;    // Geometry bitangent, a.k.a. b
    vec3 normal;    // Shading normal, a.k.a. n
    vec3 tex; // Normal from texture, scaling is accounted for., a.k.a. ntex
    float n_dot_v;
    float t_dot_v;
    float b_dot_v;
};

struct Iridescence
{
    vec3 fresnel;
    vec3 f0;
};

struct LightOutput {
    vec3 f_specular;
    vec3 f_diffuse;
    vec3 f_emissive;
    vec3 f_clearcoat;
    vec3 f_sheen;
    vec3 f_transmission;
    float albedo_sheen_scaling;

};
// KHR_lights_punctual extension.
// see https://github.com/KhronosGroup/glTF/tree/master/extensions/2.0/Khronos/KHR_lights_punctual
//
struct UboLight
{
    vec4 direction_range;
    vec4 color_intensity;
    vec4 position_type;
    vec4 extra; // inner_cone_cos, outer_cone_cos
};

struct Light
{
    vec3 direction;
    float range;

    vec3 color;
    float intensity;

    vec3 position;
    float inner_cone_cos;

    float outer_cone_cos;
    int type;
};

const int LIGHT_TYPE_DIRECTIONAL = 0;
const int LIGHT_TYPE_POINT = 1;
const int LIGHT_TYPE_SPOT = 2;

#define ALPHAMODE_OPAQUE 0
#define ALPHAMODE_MASK 1
#define ALPHAMODE_BLEND 2
