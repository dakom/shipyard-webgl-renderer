// Data imported at top level so they are available everywhere 

// Metallic Roughness
uniform vec2 u_metallic_roughness_factors; // x: metallic, y: roughness
uniform vec4 u_base_color_factor;
uniform sampler2D u_metallic_roughness_sampler;
uniform mat3 u_metallic_roughness_uv_transform;
uniform sampler2D u_base_color_sampler;
uniform mat3 u_base_color_uv_transform;

// Specular Glossiness
uniform vec3 u_specular_factor;
uniform vec4 u_diffuse_factor;
uniform float u_glossiness_factor;
uniform sampler2D u_diffuse_sampler;
uniform mat3 u_diffuse_uv_transform;
uniform sampler2D u_glossiness_sampler;
uniform mat3 u_glossiness_uv_transform;

// Sheen
uniform float u_sheen_roughness_factor;
uniform vec3 u_sheen_color_factor;
uniform sampler2D u_sheen_color_sampler;
uniform mat3 u_sheen_color_uv_sampler;
uniform sampler2D u_sheen_roughness_sampler;
uniform mat3 u_sheen_roughness_uv_transform;

// Clearcoat
uniform float u_clearcoat_factor;
uniform float u_clearcoat_roughness_factor;
uniform sampler2D u_clearcoat_sampler;
uniform mat3 u_clearcoat_uv_sampler;
uniform sampler2D u_clearcoat_roughness_sampler;
uniform mat3 u_clearcoat_roughness_uv_transform;
uniform sampler2D u_clearcoat_normal_sampler;
uniform mat3 u_clearcoat_normal_uv_transform;
uniform float u_clearcoat_normal_scale;

// Specular
uniform vec3 u_khr_specular_color_factor;
uniform float u_khr_specular_factor;
uniform sampler2D u_specular_sampler;
uniform mat3 u_specular_uv_transform;
uniform sampler2D u_specular_color_sampler;
uniform mat3 u_specular_color_uv_transform;

// Transmission
uniform float u_transmission_factor;
uniform sampler2D u_transmission_sampler;
uniform mat3 u_transmission_uv_sampler;
uniform sampler2D u_transmission_framebuffer_sampler;
uniform ivec2 u_transmission_framebuffer_size;
uniform ivec2 u_screen_size;

// Volume
uniform float u_thickness_factor;
uniform vec3 u_attentuation_color;
uniform float u_attenuation_distance;
uniform sampler2D u_thickness_sampler;
uniform mat3 u_thickness_uv_transform;

// Iridescence
uniform float u_iridescence_factor;
uniform float u_iridescence_ior;
uniform float u_iridescence_thickness_min;
uniform float u_iridescence_thickness_max;
uniform sampler2D u_iridescence_sampler;
uniform mat3 u_iridescence_uv_transform;
uniform sampler2D u_iridescence_thickness_sampler;
uniform mat3 u_iridedscence_thickness_uv_transform;


// PBR Next IOR
uniform float u_ior;

// Alpha mode
uniform float u_alpha_cutoff;

// IBL
uniform int u_mip_count;
uniform samplerCube u_lambertian_env_sampler;
uniform samplerCube u_ggx_env_sampler;
uniform sampler2D u_ggx_lut;
uniform samplerCube u_charlie_env_sampler;
uniform sampler2D u_charlie_lut;
uniform sampler2D u_sheen_e_lut;
uniform mat3 u_env_rotation;
uniform float u_env_intensity;

// Normals
uniform sampler2D u_normal_sampler;
uniform float u_normal_scale;
uniform mat3 u_normal_uv_transform;

// Emissive
uniform float u_emissive_strength;
uniform vec3 u_emissive_factor;
uniform sampler2D u_emissive_sampler;
uniform mat3 u_emissive_uv_transform;

// Ambient Occlusion
uniform sampler2D u_occlusion_sampler;
uniform float u_occlusion_strength;
uniform mat3 u_occlusion_transform;

#ifdef MAX_LIGHTS
layout (std140) uniform ubo_lights {
    float active_len; // has hidden padding! 
    UboLight light[MAX_LIGHTS];
} u_lights;
#endif
