
const handlers = [
    console.log
];
function addSettingsChangeListener(cb) {
    handlers.push(cb);
}

function _genNamedDiv(name, ...classes) {
    const div = document.createElement("div");
    div.classList.add("field", ...classes);

    if(name) {
        const nameField = document.createElement("p");
        nameField.innerText = name;
        div.appendChild(nameField);
    }

    return div;
}

function genField(name, value, cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name, "text_field");

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

function genSlider(name, content, cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name, "slider");

    const valueField = document.createElement("input");
    valueField.classList.add("input");
    valueField.readOnly = readOnly;
    valueField.type = "range";
    valueField.min = content.min;
    valueField.max = content.max;
    valueField.step = content.inc;

    valueField.addEventListener("input", e => cb(parseFloat(e.target.value)));

    const changeText = (t) => valueField.value = t;
    changeText(content.value);

    div.appendChild(valueField);
    cb(parseFloat(valueField.value));

    return [div, changeText];
}

function genArray(name, content, parent_cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name, "array");

    const vecDiv =  document.createElement("div");
    const values = [];
    const setters = [];

    for (let index = 0; index < content.length; index++) {
        const element = content[index];

        const cb = v => {
            values[index] = v;   // This should be useless
            parent_cb(values);
        };

        const fieldElement = genSetting(element, v => values[index] = v, () => parent_cb(values));
        if(fieldElement) {
            div.appendChild(fieldElement[0]);
            setters.push(fieldElement[1]);
        }
    }

    const changeText = (t) => {
        for (let index = 0; index < t.length; index++) {
            setter[index](t[index]);
        }
    };

    div.appendChild(vecDiv);

    parent_cb(values);

    return [div, changeText];
}

function genCheck(name, value, cb = (e) => {}, readOnly=false) {
    const div = _genNamedDiv(name, "check");

    const valueField = document.createElement("input");
    valueField.classList.add("input");
    valueField.readOnly = readOnly;
    valueField.type = "checkbox";

    valueField.addEventListener("input", e => cb(e.target.checked));

    const changeChecked = (t) => valueField.checked = t;
    changeChecked(value);

    div.appendChild(valueField);
    cb(valueField.checked);

    return [div, changeChecked];
}

function genSetting(field, setter, flush) {
    const cb = v => {
        setter(v);
        flush();
    };

    switch(field.type) {
        case "array":
            return genArray(field.name, field.content, cb);
        case "text":
            return genField(field.name, field.content, cb);
        case "slider":
            return genSlider(field.name, field.content, cb);
        case "settings":
            return genSettings(field.name, field.content, cb);
        case "check":
            return genCheck(field.name, field.content, cb);
        case "data":
            setter(field.content);
            break;
        default:
            console.error("Wrong field type "+ field.type);
    }
}

function genSettings(name, settings, parent_cb) {
    console.log(settings);
    const wrapper = _genNamedDiv(name, "settings", settings.class);
    const div = document.createElement("div");
    div.classList.add("input");
    wrapper.appendChild(div);

    const values = {};

    for(let field of settings.fields) {
        const fieldElement = genSetting(field, v => values[field.id] = v, () => parent_cb(values));
        if(fieldElement)
            div.appendChild(fieldElement[0]);
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
        console.log(v);
        for(cb of handlers) {
            cb(v);
        }
    };

    const settingsDiv = document.getElementById("settings");
    settingsDiv.innerHTML = "";

    const [div, _] = genSettings("", settings, broadcast);

    stop_wrapper.inner = false;

    settingsDiv.appendChild(div);
}
