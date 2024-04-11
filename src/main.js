const { invoke } = window.__TAURI__.core;

let folderPathText;

async function selectFile() {
  await invoke("select_file");
  window.__TAURI__.event.listen('file-selected', (event) => {
    folderPathText.textContent = event.payload;
  });
}

window.addEventListener("DOMContentLoaded", () => {
  folderPathText = document.querySelector("#sync-folder-path");
  document.getElementById("choose-button").addEventListener("click", function() {
    selectFile();
  });
});
