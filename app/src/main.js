const { invoke } = window.__TAURI__.core;

let folderPathText;

async function selectFile() {
  await invoke("select_file");
  window.__TAURI__.event.listen("folder-selected", (event) => {
    folderPathText.textContent = event.payload;
  });
}

async function addGames() {
  console.log("here1");

  invoke("get_games_list")
    .then((message) => {
      console.log(message);

      // Converting the received object to a js JSON object
      const jsonObj = JSON.parse(message);

      const container = document.getElementById("game-container");
      for (let key in jsonObj) {
        if (jsonObj.hasOwnProperty(key)) {
          const game = document.createElement("div");
          game.classList.add("game-entry");
          game.textContent = `${key}: ${jsonObj[key]}`;
          container.appendChild(game);
        }
      }
    })
    .catch((error) => console.error(error));

  // // Invoke get_games_list and wait for the response
  // try {
  //   const jsonData = await invoke("get_games_list");
  //   console.log(jsonData);

  //   // Converting the jsonData to a proper js JSON object
  //   jsonObj = JSON.parse(jsonData);

  //   for (let key in obj) {
  //     if (obj.hasOwnProperty(key)) {
  //       const game = document.createElement("div");
  //       game.classList.add("game-entry");
  //       game.textContent = `${entry.key}`; // ${entry.value}
  //       container.appendChild(game);
  //     }
  //   }
  // } catch (error) {
  //   console.error(error);
  // }
}

window.addEventListener("DOMContentLoaded", async () => {
  // Setting up the divs to select
  folderPathText = document.querySelector("#sync-folder-path");

  // Adding the games list to the UI
  await addGames();

  document
    .getElementById("choose-button")
    .addEventListener("click", function () {
      selectFile();
    });
});
