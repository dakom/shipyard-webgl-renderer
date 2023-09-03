#version 300 es

precision highp float;

out vec2 tex_coord;

void main(void) 
{
    /*
        See: https://www.saschawillems.de/blog/2016/08/13/vulkan-tutorial-on-rendering-a-fullscreen-quad-without-buffers/
        or: https://wallisc.github.io/rendering/2021/04/18/Fullscreen-Pass.html
        or: https://michaldrobot.com/2014/04/01/gcn-execution-patterns-in-full-screen-passes/

        the basic idea is that instead of drawing a quad, which requires attributes and 2 triangles
        we can just draw 3 vertices to create a giant triangle at: (-1, -1), (3, -1), (-1, 3)
        and this perfectly circumscibes the quad at (-1, -1), (1, -1), (-1, 1), (1, 1)
        in other words it perfectly covers the entire screen

        it does go beyond, but that's fine, the gpu will just discard fragments outside the quad
        and not spend time rendering them (see above articles for more details)
    */
    float x = float((gl_VertexID & 1) << 2);
    float y = float((gl_VertexID & 2) << 1);
    tex_coord.x = x * 0.5;
    tex_coord.y = y * 0.5;
    gl_Position = vec4(x - 1.0, y - 1.0, 0, 1);
}