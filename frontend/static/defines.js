
const div = document.getElementById("info");
function set_info(x, y, z, angl_x, angl_y, angl_z) {
    div.innerHTML = `<p>Position: ${x.toFixed(2)}, ${y.toFixed(2)}, ${z.toFixed(2)}</p>
        <p>Rotation: ${angl_x.toFixed(2)}, ${angl_y.toFixed(2)}, ${angl_z.toFixed(2)}</p>`;
}
