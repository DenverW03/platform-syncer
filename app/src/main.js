const { invoke } = window.__TAURI__.core;

let folderPathText;

async function selectFolder() {
  // Getting the name the user wants
  let inputField = document.getElementById("name-input");
  let gameName = inputField.value;

  // If a game name is not inputted, don't allow selection
  if (gameName == "") return;

  // Rust backend handles folder selection, syncing and synced game list updating
  await invoke("select_folder", { gameName: gameName });
  window.__TAURI__.event.listen("folder-selected", (event) => {
    addSpecificGame(gameName, event.payload);
  });
}

async function newServerAddress(serverAddress) {
  await invoke("new_server_address", { serverAddress: serverAddress });
}

// This function syncs the game with the cloud: TODO!
async function sync_game(game_name, path) {
  // Call the rust backend to check the last modified date of the file
  // If the last modified date of the file is newer than the cloud file, upload the file!
  // Otherwise, request the file from the server and save it
  await invoke("sync_game", { gameName: game_name, path: path });
}

// Function used to circumvent reloading entire GUI upon new folder sync addition
function addSpecificGame(gameName, dir) {
  const container = document.getElementById("game-container");
  const game = document.createElement("div");

  // Adding the game entry to the game container
  const gameEntry = document.createElement("div");
  gameEntry.classList.add("game-entry");

  // Adding a text div to the game entry
  const gameText = document.createElement("div");
  gameText.classList.add("game-text");
  gameText.textContent = `${gameName}: ${dir}`;
  gameEntry.appendChild(gameText);

  // Creating a sync button to add to the div
  const syncButton = document.createElement("button");
  syncButton.classList.add("sync-button");
  syncButton.textContent = "SYNC";

  // Adding the button functionality to the sync button
  syncButton.addEventListener("click", function () {
    // Calling the syncing function with the file path
    sync_game(`${gameName}`, `${dir}`);
  });

  gameEntry.appendChild(syncButton);
  container.appendChild(gameEntry);
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
          const gameEntry = document.createElement("div");
          gameEntry.classList.add("game-entry");

          // Adding a text div to the game entry
          const gameText = document.createElement("div");
          gameText.classList.add("game-text");
          gameText.textContent = `${key}: ${jsonObj[key]}`;
          gameEntry.appendChild(gameText);

          // Creating a sync button to add to the div
          const syncButton = document.createElement("button");
          syncButton.classList.add("sync-button");
          syncButton.textContent = "SYNC";

          // Adding the button functionality to the sync button
          syncButton.addEventListener("click", function () {
            // Calling the syncing function with the file path
            sync_game(`${key}`, `${jsonObj[key]}`);
          });

          gameEntry.appendChild(syncButton);
          container.appendChild(gameEntry);
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

  const serverInput = document.getElementById("server-input");

  // Setting the visible URL if there is one, if not there is placeholder text - "server address"
  invoke("get_current_server_address").then((serverURL) => {
    if (serverURL != "") {
      serverInput.placeholder = serverURL;
    }
  });

  serverInput.addEventListener("keyup", (event) => {
    // Check if the key pressed was Enter
    if (event.key === "Enter") {
      const serverAddress = serverInput.value.trim();
      if (serverAddress) {
        newServerAddress(serverAddress);
      }
    }
  });
});
