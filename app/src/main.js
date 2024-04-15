const { invoke } = window.__TAURI__.core;

let folderPathText;
let gamesListPath;

async function selectFile() {
  await invoke("select_file");
  window.__TAURI__.event.listen("folder-selected", (event) => {
    folderPathText.textContent = event.payload;
  });
}

async function addGames() {
  console.log("here1");

  // Invoke get_games_list and wait for the response
  try {
    const jsonData = await invoke("get_games_list");
    console.log(jsonData);
  } catch (error) {
    console.error(error);
  }
}

window.addEventListener("DOMContentLoaded", async () => {
  folderPathText = document.querySelector("#sync-folder-path");

  // Adding the games list
  addGames();

  document
    .getElementById("choose-button")
    .addEventListener("click", function () {
      selectFile();
    });
});
