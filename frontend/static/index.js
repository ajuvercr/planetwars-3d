import init, { WebGl, CameraHandle } from "./wasm.js"

window.addEventListener("gamepadconnected", connecthandler);
window.addEventListener("gamepaddisconnected", disconnecthandler);

function disconnecthandler(e) {
    delete controllers[e.gamepad.index];
}

function connecthandler(e) {
    controllers[e.gamepad.index] = e.gamepad;
}

const controllers = {};
function scangamepads() {
    var gamepads = navigator.getGamepads ? navigator.getGamepads() : (navigator.webkitGetGamepads ? navigator.webkitGetGamepads() : []);
    for (var i = 0; i < gamepads.length; i++) {
        if (gamepads[i]) {
            controllers[gamepads[i].index] = gamepads[i];
        }
    }
}

if (!'ongamepadconnected' in window) {
    setInterval(scangamepads, 500);
}

const movement = {
    up: false,
    down: false,
    left: false,
    right: false,
    forward: false,
    back: false,
};

async function doInit() {
    await init();

    const canvas = document.getElementById("canvas");

    let webGL = await new WebGl("canvas").init_renderer();

    window.addEventListener('resize', () => webGL.resize());

    /** @type {CameraHandle} */
    let handle = webGL.camera_handle();
    document.addEventListener("keydown", e => {
        switch (e.key) {
            case "w":
                movement.back = true;
                break;
            case "s":
                movement.forward = true;
                break;
            case "a":
                movement.left = true;
                break;
            case "d":
                movement.right = true;
                break;
        }
    });
    document.addEventListener("keyup", e => {
        switch (e.key) {
            case "w":
                movement.back = false;
                break;
            case "s":
                movement.forward = false;
                break;
            case "a":
                movement.left = false;
                break;
            case "d":
                movement.right = false;
                break;
        }
    });

    function handleInput(dt) {
        handle.add_position(
            (movement.right ? 10 * dt : 0) + (movement.left ? -10 * dt : 0),
            (movement.up ? 10 * dt : 0) + (movement.down ? -10 * dt : 0),
            (movement.forward ? 10 * dt : 0) + (movement.back ? -10 * dt : 0),
        );

        for (let j in controllers) {
            var controller = controllers[j];
            const axes = controller.axes;
            handle.add_position(
                (axes[0] * axes[0] > 0.02 ? axes[0] * dt: 0),
                0.0,
                (axes[1] * axes[1] > 0.02 ? axes[1] * dt: 0),

                (Math.round(10 * (axes[1] + Number.EPSILON)) / 10) * dt,
            );

            handle.add_angle(
                (axes[3] * axes[3] > 0.02 ? -50 * axes[3] * dt: 0),
                (axes[2] * axes[2] > 0.02 ? -50 * axes[2] * dt: 0),
                0.0,
            );
        }
    }

    let pTime = 0;
    function render(time) {
        const dt = (time - pTime) * 0.001;
        pTime = time;
        handleInput(dt);

        let err = webGL.update(dt);
        if (err) console.error(err);
        webGL.render_gl();
        window.requestAnimationFrame(render);
    }
    window.requestAnimationFrame(render);
}

doInit();
