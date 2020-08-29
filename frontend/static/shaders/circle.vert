precision mediump float;

uniform mat4 u_world;
uniform mat4 u_worldViewProjection;

varying vec3 v_color;
attribute vec3 a_position;
attribute vec3 a_color;

void main() {
    v_color = a_color;
    gl_Position = u_worldViewProjection * (u_world * vec4(a_position, 1.0));
}
