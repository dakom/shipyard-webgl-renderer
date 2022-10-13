#version 300 es

precision mediump float;
precision highp int;

% INCLUDES_CAMERA %
% INCLUDES_NORMALS %
% INCLUDES_VECTORS %
% INCLUDES_LIGHT %

out vec4 diffuse; 

void main() {

    Vectors vectors = getVectors();
    vec3 lightDirection = vec3(.5, 1.0, 0.0);
    vec3 lightColor = vec3(1.0, 1.0, 1.0);
    float lightIntensity = 1.0;
    Light light = getDirectionalLight(vectors, lightDirection, lightColor, lightIntensity);
    //LIGHTS_FUNCS += `color += getLightColor(pbr, fragment, light);\n`;
    vec3 color = light.NdotL * light.color * light.falloff * light.intensity;
    diffuse = vec4(color, 1.0);
}
