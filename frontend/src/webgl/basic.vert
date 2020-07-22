precision mediump float;

uniform float u_time;
uniform float u_aspect;
attribute vec3 a_position;

void main() {
    vec3 pos = normalize(a_position);
    float time = u_time * 0.001;
    mat3 rot2 = mat3(
        cos(time), 0.0, sin(time),
        0.0, 1.0, 0.0,
        -sin(time), 0.0, cos(time));

    mat3 rot = mat3(
        1.0, 0.0, 0.0,
        0.0, cos(time), -sin(time),
        0.0, sin(time), cos(time)
    );
    vec3 scale = vec3(u_aspect, 1.0, 1.0);
    gl_Position = vec4(pos * rot * rot2 / scale * 0.7, 1.0);
}
