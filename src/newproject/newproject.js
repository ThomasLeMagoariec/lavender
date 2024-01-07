const { invoke } = window.__TAURI__.tauri;

const create = document.querySelector(".create");
const name = document.querySelector("#project-name");
const path = document.querySelector("#project-path");
const editor = document.querySelector("#project-editor");
const description = document.querySelector("#project-description");

create.addEventListener("click", () => {
    console.log("Create!")
    if (check_fields()) {
        console.log("true")
    } else {
        console.log("false")
    }
})

function check_fields() {

    if (name.value == "" || name.value === " ") { alert("name empty bitch"); }
    else if (path.value == "" || path.value === " ") { alert("path empty bitch"); }
    else if (editor.value == "" || editor.value === " ") { alert("editor empty bitch"); }

    if (invoke("check_path", { "path": path.value })) {
        alert("invalid  path my g")
        return false;
    }

    return true
}