#version 330 core
layout (location = 0) in vec3 aPos;
out vec3 tileHeight;
void main() {
    gl_Position = vec4(aPos.x, aPos.y, 0.0, 1.0);
    tileHeight = vec3(aPos.z,aPos.z,aPos.z);
}