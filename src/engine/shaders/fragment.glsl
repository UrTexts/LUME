#version 330 core

in vec3 fragColor; // Color passed from vertex shader

out vec4 FragColor;

void main()
{
    FragColor = vec4(fragColor, 1.0); // Set the final pixel color
}

