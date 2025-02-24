#version 330 core
in vec2 TexCoord;
out vec4 FragColor;
uniform sampler2D texture1;

void main() {
    vec4 texColor = texture(texture1, TexCoord);
    float intensity = dot(texColor.rgb, vec3(0.3, 0.6, 0.1));
    float dither = mod(gl_FragCoord.x + gl_FragCoord.y, 4.0) / 4.0;
    texColor.rgb = floor(texColor.rgb * 4.0) / 4.0 + dither * 0.1;

    FragColor = texColor;
}

