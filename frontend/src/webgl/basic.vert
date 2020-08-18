precision mediump float;

uniform float u_time;
uniform float u_aspect;
uniform vec2 u_viewport;

attribute vec3 a_position;

void main() {

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

    vec4 position = vec4(a_position * rot * rot2, 1.0);
    vec2 scale = vec2(u_aspect, 1.0);
    gl_Position = vec4(position.xy / u_viewport / scale, 1.0, 1.0);
}
