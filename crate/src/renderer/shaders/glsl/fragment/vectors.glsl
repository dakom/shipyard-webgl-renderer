in vec3 v_pos;

% INCLUDES_VECTORS_NORMAL %


#ifdef HAS_NORMALS
    #ifdef HAS_TANGENTS
        in mat3 v_tbn;
    #endif
#endif

#ifdef HAS_NORMALMAP
    uniform sampler2D u_normal_sampler;
    uniform float u_normal_scale;
#endif

struct FragmentVectors 
{
    vec3 normal; // fragment normal
    vec3 surfaceToCamera; //normalized vector from surface point to camera
    vec3 reflection; //reflection vector
    float NdotV; // cos angle between normal and view direction
};

vec3 getNormal(vec2 uv)
{
    // Retrieve the tangent space matrix
    #ifndef HAS_TANGENTS
        vec3 pos_dx = dFdx(v_pos);
        vec3 pos_dy = dFdy(v_pos);
        vec3 tex_dx = dFdx(vec3(uv, 0.0));
        vec3 tex_dy = dFdy(vec3(uv, 0.0));
        vec3 t = (tex_dy.t * pos_dx - tex_dx.t * pos_dy) / (tex_dx.s * tex_dy.t - tex_dy.s * tex_dx.t);

        #ifdef HAS_NORMALS
            vec3 ng = normalize(v_normal);
        #else
            vec3 ng = cross(pos_dx, pos_dy);
        #endif

        t = normalize(t - ng * dot(ng, t));
        vec3 b = normalize(cross(ng, t));
        mat3 tbn = mat3(t, b, ng);
    #else
        mat3 tbn = v_tbn;
    #endif

    #ifdef HAS_NORMALMAP
        vec3 n = texture2D(u_normal_sampler, uv).rgb;
        n = normalize(tbn * ((2.0 * n - 1.0) * vec3(u_normal_scale, u_normal_scale, 1.0)));
    #else
        // The tbn matrix is linearly interpolated, so we need to re-normalize
        vec3 n = normalize(tbn[2].xyz);
    #endif

    return n;
}

FragmentVectors getFragmentVectors(vec2 normal_uv) {
    Camera camera = getCamera();

    vec3 normal = getNormal(normal_uv);
    vec3 surfaceToCamera = normalize(camera.position - v_pos); 
    vec3 reflection = -normalize(reflect(surfaceToCamera, normal));
    float NdotV = abs(dot(normal, surfaceToCamera)) + 0.001;
    return FragmentVectors(normal, surfaceToCamera, reflection, NdotV);
}
