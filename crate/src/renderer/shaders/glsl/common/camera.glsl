layout (std140) uniform ubo_camera {
    mat4 view;
    mat4 projection;
    mat4 view_projection_inverse;
    mat4 view_projection_direction_inverse;
    vec4 position; // needs to be vec4 for layout
} camera;

struct Camera 
{
    mat4 view;
    mat4 projection;
    mat4 view_projection_inverse;
    mat4 view_projection_direction_inverse;
    vec3 position;
};

Camera getCamera() {
    return Camera(
        camera.view,
        camera.projection,
        camera.view_projection_inverse,
        camera.view_projection_direction_inverse,
        camera.position.xyz
    );
}
