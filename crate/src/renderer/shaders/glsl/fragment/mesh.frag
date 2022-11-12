#version 300 es

precision mediump float;
precision highp int;

% INCLUDES_HELPERS %
% INCLUDES_CAMERA %
% INCLUDES_VECTORS %
% INCLUDES_LIGHT %
% INCLUDES_MATERIAL %

out vec4 fragment_color; 

void main() {

    % INCLUDES_FVECTORS %

    // hardcoded lighting for now
    vec3 lightDirection = vec3(-1.0, 0.5, -0.25);
    vec3 lightColor = vec3(1.0, 1.0, 1.0);
    float lightIntensity = 2.0;
    vec3 ambientLight = vec3(2.0);
    Light light = getDirectionalLight(fvectors, lightDirection, lightColor, lightIntensity);
    //LIGHTS_FUNCS += `color += getLightColor(pbr, fragment, light);\n`;

    #ifdef PBR_MATERIAL
        Pbr pbr = getPbr();
        vec3 diffuse = pbr_lightColor(pbr, fvectors, light); 
        diffuse = clamp(diffuse + pbr_ambient(pbr, ambientLight), 0.0, 1.0);
    #else
        vec3 diffuse = light.NdotL * light.color * light.falloff * light.intensity;
    #endif


    fragment_color = vec4(diffuse, 1.0);
}
