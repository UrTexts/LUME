#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec2 aTexCoord;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec2 TexCoord;

void main() {
    vec4 world_pos = model * vec4(aPos, 1.0);
    world_pos.xyz = floor(world_pos.xyz * 10.0) / 10.0; // Vertex snapping

    gl_Position = projection * view * world_pos;
    TexCoord = aTexCoord;
}

