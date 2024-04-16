const { invoke } = window.__TAURI__.core;

let folderPathText;

async function selectFolder() {
  let gameName = "Dark Souls III";

  await invoke("select_folder", { gameName: gameName });
  window.__TAURI__.event.listen("folder-selected", (event) => {
    folderPathText.textContent = event.payload;
  });

  addSpecificGame(gameName);
}

// Function used to circumvent reloading entire GUI upon new folder sync addition
async function addSpecificGame(key) {
  // Getting the games list JSON and adding the new JSON object
  invoke("get_games_list")
    .then((message) => {
      // Converting the received object to a js JSON object
      const jsonObj = JSON.parse(message);

      const container = document.getElementById("game-container");
      if (jsonObj.hasOwnProperty(key)) {
        // Adding the game entry to the game container
        const game = document.createElement("div");
        game.classList.add("game-entry");
        game.textContent = `${key}: ${jsonObj[key]}`;
        container.appendChild(game);
      }
    })
    .catch((error) => console.error(error));
}

async function addGames() {
  // Getting the synced games list and adding to the GUI
  invoke("get_games_list")
    .then((message) => {
      // Converting the received object to a js JSON object
      const jsonObj = JSON.parse(message);

      const container = document.getElementById("game-container");
      for (let key in jsonObj) {
        if (jsonObj.hasOwnProperty(key)) {
          // Adding the game entry to the game container
          const game = document.createElement("div");
          game.classList.add("game-entry");
          game.textContent = `${key}: ${jsonObj[key]}`;
          container.appendChild(game);
        }
      }
    })
    .catch((error) => console.error(error));
}

window.addEventListener("DOMContentLoaded", async () => {
  // Setting up the divs to select
  folderPathText = document.querySelector("#sync-folder-path");

  // Adding the games list to the UI
  await addGames();

  document
    .getElementById("choose-button")
    .addEventListener("click", function () {
      selectFolder();
    });
});
