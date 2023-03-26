#version 330 core
in vec3 tileHeight;
out vec4 FragColor;
void main() {
    if (tileHeight.x*255 < 120.0)
        FragColor = vec4(0.0,0.9,0.0,1.0);
    else if (tileHeight.x*255 < 150.0)
        FragColor = vec4(0.0,0.0,0.9,1.0f);
    else
    FragColor = vec4(0.0,0.0,0.5,1.0);
}