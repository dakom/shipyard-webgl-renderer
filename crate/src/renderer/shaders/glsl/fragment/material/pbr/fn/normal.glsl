NormalInfo get_normal_info(Camera camera)
{

    NormalInfo info;

    vec2 uv = get_normal_uv();
    vec3 uv_dx = dFdx(vec3(uv, 0.0));
    vec3 uv_dy = dFdy(vec3(uv, 0.0));


    if (length(uv_dx) + length(uv_dy) <= 1e-6) {
        uv_dx = vec3(1.0, 0.0, 0.0);
        uv_dy = vec3(0.0, 1.0, 0.0);
    }

    vec3 t_ = (uv_dy.t * dFdx(v_position) - uv_dx.t * dFdy(v_position)) /
        (uv_dx.s * uv_dy.t - uv_dy.s * uv_dx.t);

    vec3 n, t, b, ng;

    // Compute geometrical TBN:
    #ifdef VARYING_NORMAL 
        #ifdef VARYING_TANGENT_MATRIX
            // Trivial TBN computation, present as vertex attribute.
            // Normalize eigenvectors as matrix is linearly interpolated.
            t = normalize(v_tbn[0]);
            b = normalize(v_tbn[1]);
            ng = normalize(v_tbn[2]);
        #else
            // Normals are either present as vertex attributes or approximated.
            ng = normalize(v_normal);
            t = normalize(t_ - ng * dot(ng, t_));
            b = cross(ng, t);
        #endif
    #else
        ng = normalize(cross(dFdx(v_position), dFdy(v_position)));
        t = normalize(t_ - ng * dot(ng, t_));
        b = cross(ng, t);
    #endif


    // For a back-facing surface, the tangential basis vectors are negated.
    if (gl_FrontFacing == false)
    {
        t *= -1.0;
        b *= -1.0;
        ng *= -1.0;
    }

    // Compute normals:
    info.geom_normal = ng;

    #ifdef NORMAL_UV_MAP
        info.tex = texture(u_normal_sampler, uv).rgb * 2.0 - vec3(1.0);
        info.tex *= vec3(u_normal_texture_scale, u_normal_texture_scale, 1.0);
        info.tex = normalize(info.tex);
        info.normal = normalize(mat3(t, b, ng) * info.tex);
    #else
        info.normal = ng;
    #endif

    info.tangent = t;
    info.bitangent = b;
    info.view = normalize(camera.position - v_position); 
    info.n_dot_v = clamped_dot(info.normal, info.view);
    info.t_dot_v = clamped_dot(info.tangent, info.view);
    info.b_dot_v = clamped_dot(info.bitangent, info.view);
    return info;
}

