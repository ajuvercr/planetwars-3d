precision mediump float;

uniform mat4 u_world;
uniform mat4 u_worldViewProjection;

varying vec3 v_pos;
varying float v_layer;

attribute vec3 a_position;
attribute float a_layer;

void main() {

    v_layer = a_layer;

    v_pos = a_position;

    gl_Position = u_worldViewProjection *u_world *  vec4(a_position, 1.0);
}
