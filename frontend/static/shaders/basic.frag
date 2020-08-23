precision mediump float;

varying vec3 v_pos;
varying float v_layer;
uniform float u_time;

void main() {
    float r = sin(v_layer);
    float g = cos(v_layer);
    float b = sin(u_time * 0.0007);

    r = v_layer / 5.0;
    g = v_layer / 5.0;

    gl_FragColor = vec4(vec3(fract(v_layer)), 1.0);
    return;

    if(fract(v_layer) < 0.01) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        gl_FragColor = vec4(vec3(r, 1.0, 1.0), 1.0);
    }

    // gl_FragColor = vec4(1.0);
}
