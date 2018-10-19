#version 450

layout(location = 0) in vec3 position;

layout(set = 0, binding = 0) uniform VSData {
    mat4 transform;
} vs_uniforms;

void main() {
    gl_Position = vs_uniforms.transform * vec4(position, 1.0);
}
