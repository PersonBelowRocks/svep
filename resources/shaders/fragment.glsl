#version 140

in vec3 v_normal;
in vec3 v_color;

out vec4 f_color;

const vec3 LIGHT = vec3(-0.2, 0.8, 0.1);
const vec3 COLOR = vec3(58.0/255.0, 2.0/255.0, 105.0/255.0);

void main() {
    float lum = max(dot(normalize(v_normal), normalize(LIGHT)), 0.0);

    vec3 color = (0.3 + 0.7 * lum) * COLOR;

    f_color = vec4(color, 1.0);
}