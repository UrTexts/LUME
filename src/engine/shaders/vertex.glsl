#version 330 core

layout(location = 0) in vec3 aPos; // Vertex position
layout(location = 1) in vec3 aColor; // Vertex color

out vec3 fragColor; // Color to pass to fragment shader

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main()
{
    fragColor = aColor; // Pass color to fragment shader
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}

