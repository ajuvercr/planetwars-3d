precision mediump float;

uniform float u_time;
uniform float u_aspect;
uniform vec2 u_viewport;
uniform mat4 u_matrix;

varying vec3 v_pos;
varying float v_layer;

attribute vec3 a_position;
attribute float a_layer;

void main() {

    v_layer = a_layer;

    float time = u_time * 0.001;
    mat3 rot2 = mat3(
        cos(time), 0.0, sin(time),
        0.0, 1.0, 0.0,
        -sin(time), 0.0, cos(time));

    time = time * 0.0;
    mat3 rot = mat3(
        1.0, 0.0, 0.0,
        0.0, cos(time), -sin(time),
        0.0, sin(time), cos(time)
    );

    vec2 scale = vec2(u_aspect, 1.0);
    v_pos = a_position;

    vec4 position = u_matrix * vec4(a_position * rot * rot2, 1.0);
    gl_Position = position;
}
