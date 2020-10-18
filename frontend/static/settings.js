
const handlers = [];
function addSettingsChangeListener(cb) {
    handlers.push(cb);
}

function _genNamedDiv(name, ...classes) {
    const div = document.createElement("div");
    div.classList.add("field", ...classes);

    const nameField = document.createElement("p");
    nameField.innerText = name;
    div.appendChild(nameField);
    return div;
}

function genField(name, value, cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name);

    const valueField = document.createElement("input");
    valueField.classList.add("input");
    valueField.readOnly = readOnly;
    valueField.type = "text";

    valueField.addEventListener("input", e => cb(e.target.value));

    const changeText = (t) => valueField.value = t;
    changeText(value);

    div.appendChild(valueField);
    cb(valueField.value);

    return [div, changeText];
}

function genSlider(name, value, min, max, inc, cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name);

    const valueField = document.createElement("input");
    valueField.classList.add("input");
    valueField.readOnly = readOnly;
    valueField.type = "range";
    valueField.min = min;
    valueField.max = max;
    valueField.step = inc;

    valueField.addEventListener("input", e => cb(parseFloat(e.target.value)));

    const changeText = (t) => valueField.value = t;
    changeText(value);

    div.appendChild(valueField);
    cb(parseFloat(valueField.value));

    return [div, changeText];
}

function genVec3(name, value, min=0, max=1, inc=0.01, cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name);
    const currentValue = value.map(parseFloat);

    const vecDiv =  document.createElement("div");
    vecDiv.classList.add("vec3");
    vecDiv.classList.add("input");

    function genSmallField(div, index, className) {
        const wrapper = document.createElement("div");
        wrapper.classList.add(className);
        wrapper.classList.add("part");

        const field = document.createElement("input");
        field.readOnly = readOnly;
        field.type = "number";

        field.min = min;
        field.max=max;
        field.step = inc;

        field.addEventListener("input", e => {
            currentValue[index] = parseFloat(e.target.value);
            cb(currentValue);
        });

        wrapper.appendChild(field);
        div.appendChild(wrapper);

        return field;
    }

    const xField = genSmallField(vecDiv, 0, "x");
    const yField = genSmallField(vecDiv, 1, "y");
    const zField = genSmallField(vecDiv, 2, "z");

    const changeText = (t) => {
        xField.value = t[0];
        yField.value = t[1];
        zField.value = t[2];
    };
    changeText(value);

    div.appendChild(vecDiv);

    cb(value);
    return [div, changeText];
}

function genSettings(name, settings, parent_cb) {
    const wrapper = _genNamedDiv(name, "column");
    const div = document.createElement("div");
    div.classList.add("input");
    wrapper.appendChild(div);

    const values = {};

    for(let field of settings.fields) {
        values[field.id] = field.value;
        const cb = v => {
            values[field.id] = v;
            parent_cb(values);
        };

        let fieldElement;
        switch(field.type) {
            case "vector3":
                fieldElement = genVec3(field.name, field.value, field.min, field.max, field.inc, cb)[0];
                break;
            case "text":
                fieldElement = genField(field.name, field.value, cb)[0];
                break;
            case "slider":
                fieldElement = genSlider(field.name, field.value, field.min, field.max, field.inc, cb)[0];
                break;
            case "settings":
                fieldElement = genSettings(field.name, field.inner, cb)[0];
                break;
            default:
                console.error("Wrong field type "+ field.type);
                continue;
        }

        div.appendChild(fieldElement);
    }

    // Initiate chain
    parent_cb(values);

    return [wrapper, (_) => {}];
}

function set_settings(settings) {
    console.log(settings);

    // FIXME: This is used to prevent loops
    const stop_wrapper = {"inner": true};
    const broadcast = v => {
        if (stop_wrapper.inner) return;
        for(cb of handlers) {
            cb(v);
        }
    };

    const settingsDiv = document.getElementById("settings");
    settingsDiv.innerHTML = "";

    const div = genSettings("", settings, broadcast)[0];

    stop_wrapper.inner = false;

    settingsDiv.appendChild(div);
}
