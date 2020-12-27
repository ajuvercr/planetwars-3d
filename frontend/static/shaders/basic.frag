precision mediump float;

varying vec3 v_normal;

uniform float u_time;
uniform vec3 u_reverseLightDirection;
uniform vec3 u_color;

void main() {
    vec3 normal = normalize(v_normal);
    float light = dot(normal, u_reverseLightDirection);

    gl_FragColor = vec4(u_color, 1.0);
    gl_FragColor.rgb *= light;
}
