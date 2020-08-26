import init, { WebGl, CameraHandle } from "./wasm.js"

const MOV_SPEED = 1000;
const SENSITIVITY_X = 50;
const SENSITIVITY_Y = 50;

window.addEventListener("gamepadconnected", connecthandler);
window.addEventListener("gamepaddisconnected", disconnecthandler);

function disconnecthandler(e) {
    delete controllers[e.gamepad.index];
}

function connecthandler(e) {
    controllers[e.gamepad.index] = e.gamepad;
}

function _genNamedDiv(name) {
    const div = document.createElement("div");
    div.classList.add("field");

    const nameField = document.createElement("p");
    nameField.innerText = name;
    div.appendChild(nameField);
    return div;
}

function genField(name, value, readOnly=false) {
    const div = _genNamedDiv(name);

    const valueField = document.createElement("input");
    valueField.classList.add("input");
    valueField.readOnly = readOnly;
    valueField.type = "text";

    const changeText = (t) => valueField.value = t;
    changeText(value);

    div.appendChild(valueField);
    return [div, changeText];
}

function genSlider(name, value, min, max, readOnly=false) {
    const div = _genNamedDiv(name);

    const valueField = document.createElement("input");
    valueField.classList.add("input");
    valueField.readOnly = readOnly;
    valueField.type = "range";
    valueField.min = min;
    valueField.max = max;

    const changeText = (t) => valueField.value = t;
    changeText(value);

    div.appendChild(valueField);
    return [div, changeText];
}

function genVec3(name, value, min=0, max=1, inc=0.01, readOnly=false) {
    const div = _genNamedDiv(name);
   
    const vecDiv =  document.createElement("div");
    vecDiv.classList.add("vec3");
    vecDiv.classList.add("input");

    function genSmallField(div, className) {
        const wrapper = document.createElement("div");
        wrapper.classList.add(className);
        wrapper.classList.add("part");

        const field = document.createElement("input");
        field.readOnly = readOnly;
        field.type = "number";

        field.min = min;
        field.max=max;
        field.step = inc;

        wrapper.appendChild(field);
        div.appendChild(wrapper);
        
        return field;
    }

    const xField = genSmallField(vecDiv, "x");
    const yField = genSmallField(vecDiv, "y");
    const zField = genSmallField(vecDiv, "z");

    const changeText = (t) => {
        xField.value = t[0];
        yField.value = t[1];
        zField.value = t[2];
    };
    changeText(value);

    div.appendChild(vecDiv);

    return [div, changeText];
}


const controllers = {};
function scangamepads() {
    gamepadS
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

    const settingsDiv = document.getElementById("settings");
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
            (movement.right ? MOV_SPEED * dt : 0) + (movement.left ? -MOV_SPEED * dt : 0),
            (movement.up ? MOV_SPEED * dt : 0) + (movement.down ? -MOV_SPEED * dt : 0),
            (movement.forward ? MOV_SPEED * dt : 0) + (movement.back ? -MOV_SPEED * dt : 0),
        );

        for (let j in controllers) {
            var controller = controllers[j];
            const axes = controller.axes;
            handle.add_position(
                (axes[0] * axes[0] > 0.02 ? axes[0] * MOV_SPEED * dt: 0),
                0.0,
                (axes[1] * axes[1] > 0.02 ? axes[1] * MOV_SPEED * dt: 0),
            );

            handle.add_angle(
                (axes[3] * axes[3] > 0.02 ? -SENSITIVITY_X * axes[3] * dt: 0),
                (axes[2] * axes[2] > 0.02 ? -SENSITIVITY_Y * axes[2] * dt: 0),
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

    settingsDiv.appendChild(genField("Planet Name: ", "Oppo")[0]);
    settingsDiv.appendChild(genSlider("Wooble: ", 3, 0, 10)[0]);
    settingsDiv.appendChild(genVec3("Vec: ", [0.2, 0.5, 1.0])[0]);

}

doInit();
