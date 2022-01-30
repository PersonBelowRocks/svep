#version 140

in vec3 position;
in vec3 normal;

in vec3 pos;
in vec3 chunk;

out vec3 v_position;
out vec3 v_normal;
out vec3 v_color;

uniform mat4 camera;
uniform mat4 perspective;
uniform mat4 model;

void main() {
    v_position = position;

    mat4 modelview = camera * model;

    v_normal = transpose(inverse(mat3(modelview))) * normal;
    v_color = vec3(float(chunk.x)/128.0, 0.0, 0.0);
    gl_Position = perspective * modelview * vec4(position + pos + (chunk * 128.0), 1.0);
}