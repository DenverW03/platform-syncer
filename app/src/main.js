const { invoke } = window.__TAURI__.core;

let folderPathText;
let gamesListPath;

async function selectFile() {
  await invoke("select_file");
  window.__TAURI__.event.listen("folder-selected", (event) => {
    folderPathText.textContent = event.payload;
  });
}

// Logic flow

await invoke("get_games_list");
window.__TAURI__.event.listen("games-list", (event) => {
  const gamesListPath = event.payload;

  fetch(gamesListPath)
    .then((response) => response.json())
    .then((data) => {
      const container = document.getElementById("game-container");
      data.forEach((entry) => {
        const game = document.createElement("div");
        game.classList.add("game-entry");
        game.textContent = `${entry.key}`; // ${entry.value}
        container.appendChild(game);
      });
    })
    .catch((error) => console.error("Error fetching JSON:", error));
});

window.addEventListener("DOMContentLoaded", () => {
  folderPathText = document.querySelector("#sync-folder-path");
  document
    .getElementById("choose-button")
    .addEventListener("click", function () {
      selectFile();
    });
});
