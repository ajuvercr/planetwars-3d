precision mediump float;

uniform mat4 u_world;
uniform mat4 u_worldViewProjection;

varying vec3 v_normal;

attribute vec3 a_position;
attribute vec3 a_normal;

void main() {
    v_normal = mat3(u_world) * a_normal;

    gl_Position = u_worldViewProjection * (u_world * vec4(a_position, 1.0));
}
