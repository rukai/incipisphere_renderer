#version 450

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 1) uniform FSData {
    vec4 color;
} fs_uniforms;

void main() {
    f_color = fs_uniforms.color;
}
