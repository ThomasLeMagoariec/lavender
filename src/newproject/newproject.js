const { invoke } = window.__TAURI__.tauri;

const create = document.querySelector(".create");
const test = document.querySelector(".test");
const name = document.querySelector("#project-name");
const path = document.querySelector("#project-path");
const editor = document.querySelector("#project-editor");
const description = document.querySelector("#project-description");


create.addEventListener("click", async () => {
    console.log("Create!")
    if (check_fields()) {
        console.log("fields checked")
        const project = {
            "name": name.value,
            "path": path.value,
            "editor": editor.value,
            "description": description.value,
        }

        updateData(name.value, project)
    }
})

test.addEventListener("click", async () => {
    getState()
})

function check_fields() {

    if (name.value == "" || name.value === " ") { alert("name empty bitch"); }
    else if (path.value == "" || path.value === " ") { alert("path empty bitch"); }
    else if (editor.value == "" || editor.value === " ") { alert("editor empty bitch"); }

    if (!invoke("check_path", { "path": path.value })) {
        alert("invalid  path my g")
        return false; //! CHANGE BACK TO FALSE
    }

    return true
}

async function getState() {
    try {
        const state = await invoke('getState');
        console.log('State:', state);
    } catch (error) {
        console.error('Error getting state:', error);
    }
}
  
async function updateData(key, value) {
    
    try {
        await invoke('updateData', { key, value });
        console.log('Data updated successfully');
    } catch (error) {
        console.error('Error updating data:', error);
    }
}


getState()