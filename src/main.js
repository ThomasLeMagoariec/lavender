const { invoke } = window.__TAURI__.tauri;

const newproject = document.querySelector(".new-project")

newproject.addEventListener("click", () => {
    invoke("open_new_project_window");
})