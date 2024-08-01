# The cross platform game syncer!

Sync your games between MACOS, WINDOWS and LINUX easily!

![Screenshot 2024-07-28 at 14 11 54](https://github.com/user-attachments/assets/6ba255d3-2fb0-420b-8b22-c850f3417567)

There are games on steam that don't support cross platform save file synchronization.

For me, playing Lies of P on both my MacBook and Windows PC was irritating as I had to manually sync the game save.

So I made this software: __PLATFORM-SYNCER__

Platform-syncer is a full-stack application that allows you to sync your files by simply clicking a button.

## Important Info

The serverside component of this software platform must be self-hosted, whether that is from a server provider, or your home server. The server is intended to run on MacOS or Linux, but a windows release is also distributed.

The clientside runs on MacOS, Windows and Linux! It was built using the Tauri framework (https://tauri.app/) for the UI.

For both the server and the client, you can build it yourself with cargo, or use the provided executables.

## Instructions

### Client

* Download the client files
* Run the executable
* Add the URL of your server (it MUST include "http://")

![Screenshot 2024-07-28 at 14 21 47](https://github.com/user-attachments/assets/840453b2-dee9-464a-920a-d23ef4f9cbfd)

* Add a game
* Play!

When adding an extra client to your setup (ie. you have a single client set up to sync with the server and want to add another client, for syncing between two devices), rather than adding the games in the app you should edit the _games.json_ file for the application data, this can be found at:

- MacOS: Library/Application Support/syncer/games.json
- Linux: .config/syncer/games.json
- Windows: APPDATA\Roaming\ROAMING\syncer\games.json

You should make the new _games.json_ file identical to the existing one, with the path to game save files changed to match the path on the new system. This will also help to mediate any inconsistencies you could introduce by naming the same game in two different ways :).

### Server
* Download the server files
* In the server root directory, edit the "url.txt" file to contain the port you want to expose the server on (the default should be fine)
* Run the executable!
* Play!
