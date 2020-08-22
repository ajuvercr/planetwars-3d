precision mediump float;

varying vec3 v_pos;
varying float v_layer;
uniform float u_time;

void main() {
    float r = sin(u_time * 0.0003);
    float g = sin(u_time * 0.0005);
    float b = sin(u_time * 0.0007);

        if(fract(v_layer) < 0.01) {
        gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        gl_FragColor = vec4(vec3(r, g, b), 1.0);
    }

    // gl_FragColor = vec4(1.0);
}
